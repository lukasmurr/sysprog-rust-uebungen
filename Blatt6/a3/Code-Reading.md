### **1. Welche Abhängigkeiten zu externen Crates gibt es?**

* `anyhow` (Fehlerbehandlung)
* `structopt` (CLI-Argumente)
* `serde` + `serde_json` (Serialisieren/Deserialisieren)
* `chrono` (Zeitstempel)
* `home` (Home-Verzeichnis finden)

### **2. In welche Module ist es aufgeteilt?**

* `main.rs`
* `cli.rs` (CLI-Parsing)
* `tasks.rs` (Task-Struktur + JSON-Handling)

### **3. Wie werden die Kommandozeilen-Argumente eingelesen? Worauf baut das verwendete Crate auf?**

* Über `StructOpt::from_args()`
* `structopt` baut auf **clap** auf.

### **4. Wozu gibt es die `enum Action`?**

* Repräsentiert die verfügbaren CLI-Befehle:

  * `Add`
  * `List`
  * `Done`

### **5. Was passiert, wenn die Journal Datei nicht angegeben wird?**

* Es wird versucht, die Default-Datei `~/.rust-journal.json` zu finden.
* Wenn das fehlschlägt ⇒ Fehler (`anyhow!("Failed to find journal file.")`).

### **6. Was sind die Tasks?**

* Strukturen mit:

  * `text: String`
  * `created_at: DateTime<Utc>`

### **7. Wie wird eine Task angezeigt?**

* Über `impl Display for Task`
* Format:
  `"<text> [YYYY-MM-DD HH:MM]"`

### **8. Wie wird die JSON Datei erzeugt/modifiziert/gelesen?**

* Mit `serde_json`:

  * `serde_json::from_reader`
  * `serde_json::to_writer_pretty`
* Datei geöffnet mit `OpenOptions`

### **9. Gibt es `unwrap()`, `expect()` oder `panic()` Anweisungen?**

* Nein, keines davon wird verwendet.

### **10. Wie erfolgt die Fehlerbehandlung in den verschiedenen Funktionen?**

* Rückgabe von `Result<>`
* Nutzung von `?` Operator
* In `main`: `anyhow::Error` für lesbare Fehlermeldungen

### **11. Wie wird die aktuelle Uhrzeit bestimmt?**

* `chrono::Utc::now()`

### **12. Mit welchem Exit-Code wird das Programm bei Ok und wie bei einem Fehler beendet?**

* Bei `Ok(())` Exit-Code **0**
* Bei Fehlern automatisch Exit-Code **1** (durch `anyhow` + fehlgeschlagenes `main()`)
