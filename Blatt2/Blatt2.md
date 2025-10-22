# Übungen zur Systemnahen Programmierung in Rust 

**Blatt 2** 

Wintersemester 2025/2026 | <Hubert.Hoegl@tha.de>

2025-10-16

<https://tha.de/~hhoegl/home/SysProgRust>


**Hinweise**

* Lesen Sie in **The Book** (https://doc.rust-lang.org/stable/book/)
  das vierte Kapitel (Ownership, ca. 20 Seiten) und das fünfte Kapitel
  (Structs, ca. 14 Seiten).  

* Beantworten Sie zur Selbstkontrolle die Quizfragen zu den jeweiligen
  Kapiteln die Sie im Buch lesen.  Ich habe auf der Homepage der
  Veranstaltung einen Link drauf, diese sind jedoch nicht interaktiv.
  Auf der Tutor-Seite sind viele interaktive Fragen, bei denen sie gleich
  das Ergebnis angezeigt bekommen.  

* Ich möchte gerne dazu ermuntern, in Rust geschriebene Kommandozeilenprogramme
  (engl. *CLI programs*, CLI = Command Line Interpreter) zu verwenden. Meine
  Favoriten sind

  - `fd` zum Suchen von Dateien mit bestimmten Namen - <https://github.com/sharkdp/fd>
  - `ripgrep` (`rg`) zum Suchen in Textdateien - <https://github.com/BurntSushi/ripgrep>
  - `zellij` Terminal-Multiplexer - <https://zellij.dev>
  - `helix` Editor - <https://helix-editor.com>

  Es gibt noch viel mehr, in den letzten Jahren habe ich eine Liste erstellt
  unter <https://tha.de/homes/hhoegl/home/RustTools.html>.

  Auch im Kurs wollen wir hauptsächlich CLI Programme schreiben. Es
  gibt zwei Cargo Templates vom Tutor J. Knoll die schon einen Rahmen
  vorgeben, `simple-cli-template` und `subcommand-cli-template`  unter
  <https://gitlab.com/hhoegl-tha/snp-rs>.  Wer neugierig ist kann
  sich diese schon mal ansehen.

  Ausserdem gibt es im Buch von Lyu, *Practical Rust Projects* das Kapitel 2,
  *Building a Command Line Program* (S. 9-38). Sie finden das Buch in unserer
  Cloud unter <https://cloud.hs-augsburg.de/index.php/s/sZZsinnYBCQmEoe?>, bitte
  die Sachen nur für den eigenen Gebrauch verwenden. Das ist nur eine Empfehlung
  falls sich jemand intensiver mit dem Stoff befassen möchte.

* Letzter Hinweis: Bitte halten sie sich wieder an die Vorgabe mit den
  Verzeichnisnamen, `Blatt2`, etc. Wir wollen die Abgabe-Repositories
  per Software auswerten, deshalb müssen alle die gleichen Verzeichnisse
  haben.


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

Welche weiteren Kommandos könnte man in das Programm einbauen?  Anregungen
sind:

* Alle Einträge ausgeben
* Suchen nach bestimmten Namen
* Bestimmte Einträge löschen


Zunächst sollte man das Programm so schreiben, dass die Einträge nur im 
Hauptspeicher stehen, folglich sind alle Daten weg, wenn man das Programm 
beendet. **In einem zweiten Schritt machen sie die eingegebenen Telefonnummern
auch persistent, indem sie die Daten in eine Textdatei schreiben und aus dieser
beim nächsten Programmstart wieder auslesen**. In welchem Format sie die
Datei beschreiben ist ihnen überlassen.      

Eine Vorschlag für eine Einlesefunktion von der Tastatur ist in 
[input_loop.rs](input_loop.rs).

Das Rust Crate soll in `Blatt2/a2/telbuch` sein.

Ein Gerüst für das Telefonbuch in Python ist hier: [telbuch.py](telbuch.py). 
Es war mal eine Aufgabe aus einem Python-Kurs, es fehlen auch ein paar Teile.
Sie können ihre Lösung in Rust aufbauen wie sie wollen.


## Aufgabe 3 (a3)

Früher haben wir in der Systemnahen Programmierung ein kleines
Assembler-Programm geschrieben, in dem `main()` nur einen fixen Exit-Code
(z.B. 2) an den Aufrufer (Shell) zurück gegeben hat. Sonst hat das Programm
nichts gemacht.  Der Aufruf sieht dann so aus:

```bash 
$ ./programm 
$ echo $?
5
```

Mit `echo $?` gibt die Shell diesen Exit-Code aus.  Schreiben Sie so 
ein kleines Programm in Rust.

Erweitern sie ihr Programm nun, so dass sie den gewünschten Exit-Code 
auf der Kommandozeile als Argument eingeben können:

```bash 
$ ./programm 2
$ echo $?
2
```

Sie können auch eine Variante schreiben, die eines der oben erwähnten Templates
verwendet, so dass sie z.B. so aufrufen: `./programm --exit-code 2`. Es sollte
dann auch eine Option `-h` bzw. `--help` geben die zu jeder Option einen
Hilfetext ausgibt. 

Das Rust-Crate soll in `Blatt2/a3/exit` sein.


