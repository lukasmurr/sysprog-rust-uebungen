# Übungen zur Systemnahen Programmierung in Rust 

**Blatt 8** 

Wintersemester 2025/2026 | <Hubert.Hoegl@tha.de>

<https://tha.de/~hhoegl/home/SysProgRust>

2025-11-27

**Hinweise**

* Lesen Sie in **The Book** das Kapitel 15 (Smart Pointers).

  <https://doc.rust-lang.org/stable/book> 

* Zur Erinnerung: Alles was sie durch KI erstellen lassen bitte kennzeichnen.
  Wenn Sie im abgegebenen Quelltext Teile durch KI erstellen lassen, dann diese
  Abschnitte kennzeichnen mit (siehe auch Moodle Ankündigung)
  
  ```
  // BEGIN AI-GENERATED SECTION
  ...
  // END AI-GENERATED SECTION
  ```

  Eigentlich besteht aber keine Notwendigkeit, KI einzusetzen.



## Aufgabe 1 (a1)

In der früheren Veranstaltung Systemnahe Programmierung [1] ging es
hauptsächlich um Assembler-Programmierung. Das verwendete "PGU" Buch [2]
(Programming from the Ground Up) hat im Kapitel 6 eine kleines Programm
beschrieben, das "Records" von Personendaten in eine Binärdatei schreibt, liest
und modifiziert.  Jeder Record besteht aus vier Teilen, drei Strings und einem
Integer: Vorname (max. 40 Zeichen), Nachname (max. 40 Zeichen), Adresse (max.
240 Zeichen) und Alter (4 Byte Integer, im "Little-Endian" Format).  Die Strings
haben am Ende ein Null-Byte, damit wird das Ende des Strings erkannt. Das ist
in der Assembler- und C-Welt so üblich.

Eine solche Binärdatei mit drei Records finden Sie in der Datei
[pgu.dat](pgu.dat). Sie können sich den Inhalt der Datei mit einem Hexdump
Tool anschauen, z.B. unter Linux mit den altbewährten Werkzeugen `xxd` oder
`hexedit`.  Es gibt  auch bereits in Rust geschriebene Hexdumper, z.B. [3]
und [4] (bitte ausprobieren!).

Ihre Aufgabe ist es nun, eine kleine Bibliothek in Rust zu schreiben, die es
erlaubt

* eine bestehende Record-Datei in eine Collection aus Records zu lesen

* die Records in der Collection zu verändern (bitte kein Aufwand an dieser 
  Stelle -- z.B. nur mal als Demo das Alter aller Einträge um `1` erhöhen)

* eine Collection aus Records in die Datei zu schreiben. Die Datei wird
  angelegt, falls sie noch nicht existiert

Ihr Rust Programm speichert die Records in Strukturen, die aus drei
Rust-Strings (diese sind nicht Null-terminiert!) und einem Integer bestehen.
Die feste Länge der String-Felder in der Datei wird also von dem Anwender der
Bibliothek verborgen. Sie müssen keine interaktive Eingaben in ihrem Programm
machen, es genügt, wenn Sie ein paar Record-Testfälle fest in das Programm
einbauen.


[1] <https://tha.de/~hhoegl/home/SysProg>

[2] <https://hhoegl.informatik.hs-augsburg.de/sysprog/pgu/records.html>

[3] <https://github.com/badlydrawnrod/hexdmp>, <https://badlydrawnrod.github.io/hexdmp>

[4] <https://github.com/sharkdp/hexyl>



## Aufgabe 2 (a2)


Es geht hier wieder um "Code Reading", wie schon bei Blatt 6, Aufgabe 3.

Studieren Sie dazu das Kapitel 2 im Buch von *Lyu/Rzeznik, Practical Rust
Projects, 2. Auflage 2023*. Sie finden es in unserer Cloud, Link ist ist
auf Moodle. Man kann es auch mit Hochschul-Kennung vom [Springer-Link](https://login.ezproxy.hs-augsburg.de/login?qurl=https%3a%2f%2flink.springer.com)
herunterladen.

Den Quelltext legen sie in `a2/catsay`, so dass man darin `cargo ...` aufrufen
kann. Der Code zum Buch ist auf Github unter <https://github.com/Apress/Practical-Rust-Projects-2nd-ed.> zu finden.

Schreiben Sie kurze Antworten zu folgenden Punkten in `a2/Code-Reading.md`. 

1. Was sagt `cargo clippy`?

2. Welche Kommandozeilen-Argumente hat es?

3. Wie werden die Kommandozeilen-Argumente eingelesen? 
 
4. Welche Abhängigkeiten zu externen Crates gibt es?

5. In welche Module ist es aufgeteilt?

6. Wozu wird das `predicates` Crate verwendet?

7. Wie kann man die Ausgabe von `stdout` und `stderr` in Dateien umlenken? 

8. Bleibt die Farbe beim Umlenken in Dateien immer erhalten?

9. Wie wird die Ausgabe in Farbe erzeugt?
 
10. Gibt es `unwrap()`, `except()` oder `panic()` Anweisungen?

11. Wie erfolgt die Fehlerbehandlung in den verschiedenen Funktionen?
   Hier auch `main()` betrachten.

12. Mit welchem Exit-Code wird das Programm bei Ok und wie bei einem
    Fehler beendet?

13. Gibt es Unit-Tests? Wenn ja, wie lässt man sie laufen?

14. Gibt es Integrationstests? Wenn ja, wie lässt man sie laufen?

15. Wozu gibt es `src/bench.rs`? Wie wendet man es an?

16. Wie kann man das Programm installieren?

17. Wie kann man den Installationspfad verändern?

18. Was müssten Sie tun, damit ein anderer das Programm einfach mit
    `cargo install <programm>` installieren kann?  
