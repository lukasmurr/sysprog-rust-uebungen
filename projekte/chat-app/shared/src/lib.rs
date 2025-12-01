// shared/src/lib.rs

// TODO (Lukas): Dependencies in shared/Cargo.toml eintragen:
// - serde (mit derive)
// - serde_json
// - chrono (für Timestamp, optional)

// TODO (Lukas): ChatMessage-Struktur definieren:
// - Felder: username: String, content: String, timestamp (z.B. chrono::DateTime<Utc>)
// - #[derive(Debug, Clone, Serialize, Deserialize)]

// TODO (Lukas): Hilfsfunktion new_message(username: &str, content: &str) -> ChatMessage
// - timestamp mit "jetzt" setzen

// TODO (Lukas): serialize_message(msg: &ChatMessage) -> Result<String, Error>
// - Message via serde_json in String serialisieren
// - Am Ende ein '\n' anhängen, damit server/client zeilenweise lesen können

// TODO (Lukas): deserialize_message(line: &str) -> Result<ChatMessage, Error>
// - JSON-String zurück in ChatMessage parsen
// - Sinnvolle Fehler zurückgeben (nicht paniken)

// TODO (Lukas): Unit-Tests für Roundtrip:
// - msg -> serialize -> deserialize -> msg'
// - Prüfen, dass alle Felder gleich bleiben
