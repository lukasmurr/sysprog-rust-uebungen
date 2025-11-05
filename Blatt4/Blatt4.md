# Übungen zur Systemnahen Programmierung in Rust 

**Blatt 4** 

Wintersemester 2025/2026 | <Hubert.Hoegl@tha.de>

<https://tha.de/~hhoegl/home/SysProgRust>

2025-10-30


**Hinweise**

Lesen Sie in **The Book** (<https://doc.rust-lang.org/stable/book/>)
das neunte Kapitel über *Error Handling*.


## Aufgabe 1 (a1)

Folgendes Python Programm habe ich schon oft verwendet um zu demonstrieren,
wie knapp und selbstsprechend man ein Programm schreiben kann, um eine 
zufällige Zeile aus einer Datei zu lesen und anzuzeigen. 

```python
import random

def choose_line():
    questions = open('random_choice_lines.txt').read().splitlines()
    return random.choice(questions)

print(choose_line())
```
Die Datei `random_choice_lines.txt` enthält folgende Zeilen:

```text
Python ist sehr gut lesbar.
Python kann man sich sehr gut merken.
Python ist sehr gut zum Programmierenlernen geeignet.
Python kann man gut mit eigenen C-Bibliotheken erweitern.
Python ist gut für numerische und wissenschaftliche Berechnungen.
Python ist gut für Bildverarbeitung.
Python kann gut die Programmausführung parallelisieren.
Python kann gut grosse Datenmengen analysieren.
Python ist ideal für GIS Anwendungen geeignet.
Python ist gut für die Programmierung grafischer Oberflächen geeignet.
Python ist ideal für Deep Learning geeignet.
Python kann Matlab ersetzen.
Python ist ausführbarer Pseudocode.
```

**Aufgabe:** Übertragen Sie das Programm nach Rust, so dass es auch möglichst
knapp und lesbar ist ("idiomatisches Rust"). Die Datei mit den Sätzen ändern 
Sie auch ab, so dass sie Aussagen enthält, von denen Sie der Meinung sind, dass
sie für Rust zutreffen. Das muss nicht alles aus der Werbeabteilung sein, es 
darf auch Kritik dabei sein.


## Aufgabe 2 (a2)

Diese Aufgabe erweitert wieder das Exit-Code Programm von Blatt 3 / Aufgabe 3.
Sie erinnern sich an die Kommandozeilen-Argumente

```text
--help                   # gibt Hilfetext aus
-e <c>, --exit=<c>       # Exit-Code <c>
-d <s>, --delay=<s>      # Delay in Sekunden (default: 0 sec)
```

Es geht nun darum noch eine weitere Option `--echo` einzubauen, die Eingaben
von `stdin` einfach auf `stdout` weiterleitet. Wenn auf `stdin` ein Punkt (`.`)
entdeckt wird, wird die  Weiterleitung abgebrochen und das Programm bricht ab.
Die Argumente können bei Ihnen auch anders heissen.

Das Programm hat den folgenden Zweck: Mit dem Python `subprocess` Modul aus
der Python Standardbibliothek kann man externe Programme starten.  Man kann
die externen Programme mit `subprocess` so starten, dass man z.B. den Exit-Code
des  externen Programmes abfragen kann, oder man kann das externe Programm über
"Pipes" (z.B. `stdin` und `stdout`) mit dem aufrufenden Programm verbinden,
so dass man Nachrichten hin und her schicken kann. Das obige Programm mit
den  Argumenten dient dabei einfach als "Dummy Executable" das ein definiertes
Verhalten beim Aufrufen durch Setzen der Argumente erzeugen kann.

**Aufgaben:**

1. Fügen Sie noch die fehlende Option für Echo `stdin -> stdout` in das 
   Exit-Code Programm ein. Nennen Sie dieses Programm `p2`.  Die Delay Option
   soll nur zusammen mit dem Exit-Code gültig sein. Bei der Echo Option ist das
   Delay also zu ignorieren.

2. Schreiben Sie in Rust ein Programm `p1`, das es erlaubt, das andere Programm
   `p2` zu starten und dessen Optionen der Reihe nach zu testen:

   a. Exit-Code. `p1` startet `p2` mit der `-e ...` Option und prüft, ob
      der angegebene Exit Code zurückgeliefert wird. 
 
   b. Verzögerungszeit bis zum Exit. `p1` startet `p2` mit der `-d ...`
      Option und prüft, ob die angegebene Verzögerungszeit eingehalten
      wurde. 

   c. Verbindung von stdin zu stdout über die `--echo` Option, bis ein
      '.' eingegeben wird. `p1` liest dazu einen String von der Tastatur
      (mit Eingabetaste abschliessen), schickt den String über stdout zu
      `p2` und prüft ob der gleiche String über stdin wieder hereinkommt.
      Die Eingabe von Punkt (`.`) als einziges Zeichen bricht die Verbindung ab.

   Verwenden Sie zum Starten des externen Programms das `subprocess` Crate.

Beide Programme sollen Sie in ein einziges Crate in das `src/bin`
Verzeichnis packen, so dass man kompilieren kann mit `cargo build --bin p1`
und `cargo build --bin p2`.
Der Pfad zu der ausführbaren Datei kann bei `p1` in einer 
Kommandozeilenoption angegeben werden, ich schlage `--bin` vor:

```sh
$ p1 --bin /pfad/zu/p2
``` 

Den Ablauf können Sie in ein Shell Skript schreiben,
also `p2` kompilieren, dann `p1` kompilieren, dann `p1` aufrufen. Die
Tests die `p1` macht schreiben einfache Ausgaben auf die Konsole, z.B. so:

```
exit code ok/nicht ok
delay ok/nicht ok
echo ok/nicht ok
```



