# Übungen zur Systemnahen Programmierung in Rust 

**Blatt 8** 

Wintersemester 2025/2026 | <Hubert.Hoegl@tha.de>

<https://tha.de/~hhoegl/home/SysProgRust>


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

Eine solche Binärdatei mit drei Records finden Sie in der Datei [pgu.dat](pgu.dat).
Sie können sich den Inhalt der Datei mit einem Hexdump Tool anschauen, z.B.
unter Linux mit den altbewährten Werkzeugen `xxd` oder `hexedit`.  Es gibt 
auch bereits in Rust geschriebene Hexdumper, z.B. [3] (bitte ausprobieren!).

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



## Aufgabe 2 (a2)

Lesen Sie in **The Book** die beiden
Kapitel 15 (Smart Pointers) und 19 (Advanced Features).

<https://doc.rust-lang.org/stable/book> 

