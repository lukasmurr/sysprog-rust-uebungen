# Übungen zur Systemnahen Programmierung in Rust 

**Blatt 5** 

Wintersemester 2025/2026 | <Hubert.Hoegl@tha.de>

<https://tha.de/~hhoegl/home/SysProgRust>


## Aufgabe 0 (a0)

Die Aufgabe 2 vom Blatt 4 soll nochmal überarbeitet werden, machen Sie das 
in dem Ordner vom letzten übungsblatt.  Hier sind ein paar Tipps:

* Nennen Sie das Programm, das aufgerufen wird `p2`.  Die Delay Option soll 
  nur zusammen mit dem Exit-Code gültig sein. Bei der Echo Option ist das 
  Delay also zu ignorieren. 

* Das Programm `p1` ruft das Programm `p2` auf. 

* Das Programm `p2` soll am Anfang zu einer ausführbaren Datei kompiliert 
  werden. Der Pfad zu der ausführbaren Datei kann bei `p1` in einer 
  Kommandozeilenoption angegeben werden. 

* Das Programm `p1` soll die verschiedenen Aufrufmöglichkeiten von `p2` 
  nacheinander testen: 

  1. Exit-Code

  2. Verzögerungszeit bis zum Exit

  3. Verbindung von stdin zu stdout, bis ein '.' eingegeben wird. `p1` schreibt
     String auf stdout und prüft ob der gleiche String über stdin wieder 
     hereinkommt. Danach wird mit '.' abgebrochen.

* Beide Programme sollen Sie in ein einziges Package in das `src/bin` Verzeichnis 
  packen. Den Ablauf können Sie in ein Shell Skript schreiben, also `p2` 
  kompilieren, dann `p1` kompilieren und mit der richtigen Option für die 
  ausführbare Datei `p2` starten. 


## Aufgabe 1 (a1)

Eine Funktion `read_value(v: &mut Multitype)` soll unterschiedliche 
Typen in dem Argument `v` zurückgeben können. Zum Beispiel `i32`, `f32`, 
`String` und eine Struktur ihrer Wahl. Würfeln Sie in der Funktion per 
Zufall einen dieser Typen aus und geben ihn zurück. Rufen Sie die Funktion
wiederholt auf und prüfen Sie den zurückgegebenen Typ. 


## Aufgabe 2 (a2)

Unter 

* <https://tha.de/homes/hhoegl/home/sysprog/Quiz/index.html>

finden Sie einige Testfragen, die ungefähr in die Richtung gehen, wie ich mir
das für die Klausur vorstelle. Das Ganze ist aber noch stark in Bearbeitung und
soll einmal die wichtigsten Kapitel von "The Book" abdecken. Es sollten ca. 40 
Fragen werden für eine Klausur mit 60 Minuten.

Ausserdem habe ich einige Fragen aus dem "Experimental Rust Book" gesammelt unter:

* <https://tha.de/homes/hhoegl/home/sysprog/ERB-Quiz/index.html>

Versuchen Sie die jetzt schon vorhandenen Fragen zu beantworten.


## Aufgabe 3 (a3)

Vor kurzem ging der Dienst *exercism.org* wieder Online, er deckt nun auch Rust
ab.  Suchen Sie sich eine Aufgabe aus, die ihnen Spass macht und die mindestens
von der Schwierigkeit "Medium" sein sollte und lösen Sie diese.

<https://exercism.org/tracks/rust/exercises>


## Aufgabe 4 (a4)

Ich bin in der vergangenen Woche zufällig auf einen Rust Kurs von Microsoft
gestossen, den ich mir dann kurz angeschaut habe. Hier ist der Link darauf:

<https://docs.microsoft.com/de-de/learn/paths/rust-first-steps/>

Er ist in 11 Module gegliedert, die Inhalte sollten Sie schon in etwa kennen,
wenn Sie die bisherigen Kapitel in "The Book" gelesen haben.  Sollten Sie sich 
noch nicht so fit fühlen, dann sollten Sie sich die Module anschauen. 

Auf alle Fälle sollten sie am Schluss die Aufgabe "Erstellen eines
Aufgabenlisten-Befehlszeilenprogramms" machen. Man muss dabei kaum selber 
programmieren, sondern kann Code-Teile von der Angabe aneinander reihen. 
Trotzdem kann man dabei einiges lernen, was den Programmaufbau betrifft.
Ach das `anyhow` Crate kann man damit kennenlernen.  Man sollte also jede
Zeile bei dem Programm versuchen zu verstehen.

Achtung: Was bei mir nicht geklappt hat, ist der folgende Auszug aus der 
`Cargo.toml` Datei. Diese Syntax versteht `cargo` nicht:

```text
...
[dependencies.serde]  # Add serde in its own section.
version = "1.0"
features = ["derive"] # We'll need the derive feature.

[dependencies.chrono]
version = "0.4"
features = ["serde"]
...
```

Statt dessen habe ich verwendet:

```text
...
chrono = { version = "0.4", features = ["serde"] }
serde_json = "1.0"    
serde = { version = "1.0", features = ["derive"] }
...
```


## Aufgabe 5 (a5)

Lesen Sie in "The Book" (https://doc.rust-lang.org/stable/book/)
das 10. Kapitel (Generic Types, Traits, and Lifetimes) und das 11. Kapitel 
(Writing Automated Tests). 

Tipp: Ich finde die Beispielprogramme zu Traits im Kapitel 10.2 von "The Book" 
etwas schwierig zu verstehen. Klarer sind die Beispiele auf "Rust by Example", 
siehe 

<https://doc.rust-lang.org/rust-by-example/trait.html>

Nicht vergessen: Auch die entsprechenden Abschnitte in `rustlings` bearbeiten.

