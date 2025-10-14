# Übungen zur Systemnahen Programmierung in Rust 

**Blatt 2** 

Wintersemester 2025/2026 | <Hubert.Hoegl@tha.de>

<https://tha.de/~hhoegl/home/SysProgRust>

Tipp: Vollziehen Sie immer alle Beispielprogramme nach, die wir in der 
Veranstaltung besprochen haben, also `01-*.rs`, `02-*.rs`, u.s.w.

## Aufgabe 1 (a1)

Finden Sie die Fehler in [word_counter.rs](word_counter.rs). Das Programm soll
zählen, wie oft jedes Wort in einer Eingabedatei vorkommt. Der Aufruf erfolgt
so:

```bash
word_counter words
```

Die Datei `words` enthält beliebige Wörter. Diese müssen sie selber anlegen.
Was passiert, wenn `words` beim Aufruf nicht angegeben wird?

Die korrigierte Quelltextdatei legen Sie in den Ordner `Blatt2/a1`.


## Aufgabe 2 (a2)

Wer das "Guessing Game" aus Kapitel 2 noch nicht gemacht hat, der sollte sich
nun damit beschäftigen. Den Quelltext übertragen Sie wie in Blatt 1 angegeben 
in das Verzeichnis `Blatt1/a3/`.


## Aufgabe 3 (a3)

Schreiben Sie ein kleines Kommandozeilenprogramm, das zum Eingeben und Abfragen
von Telefonnummern dient.  Ein minimales "User Interface" könnte so
aussehen:

```text
telbuch> ! name number  
telbuch> ? name          
telbuch> .              
```

Der String `telbuch>` ist die Eingabeaufforderung (Prompt). Mit dem
Ausrufezeichen `!` wird ein neuer Eintrag angelegt, z.B. `! Stefan 809127`.
Die Telefonnummer wird intern nicht als Zahl sondern als String gespeichert, so
dass man auch z.B. 0821-5586-101 oder 0160-12345678 eingeben könnte. Man soll
auch unter dem gleichen Namen mehrere Nummern eingeben können.  Mit `?
Stefan` kann man die Nummern abfragen. Falls es den Namen nicht gibt, sollte
ein passender Kommentar ausgegeben werden. Mit dem Punkt `.` wird das
Programm beendet. 

Welche weiteren Kommandos könnte man in das Programm einbauen?  Gerne dürfen
Sie es um weitere Kommandos erweitern, z.B. um eines, mit dem man Einträge
wieder löschen kann.

Zunächst sollte man das Programm so schreiben, dass die Einträge nur im 
Hauptspeicher stehen, folglich sind alle Daten weg, wenn man das Programm 
beendet. In Python gibt es z.B. das `pickle` Modul, mit dem man eine 
Datenstruktur in eine Datei schreiben kann, so dass man sie später wieder 
daraus herstellen kann.  Vielleicht finden Sie sowas ähnliches für die 
Sprache Rust - gerne auf https://crates.io, https://docs.rs und https://lib.rs
suchen!  Ein möglicher Kandidat könnte vielleicht **microkv** sein, habe es
aber noch nicht ausprobiert (siehe https://crates.io/crates/microkv).

Ein Gerüst für das Telefonbuch in Python ist hier: [telbuch.py](telbuch.py).
Das ist nur zum Vergleich mit ihrer Lösung in Rust hier.  Es war mal eine Aufgabe
aus einem Python-Kurs, es fehlen also noch ein paar Teile. Sie können ihre
Lösung in Rust aber aufbauen wie sie wollen. 

Eine Vorschlag für eine Einlesefunktion von der Tastatur ist in 
[input_loop.rs](input_loop.rs).



## Aufgabe 4 (a4)

Früher haben wir in der Systemnahen Programmierung ein kleines
Assembler-Programm geschrieben, in dem `main()` nur einen fixen Exit-Code
(z.B. 5) an den Aufrufer (Shell) zurück gegeben hat. Sonst hat das Programm
nichts gemacht.  Der Aufruf sieht dann so aus:

```bash 
$ ./programm 
$ echo $?
5
```

Mit `echo $?` gibt die Shell diesen Exit-Code aus.  Schreiben Sie so 
ein minimales Programm in Rust.

Erweitern sie ihr Programm nun, so dass sie den gewünschten Exit-Code 
auf der Kommandozeile als Argument eingeben können:

```bash 
$ ./programm 0
$ echo $?
0
```



## Aufgabe 5 (a5)

Lesen Sie in **The Book** (https://doc.rust-lang.org/stable/book/)
das vierte Kapitel (Ownership, ca. 20 Seiten) und das fünfte Kapitel
(Structs, ca. 14 Seiten).  

