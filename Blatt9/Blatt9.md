# Übungen zur Systemnahen Programmierung in Rust 

**Blatt 9** 

Wintersemester 2025/2026 | <Hubert.Hoegl@tha.de>

<https://tha.de/~hhoegl/home/SysProgRust>

2025-12-04


**Hinweise**

* Beantworten Sie die interaktiven Quizfragen zu Smart Pointern auf der
  Tutor Seite <https://tha.de/homes/knolljo/rust>. Ansonsten gibt es auf
  diesem Blatt keine Aufgaben zu Smart Pointern.

* Lesen Sie das Kapitel 16 in "The Book" (Fearless Concurrency).


---

Bei den Aufgaben in diesem Blatt werden sie C Code umschreiben nach Rust. Der
C Code kommt den INF Studierenden sicher bekannt vor, sie sollten ihn bereits im
Fach "Betriebssysteme" behandelt haben. Den Code finden Sie im Verzeichnis

<https://hhoegl.informatik.hs-augsburg.de/rust/betriebssysteme24>

Es geht dabei hauptsächlich um *Concurrency*. Das wird auch das Thema
nächste Woche sein.


# Aufgabe 1  

(Verzeichnis `a1/`)

Bearbeiten Sie auf dem folgenden Übungsblatt die Aufgabe

- A2 (Multiprogrammierung auf Prozessen)

<https://hhoegl.informatik.hs-augsburg.de/rust/betriebssysteme24/Praktikum1_Code-Beispiele/praktikum1.pdf>

Sie brauchen nicht die Schritte a) bis f) zu bearbeiten, erstellen sie
einfach das gefragte Programm in Rust.  Machen Sie aber eine ordentliche
Fehlerbehandlung und auch die CLI Argumente sollten mit `clap` eingelesen
werden.


# Aufgabe 2  

(Verzeichnis `a2/`)

Bearbeiten Sie auf dem folgenden Übungsblatt die Aufgabe

- A3 (Multiprogrammierung mit Threads)

<https://hhoegl.informatik.hs-augsburg.de/rust/betriebssysteme24/Praktikum1_Code-Beispiele/praktikum1.pdf>



# Aufgabe 3 

(Verzeichnis `a3/`)

Ändern Sie das Programm von Aufgabe 2 so dass die erzeugten Zufallszahlen nicht
mehr in den Threads ausgegeben werden sondern zum Hauptprogramm geschickt und
dort ausgegeben werden.



# Aufgabe 4

(Verzeichnis `a4/`)

Bearbeiten Sie Aufgabe 4 (Bank Simulation) auf folgendem Übungsblatt

<https://hhoegl.informatik.hs-augsburg.de/rust/betriebssysteme24/Praktikum2_Code-Beispiele/praktikum2.pdf>

Die im Aufgabenteil d) angegebenen Parameter soll das Programm aus
einer TOML Konfigurationsdatei lesen. Mit der `-c` Option soll eine
beliebige TOML Datei gelesen werden. Die Aufgabenteile a) bis f)
können Sie ansonsten ignorieren. Legen Sie mehrere verschiedene TOML
Dateien an und lassen Sie das Programm über ein `Makefile` oder
`Justfile` (<https://just.systems>, ist auch in Rust) laufen. Es muss
immer reibungslos funktionieren.

