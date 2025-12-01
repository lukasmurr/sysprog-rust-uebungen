// server/src/main.rs

// TODO (Lukas): Abhängigkeiten in server/Cargo.toml eintragen:
// - tokio (mit "full" oder mind. net + io + macros)
// - anyhow oder eigene Error-Typen
// - shared (path = "../shared")

// TODO (Lukas): use-Statements vorbereiten:
// - Tokio: TcpListener, TcpStream, spawn, sync::broadcast, io (AsyncRead/Write, BufReader)
// - std::net::SocketAddr
// - std::collections::HashMap
// - std::sync::Arc
// - shared::{ChatMessage, new_message, serialize_message, deserialize_message}

// TODO (Lukas): Typalias für Clients:
// type Clients = Arc<Mutex<HashMap<SocketAddr, String>>>;

// TODO (Lukas): main-Funktion als async definieren mit #[tokio::main]:
// - Listener auf "0.0.0.0:8080" binden
// - Broadcast-Channel (Sender/Receiver) für ChatMessage anlegen
// - Gemeinsame Clients-Map (Arc<Mutex<...>>)
// - Endlosschleife: incoming connections akzeptieren
// - Für jede Verbindung einen Task mit tokio::spawn starten
//   - handle_client(stream, addr, clients.clone(), tx.clone(), tx.subscribe())

// TODO (Lukas): Port konfigurierbar machen (später):
// - über Umgebungsvariable oder CLI-Argument (simple Variante: env::var)

// TODO (Lukas): Funktion handle_client(...) definieren:
// Parameter:
// - TcpStream
// - SocketAddr
// - Clients (Arc<Mutex<...>>)
// - broadcast::Sender<ChatMessage>
// - broadcast::Receiver<ChatMessage>
//
// Schritte in handle_client:
// 1. Stream in reader/writer aufteilen (split)
// 2. Reader mit BufReader wrappen für read_line
// 3. Username vom Client lesen:
//    - Dem Client vorher Hinweis schicken: "Enter your username:"
//    - Erste Zeile als username.trim() übernehmen
//    - Wenn leer: Fehlermeldung senden und Verbindung schließen
// 4. Username in Clients-Map eintragen (addr -> username)
// 5. Join-Nachricht erstellen (SERVER-User) und an alle broadcasten
// 6. Zwei asynchrone Teilaufgaben (tokio::spawn) vorbereiten:
//
//    a) Lese-Task: Nachrichten dieses Clients lesen
//       - Schleife: read_line
//       - "/quit" als spezielles Kommando zum Verlassen behandeln
//       - normale Zeilen als ChatMessage (new_message(username, inhalt)) bauen
//       - Message per tx.send(...) an alle broadcasten
//       - Fehler beim Lesen oder send() sinnvoll loggen und Schleife beenden
//
//    b) Schreib-Task: Broadcast-Nachrichten an diesen Client senden
//       - Schleife: rx.recv() auf Broadcast-Receiver
//       - Jede ChatMessage mit serialize_message(...) in JSON verwandeln
//       - Zum Client-Writer schreiben
//       - Fehler beim Schreiben behandeln (z.B. Verbindung weg → Schleife beenden)
//
// 7. Mit tokio::select! oder ähnlichem auf erstes Task-Ende warten
// 8. Cleanup bei Disconnect:
//    - Client aus Clients-Map entfernen
//    - Leave-Nachricht (SERVER) an alle broadcasten
//    - Logging "addr disconnected (username)"

// TODO (Lukas): Logging verbessern (optional):
// - Anzahl verbundener Clients nach Join/Leave ausgeben
// - Fehler mit eprintln! markieren

// TODO (Lukas): einfache manuelle Tests planen:
// - Server starten
// - Mit mehreren Clients verbinden (später Davids client oder telnet/netcat)
// - Nachrichten broadcastet sehen
// - Prüfen, ob Join/Leave-Nachrichten korrekt funktionieren
