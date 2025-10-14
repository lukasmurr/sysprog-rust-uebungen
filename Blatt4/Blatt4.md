# Übungen zur Systemnahen Programmierung in Rust 

**Blatt 4** 

Wintersemester 2025/2026 | <Hubert.Hoegl@tha.de>

<https://tha.de/~hhoegl/home/SysProgRust>


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

Diese Aufgabe erweitert wieder das Exit-Code Programm von Blatt 3 / Aufgabe 4.

Vor einiger Zeit habe ich ein Programm in C geschrieben, das folgende 
Aufrufargumente kennt:

```text
(base) hhoegl@msi$ ./programm
usage: program [opts]
   -h   --  print usage
   -d n --  delay for n seconds
   -e   --  echo stdin to stdout, until '.'
   -r n  -- return with exit code n
```

Die meisten Argumente haben Sie schon in Ihr Exit-Code Programm eingebaut. Es
fehlt noch die `-e` Option, die Eingaben von `stdin` einfach auf `stdout`
weiterleitet. Wenn auf `stdin` ein Punkt (`.`) entdeckt wird, wird die 
Weiterleitung abgebrochen und das Programm bricht ab. Die Argumente können
bei Ihnen auch anders heissen.

Das Programm hatte folgenden Zweck: Mit dem Python `subprocess` Modul aus der
Python Standardbibliothek kann man externe Programme starten.  Man kann die
externen Programme mit `subprocess` so starten, dass man z.B. den Exit-Code des 
externen Programmes abfragen kann, oder man kann das externe Programm über
"Pipes" (z.B. `stdin` und `stdout`) mit dem aufrufenden Programm verbinden,
so dass man Nachrichten hin und her schicken kann. Das obige Programm mit den 
Argumenten dient dabei einfach als "Dummy Executable" das ein definiertes
Verhalten beim Aufrufen durch Setzen der Argumente erzeugen kann.


**Aufgaben:**

1. Fügen Sie noch die fehlende Option für Echo `stdin -> stdout` in das 
   Exit-Code Programm ein.

2. Schreiben Sie in Rust ein Programm, das es erlaubt, ein anderes Programm
   zu starten und sich über stdin und stdout mit dem anderen Programm zu 
   verbinden. Nach dem Starten sollen ein paar Strings über stdout an das 
   externe Programm geschickt werden und wieder über stdin eingelesen werden.

Ich habe die Lösung noch nicht selber programmiert, vermute aber, dass es 
sowohl mit bereits in Rust eingebauten Mitteln geht, die Systemaufrufe in Linux
heissen für solche Aufgaben meist `popen()` oder ähnlich, leider ist das aber
nicht portabel auf Windows (muss es in unserem Fall auch nicht sein). Sicher 
gibt es auch Hilfen durch externe Crates, z.B. findet man gleich was wenn man 
nach "Rust" und "subprocess" sucht. Probieren Sie einfach ein paar Sachen aus.



## Aufgabe 3 (a3)

Lesen Sie in **The Book** (https://doc.rust-lang.org/stable/book/)
das neunte Kapitel über *Error Handling*.



