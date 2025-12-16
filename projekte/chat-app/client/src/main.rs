// client/src/main.rs

// Abhängigkeiten in client/Cargo.toml:
// - tokio (mit io + net + macros)
// - clap (für CLI)
// - chrono (für Zeitformatierung)
// - shared (path = "../shared")

// use-Statements:
// - clap::Parser für CLI-Args
// - tokio::net::TcpStream
// - tokio::io::{stdin, AsyncBufReadExt, AsyncWriteExt, BufReader}
// - tokio::spawn, tokio::select
// - shared::{ChatMessage, deserialize_message}
// - std::net::SocketAddr

use clap::Parser;
use shared::{deserialize_message, ChatMessage};
use std::net::SocketAddr;
use tokio::io::{stdin, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;

// CLI-Struct (Args):
// - server: String (default "127.0.0.1:8080")
// - username: String (required, z.B. -u / --username)
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "127.0.0.1:8080")]
    server: String,

    #[arg(short, long)]
    username: String,
}

// main-Funktion als #[tokio::main] async fn main() -> Result<...>
// Schritte in main:
// 1. CLI-Argumente parsen (Args::parse())
// 2. Server-Adresse in SocketAddr parsen
// 3. TCP-Verbindung zum Server aufbauen (TcpStream::connect)
// 4. Stream in reader/writer aufsplitten, reader in BufReader wrappen
// 5. Username einmalig zum Server senden (z.B. "username\n")
// 6. Zwei parallele Tasks:
//    a) Task zum Lesen vom Server (read_task)
//    b) Task zum Lesen von stdin und Senden an Server (write_task)
// 7. Mit tokio::select! auf das erste Ende der beiden Tasks warten
// 8. Bei Beendigung saubere Ausgabe: "Disconnected from server."
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let server_addr: SocketAddr = args.server.parse()?;

    println!("Connecting to server at {}...", server_addr);

    let stream = TcpStream::connect(server_addr).await?;
    println!("Connected to server!");
    println!("Type /quit to exit.\n");

    let (read_half, mut write_half) = stream.into_split();
    let reader = BufReader::new(read_half);

    let username_msg = format!("{}\n", args.username);
    write_half.write_all(username_msg.as_bytes()).await?;

    let username = args.username.clone();

    let read_handle = tokio::spawn(read_task(reader));
    let write_handle = tokio::spawn(write_task(write_half, username));

    tokio::select! {
        result = read_handle => {
            if let Err(e) = result {
                eprintln!("Read task error: {}", e);
            }
        }
        result = write_handle => {
            if let Err(e) = result {
                eprintln!("Write task error: {}", e);
            }
        }
    }

    println!("\nDisconnected from server.");

    Ok(())
}

// read_task:
// - Parameter: reader (BufReader<...>)
// - Endlosschleife:
//   - per read_line Zeilen vom Server lesen
//   - Bei bytes == 0 → Server hat Verbindung geschlossen → Info ausgeben & break
//   - Versuchen, die Zeile mit deserialize_message in ChatMessage zu parsen
//   - Falls erfolgreich: schön formatiert ausgeben (z.B. [HH:MM:SS] <user> msg)
//   - Falls nicht: als Rohtext anzeigen (Fallback)
async fn read_task(mut reader: BufReader<OwnedReadHalf>) {
    let mut line = String::new();

    loop {
        line.clear();

        match reader.read_line(&mut line).await {
            Ok(0) => {
                println!("\nServer closed the connection.");
                break;
            }
            Ok(_) => {
                match deserialize_message(line.trim()) {
                    Ok(msg) => {
                        print_message(&msg);
                    }
                    Err(_) => {
                        let trimmed = line.trim();
                        if !trimmed.is_empty() {
                            println!("{}", trimmed);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading from server: {}", e);
                break;
            }
        }
    }
}

// write_task:
// - Parameter: stdin-Reader (BufReader<Stdin>), writer (Half des TcpStream), username
// - Endlosschleife:
//   - Zeile von stdin lesen
//   - Trimmen; leere Zeilen ignorieren
//   - "/quit" als Kommando behandeln → Info ausgeben, break
//   - Nachricht als Klartext zum Server senden (Server erstellt ChatMessage)
//   - Fehler beim Senden loggen und Schleife beenden
async fn write_task(mut writer: OwnedWriteHalf, _username: String) {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin);
    let mut line = String::new();

    loop {
        line.clear();

        match reader.read_line(&mut line).await {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                let trimmed = line.trim();

                if trimmed.is_empty() {
                    continue;
                }

                if trimmed == "/quit" {
                    println!("Leaving chat...");
                    let _ = writer.write_all(b"/quit\n").await;
                    break;
                }

                let msg_line = format!("{}\n", trimmed);
                if let Err(e) = writer.write_all(msg_line.as_bytes()).await {
                    eprintln!("Error sending message: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error reading from stdin: {}", e);
                break;
            }
        }
    }
}

// print_message(msg: &ChatMessage):
// - Zeit formatieren (z.B. "%H:%M:%S")
// - println! im Format: "[HH:MM:SS] <username> content"
// - Systemnachrichten (SERVER) werden mit * markiert
fn print_message(msg: &ChatMessage) {
    let local_time = msg.timestamp.with_timezone(&chrono::Local);
    let time_str = local_time.format("%H:%M:%S");

    if msg.username == "SERVER" {
        println!("[{}] * {} *", time_str, msg.content);
    } else {
        println!("[{}] <{}> {}", time_str, msg.username, msg.content);
    }
}
