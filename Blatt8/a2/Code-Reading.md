# Code Reading - catsay

1. **Was sagt `cargo clippy`?**
   `cargo clippy` läuft ohne Warnungen oder Fehler durch ("Finished `dev` profile ...").

2. **Welche Kommandozeilen-Argumente hat es?**
   - `-i` / `--stdin`: Liest die Nachricht von STDIN.
   - `message`: Die Nachricht, die die Katze sagt (Standard: "Meow!").
   - `-f` / `--file`: Lädt ein Katzenbild aus einer Datei.
   - `-d` / `--dead`: Lässt die Katze tot aussehen (Augen "x" statt "o").

3. **Wie werden die Kommandozeilen-Argumente eingelesen?**
   Mithilfe des `structopt` Crates. Es wird ein Struct `Options` definiert, das `#[derive(StructOpt)]` verwendet. Die Argumente werden dann mit `Options::from_args()` geparst.

4. **Welche Abhängigkeiten zu externen Crates gibt es?**
   - `structopt`
   - `colored`
   - `failure`
   - `exitfailure`
   - `assert_cmd` (dev-dependency, implizit durch Nutzung in tests)
   - `predicates` (dev-dependency, implizit durch Nutzung in tests)

5. **In welche Module ist es aufgeteilt?**
   Der Code liegt hauptsächlich in `src/main.rs`. Es gibt keine explizite Aufteilung in weitere Module (keine `mod` Deklarationen in `main.rs`). Die Datei `src/bench.rs` existiert, ist aber im aktuellen Zustand nicht als Modul eingebunden.

6. **Wozu wird das `predicates` Crate verwendet?**
   Es wird in den Integrationstests (`tests/integration_test.rs`) verwendet, um Assertions auf den Output des Programms zu machen (z.B. `predicate::str::contains("Meow!")`).

7. **Wie kann man die Ausgabe von `stdout` und `stderr` in Dateien umlenken?**
   Über die Shell-Redirection:
   `cargo run -- "message" > stdout.txt 2> stderr.txt`
   Im Code wird `stdout` über `io::stdout()` und `stderr` über `eprintln!` angesprochen.

8. **Bleibt die Farbe beim Umlenken in Dateien immer erhalten?**
   Nein, das `colored` Crate erkennt normalerweise, ob die Ausgabe in ein Terminal (TTY) erfolgt. Wenn in eine Datei umgelenkt wird, werden die ANSI-Farbcodes meist deaktiviert, um die Datei nicht mit Steuerzeichen zu verschmutzen.

9. **Wie wird die Ausgabe in Farbe erzeugt?**
   Durch Methoden des `colored` Crates, die auf Strings aufgerufen werden, z.B. `.bright_yellow().underline().on_purple()`.

10. **Gibt es `unwrap()`, `except()` oder `panic()` Anweisungen?**
    Im eigentlichen Programmcode (`src/main.rs`) werden keine expliziten `unwrap()`, `expect()` oder `panic!()` verwendet. Stattdessen wird der `?`-Operator zur Fehlerpropagation genutzt. In den Tests (`tests/integration_test.rs`) wird `expect()` verwendet.

11. **Wie erfolgt die Fehlerbehandlung in den verschiedenen Funktionen?**
    Die `main`-Funktion gibt ein `Result<(), ExitFailure>` zurück. Fehler (z.B. beim Lesen von Dateien oder Schreiben auf stdout) werden mit dem `?`-Operator propagiert. `failure::ResultExt` (`with_context`) wird genutzt, um Fehlern Kontext hinzuzufügen. `ExitFailure` sorgt für eine saubere Fehlerausgabe und den Exit-Code.

12. **Mit welchem Exit-Code wird das Programm bei Ok und wie bei einem Fehler beendet?**
    - **Ok:** Exit-Code 0 (durch `Ok(())`).
    - **Fehler:** Ein Non-Zero Exit-Code (meist 1), gesteuert durch `ExitFailure`.

13. **Gibt es Unit-Tests? Wenn ja, wie lässt man sie laufen?**
    Es gibt keine Unit-Tests in `src/main.rs`. `src/bench.rs` enthält Code, der wie ein Benchmark-Test aussieht, ist aber nicht eingebunden.

14. **Gibt es Integrationstests? Wenn ja, wie lässt man sie laufen?**
    Ja, in `tests/integration_test.rs`. Man lässt sie mit `cargo test` laufen.

15. **Wozu gibt es `src/bench.rs`? Wie wendet man es an?**
    Es ist für Benchmarks gedacht (nutzt `#![feature(test)]`). Um es anzuwenden, müsste man Rust Nightly verwenden und es als Modul oder Benchmark-Target einbinden (z.B. `cargo bench`), was in der aktuellen Konfiguration aber nicht vorbereitet ist.

16. **Wie kann man das Programm installieren?**
    Mit `cargo install --path .` im Projektverzeichnis.

17. **Wie kann man den Installationspfad verändern?**
    Durch die Option `--root <PFAD>` beim `cargo install` Befehl oder durch Setzen der Umgebungsvariable `CARGO_INSTALL_ROOT`.

18. **Was müssten Sie tun, damit ein anderer das Programm einfach mit `cargo install <programm>` installieren kann?**
    Das Paket (Crate) müsste auf [crates.io](https://crates.io) veröffentlicht werden (`cargo publish`).
