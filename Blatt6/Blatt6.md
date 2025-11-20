# Übungen zur Systemnahen Programmierung in Rust 

**Blatt 6** 

Wintersemester 2025/2026 | <Hubert.Hoegl@tha.de>

<https://tha.de/~hhoegl/home/SysProgRust>

2025-11-13


**Hinweise**

* Lesen Sie in **The Book** (https://doc.rust-lang.org/stable/book/) das 12. 
  Kapitel "An I/O Project: Building a Command Line Program".

  Anregung: Noch etwas intensiver wird das Thema im Buch "Command line
  apps in Rust" behandelt:

  <https://rust-cli.github.io/book/index.html>

  Wer noch mehr Ideen und Tipps für CLI Tools benötigt der kann in das folgende
  Buch schauen: *Ken Youens-Clark, Command-Line Rust. A Project-Based Primer
  for Writing Rust-CLIs*. Es liegt auf unserer Nextcloud, der Link steht auf der
  Moodle Startseite von diesem Kurs.
   

* In *Rustlings* die Abschnitte `12_options/*`,  `13_error_handling/*`,
  `14_generics/*` und `15_traits/*` bearbeiten.  Gerne auch `quiz1.rs` und
  `quiz2.rs` ansehen.


* Denken Sie an die "Mini Projekte" (drei Stück für Oktober, November,
  Dezember).  Diese bewerte ich als Bonus-Punkte
  mit bis zu 25% bei der Klausur. Sie können diese entweder allein oder
  mit anderen in der Gruppe machen.  Die Abgabe erfolgt in dem Repo das
  Sie jetzt schon haben, und auch nur bei einem/r Teilnehmer/in einer
  Gruppe. Die Namen und Matrikelnummern aller Gruppenmitglieder sollen
  dabei stehen. Mögliche Anregungen sind Programme aus früheren Vorlesungen,
  z.B. Programmieren 1/2/3, Betriebssysteme, eigene Ideen, etc.  
 

## Aufgabe 1

* Nehmen Sie den Quelltext für das minigrep Beispiel aus Kapitel 12 in den
  Ordner `a1/` auf. Vergessen Sie nicht die Tests.

  Das Programm besteht aus einer `lib.rs` und einer `main.rs`. Insgesamt sind
  es nur 124 Zeilen Code.

<!--
Noch eine Idee fand ich im neuen Linux Magazin 12/2022: Es gibt dort 
monatlich ein interessantes kleines Projekt das in Go geschrieben ist, aktuell
ist es ein Passwort-Manager mit der Age Bibliothek (<https://age-encryption.org>). 
Es gibt auch eine Variante in Rust, siehe <https://lib.rs/crates/age>, damit 
könnte man das kleine Programm auch in Rust schreiben. Der Artikel ist hier:

<https://tha.de/~hhoegl/home/sysprog/schilli-12-22-age.pdf>
-->



## Aufgabe 2 

Überarbeiten Sie den Code aus Aufgabe 1 mit Hilfe der vielen Tipps, die
in dem *Command line apps with Rust* Buch stehen [1]. Legen Sie den neuen
Code in den Ordner `a2/`.

Erweiterungen die sie einbauen sollen:

* Argumente mit clap derive

* `--verbose` CLI Argument

* Fehlerbehandlung mit `anyhow`

* Logging (Log-Level einstellbar über RUST_LOG und CLI Argument)

* Konfiguration über Environment Variablen (die Kommandozeile übersteuert
  diese jedoch)

* Optionalen JSON Output

[1] <https://rust-cli.github.io/book/index.html>


## Aufgabe 3

Es geht hier um "Code Reading".

Bei dieser Aufgabe ist kein Rust Quellcode zu erstellen, sondern ein
bestehendes Programm zu analysieren. Das Programm finden Sie unter

<https://gitlab.com/hhoegl-tha/snp-rs/rusty-journal>

Sie müssen das Repo *nicht* in ihre Abgabe übernehmen. Versuchen Sie es
vollständig zu verstehen. Schreiben Sie eine Zusammenfassung in Stichpunkten in
die Datei `a3/Code-Reading.md`. Gehen Sie mindestens auf die folgenden Punkte
ein:

1. Welche Abhängigkeiten zu externen Crates gibt es?

2. In welche Module ist es aufgeteilt?

3. Wie werden die Kommandozeilen-Argumente eingelesen? Worauf baut das
   verwendete Crate auf?

4. Wozu gibt es die `enum Action`?

5. Was passiert, wenn die Journal Datei nicht angegeben wird?

6. Was sind die Tasks?

7. Wie wird eine Task angezeigt?

8. Wie wird die JSON Datei erzeugt/modifiziert/gelesen?

9. Gibt es `unwrap()`, `except()` oder `panic()` Anweisungen?

10. Wie erfolgt die Fehlerbehandlung in den verschiedenen Funktionen?

11. Wie wird die aktuelle Uhrzeit bestimmt?

12. Mit welchem Exit-Code wird das Programm bei Ok und wie bei einem Fehler beendet?




