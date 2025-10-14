# Zusammenfassung von Getting Started mit Rust

Link: https://www.rust-lang.org/learn/get-started

### 1. Rust Installation mit Rustup

**Rustup** ist das primäre Tool zur Installation und Versionsverwaltung von Rust:

- **Unix/Linux/macOS/WSL**: 
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Windows**: Herunterladen und Ausführen von `rustup-init.exe`
- **Update**: `rustup update` um Rust aktuell zu halten

### 2. Cargo - Build Tool und Package Manager

Cargo wird automatisch mit Rustup installiert und bietet folgende Funktionen:

- `cargo build` - Projekt kompilieren
- `cargo run` - Projekt ausführen
- `cargo test` - Tests ausführen  
- `cargo doc` - Dokumentation erstellen
- `cargo publish` - Bibliothek auf crates.io veröffentlichen
- `cargo new <projektname>` - Neues Projekt erstellen
- `cargo add <crate>` - Abhängigkeiten hinzufügen

### 3. Projektstruktur

Ein neues Rust-Projekt hat folgende Struktur:
```
hello-rust/
├── Cargo.toml    # Manifest-Datei mit Metadaten und Abhängigkeiten
└── src/
    └── main.rs   # Hauptanwendungscode
```

### 4. Abhängigkeiten verwalten

- Packages (Crates) werden auf **crates.io** bereitgestellt
- Abhängigkeiten werden in `Cargo.toml` unter `[dependencies]` definiert
- `Cargo.lock` protokolliert exakte Versionen der verwendeten Abhängigkeiten

### 5. Das ferris-says Beispiel

Demonstriert die Verwendung einer externen Bibliothek:

1. Abhängigkeit zu `Cargo.toml` hinzufügen: `ferris-says = "0.3.1"`
2. Import in `main.rs`: `use ferris_says::say;`
3. Verwendung der Funktion mit entsprechenden Parametern

### 6. Rust-Community

- **Rustaceans**: So nennen sich Rust-Programmierer (Wortspiel auf "Crustaceans")
- **Ferris**: Inoffizielles Maskottchen der Rust-Community (Name leitet sich von "ferrous" = eisenhaltig ab)
- Weitere Lernressourcen auf der Learn-Seite verfügbar
