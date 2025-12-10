use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, Mutex};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::net::SocketAddr;
use std::collections::HashMap;
use std::sync::Arc;
use std::env;
use anyhow::Result;
use shared::{ChatMessage, new_message, serialize_message};

type Clients = Arc<Mutex<HashMap<SocketAddr, String>>>;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    let listener = TcpListener::bind(&addr).await?;
    println!("Server running on {}", addr);

    let (tx, _rx) = broadcast::channel::<ChatMessage>(100);
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, addr) = listener.accept().await?;
        let tx = tx.clone();
        let rx = tx.subscribe();
        let clients = clients.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, addr, clients, tx, rx).await {
                eprintln!("Error handling client {}: {}", addr, e);
            }
        });
    }
}

async fn handle_client(
    mut socket: TcpStream,
    addr: SocketAddr,
    clients: Clients,
    tx: broadcast::Sender<ChatMessage>,
    mut rx: broadcast::Receiver<ChatMessage>,
) -> Result<()> {
    let (reader, mut writer) = socket.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    writer.write_all(b"Enter your username:\n").await?;
    
    if reader.read_line(&mut line).await? == 0 {
        return Ok(());
    }
    let username = line.trim().to_string();
    if username.is_empty() {
        writer.write_all(b"Username cannot be empty.\n").await?;
        return Ok(());
    }

    {
        let mut clients_guard = clients.lock().await;
        clients_guard.insert(addr, username.clone());
        println!("Client joined: {} ({})", username, addr);
        println!("Connected clients: {}", clients_guard.len());
    }

    let join_msg = new_message("SERVER", &format!("{} has joined the chat", username));
    let _ = tx.send(join_msg);

    loop {
        tokio::select! {
            result = reader.read_line(&mut line) => {
                if result? == 0 {
                    break;
                }
                let content = line.trim();
                if content == "/quit" {
                    break;
                }
                if !content.is_empty() {
                    let msg = new_message(&username, content);
                    let _ = tx.send(msg);
                }
                line.clear();
            }
            result = rx.recv() => {
                let msg = result?;
                let serialized = serialize_message(&msg)?;
                writer.write_all(serialized.as_bytes()).await?;
            }
        }
    }

    {
        let mut clients_guard = clients.lock().await;
        clients_guard.remove(&addr);
        println!("Client left: {} ({})", username, addr);
        println!("Connected clients: {}", clients_guard.len());
    }

    let leave_msg = new_message("SERVER", &format!("{} has left the chat", username));
    let _ = tx.send(leave_msg);

    Ok(())
}
