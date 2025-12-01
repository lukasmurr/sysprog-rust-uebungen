// client/src/main.rs

// TODO (David): Abhängigkeiten in client/Cargo.toml eintragen:
// - tokio (mit io + net + macros)
// - clap (für CLI)
// - anyhow oder eigene Error-Typen
// - shared (path = "../shared")

// TODO (David): use-Statements vorbereiten:
// - clap::Parser für CLI-Args
// - tokio::net::TcpStream
// - tokio::io::{stdin, AsyncBufReadExt, AsyncWriteExt, BufReader}
// - tokio::spawn, tokio::select
// - shared::{ChatMessage, new_message, serialize_message, deserialize_message}
// - std::net::SocketAddr

// TODO (David): CLI-Struct definieren (Args):
// - server: String (default "127.0.0.1:8080")
// - username: String (required, z.B. -u / --username)
//
// #[derive(Parser, Debug)]
// struct Args { ... }

// TODO (David): main-Funktion als #[tokio::main] async fn main() -> Result<...>
// Schritte in main:
//
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

// TODO (David): read_task implementieren:
// - Parameter: reader (BufReader<...>)
// - Endlosschleife:
//   - per read_line Zeilen vom Server lesen
//   - Bei bytes == 0 → Server hat Verbindung geschlossen → Info ausgeben & break
//   - Versuchen, die Zeile mit deserialize_message in ChatMessage zu parsen
//   - Falls erfolgreich: schön formatiert ausgeben (z.B. [HH:MM:SS] <user> msg)
//   - Falls nicht: als Rohtext anzeigen (Fallback)

// TODO (David): write_task implementieren:
// - Parameter: stdin-Reader (BufReader<Stdin>), writer (Half des TcpStream), username
// - Endlosschleife:
//   - Zeile von stdin lesen
//   - Trimmen; leere Zeilen ignorieren
//   - "/quit" als Kommando behandeln → Info ausgeben, break
//   - Aus dem Text eine ChatMessage mit new_message(username, text) bauen
//   - serialize_message aufrufen, Result behandeln
//   - JSON-String zum Server schreiben (write_all)
//   - Fehler beim Senden loggen und Schleife beenden

// TODO (David): Funktion print_message(msg: &ChatMessage):
// - Zeit formatieren (z.B. "%H:%M:%S")
// - println! im Format: "[HH:MM:SS] <username> content"

// TODO (David): UX-Verbesserungen (optional):
// - Farben mit colored oder ähnlichem Crate (nicht Pflicht)
// - Anzeige von Systemnachrichten (SERVER) in anderer Farbe
// - Hinweis beim Start: "Type /quit to exit."
//
// TODO (David): manuelle Tests planen:
// - Einen Server starten (Lukas-Version)
// - Einen Client starten: cargo run -- -u Lukas
// - Zweiten Client starten: cargo run -- -u David
// - Prüfen, ob Nachrichten bei allen ankommen
// - /quit testen
