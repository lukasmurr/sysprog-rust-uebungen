# Übungen zur Systemnahen Programmierung in Rust 

**Blatt 5** 

Wintersemester 2025/2026 | <Hubert.Hoegl@tha.de>

<https://tha.de/~hhoegl/home/SysProgRust>

2025-11-06


**Hinweise**

* Lesen Sie in "The Book" (<https://doc.rust-lang.org/stable/book>)
  das 10. Kapitel (Generic Types, Traits, and Lifetimes) und das 11. Kapitel 
  (Writing Automated Tests). 

  Tipp: Ich finde die Beispielprogramme zu Traits im Kapitel 10.2 von
  "The Book"  etwas schwierig zu verstehen. Klarer sind die Beispiele auf
  "Rust by Example", siehe 

  <https://doc.rust-lang.org/rust-by-example/trait.html>


* Unter 

  <https://tha.de/homes/hhoegl/home/sysprog/Quiz/index.html>

  finden Sie einige Testfragen, die ungefähr in die Richtung gehen, wie ich mir
  das für die Klausur vorstelle. Das Ganze ist aber noch stark in Bearbeitung
  und soll einmal die wichtigsten Kapitel von "The Book" abdecken. Es sollten
  ca. 50 Fragen werden für eine Klausur mit 60 Minuten.

  Ausserdem habe ich einige Fragen aus dem "Experimental Rust Book"
  gesammelt unter:

  <https://tha.de/homes/hhoegl/home/sysprog/ERB-Quiz/index.html>

  Diese finden Sie in schönerer Form (interaktiv, vollständig) auch
  im Material von Tutor J. Knoll:

  <https://tha.de/homes/knolljo/rust>


* Nicht vergessen: Auch die den Kapiteln entsprechenden Abschnitte
  in `rustlings` bearbeiten.



## Aufgabe 1 (a1)

Eine Funktion `read_value(v: &mut Multitype)` soll unterschiedliche 
Typen in dem Argument `v` zurückgeben können. Zum Beispiel `i32`, `f32`, 
`String` und eine Struktur ihrer Wahl. Würfeln Sie in der Funktion per 
Zufall einen dieser Typen aus und geben ihn zurück. Rufen Sie die Funktion
wiederholt auf und prüfen Sie den zurückgegebenen Typ. 



## Aufgabe 2 (a2)

Suchen Sie sich eine Rust Aufgabe auf <https://exercism.org> aus, die ihnen
Spass macht und die mindestens von der Schwierigkeit "Medium" sein sollte und
lösen Sie diese.

<https://exercism.org/tracks/rust/exercises>




## Aufgabe 3 (a3)

Schreiben Sie eine CLI Anwendung, die mit Hilfe des `sysinfo` Crates
einige Informationen über ihren Rechner ausgibt. Verwenden Sie

* Kommandozeilen Argumente mit `clap` (siehe das `cargo generate` Template).
  Damit soll man steuern können, welche Informationen angezeigt werden.  
  Sie bestimmen selbst, welche Argumente sie aufnehmen.

* Fehlerbehandlung (es sollen keine panic und expect Anweisungen zum
  Abbruch führen).

* `cargo clippy` soll ohne Meldungen durchlaufen.

Das sysinfo Crate ist hier: <https://crates.io/crates/sysinfo>
