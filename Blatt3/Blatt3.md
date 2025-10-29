# Übungen zur Systemnahen Programmierung in Rust

**Blatt 3**

Wintersemester 2025/2026 | <Hubert.Hoegl@tha.de>

2025-10-23

<https://tha.de/~hhoegl/home/SysProgRust>

**Hinweise**

* Lesen Sie in **The Book** (<https://doc.rust-lang.org/stable/book>)
  das sechste Kapitel (Enums and Pattern Matching) und das achte Kapitel
  (Common Collections).  

* Machen Sie alle Rustlings Aufgaben bis einschliesslich aller `hashmap`
  Übungen. Mit dem Kommando `rustlings list` sehen Sie alle bisher bearbeiteten
  und unbearbeiteten Aufgaben.

* Auch "Rust by Example" ist immer einen Besuch wert:
  <https://doc.rust-lang.org/stable/rust-by-example>.
  
## Aufgabe 1 (a1)

Hier soll ein Cargo Package entstehen, das eine Bibliothek und eine ausführbare
Datei enthält.  Der Bibliotheksteil ist in [input.rs](input.rs).

Holen Sie sich von der Standardeingabe in einer Zeile mehrere ganze Zahlen,  die
mit einem Leerzeichen voneinander getrennt sind (siehe `input()` in der Lib).
Berechnen Sie aus der Folge von Zahlen

(1) den Mittelwert (Summe durch Anzahl)
(2) den Medianwert (den Wert in der Mitte wenn die Zahlenfolge sortiert ist)
(3) den am häufigsten vorkommenden Wert

Bei (3) soll man eine `HashMap` verwenden, welche die Zahlenwerte auf ihre
Häufigkeit abbildet.

## Aufgabe 2 (a2)

Sie sollen einen einfachen Reaktionszeitmesser bauen, der auf dem Bildschirm
nach einer zufälligen Zeitverzögerung eine Ausgabe macht, auf die man dann als
Reaktion eine Taste betätigen muss. Die Zeit zwischen der Ausgabe und dem
Tastendruck wird gemessen. Das Drücken einer einzelnen Taste soll mit dem
externen Crate `termion` festgestellt werden. In [reaktion.rs](reaktion.rs)
finden Sie schon mal ein paar Bausteine für diese Aufgabe.

Die Aufgabe lässt sich in einem zweiten Schritt etwas anspruchsvoller
gestalten, wenn man in der Reaktionszeit z.B. kleine Rechenaufgaben lösen
muss, die einem spontan gestellt werden.

Im Buch von Eshwarla, Practical System Programming for Rust Developers, Packt
2020, gibt es einen Abschnitt über Termion:

* Chapter 7: Implementing Terminal I/O in Rust

Das Buch ist auf dem Cloud Link zu finden, siehe den Moodle Kurs.

## Aufgabe 3 (a3)

Das Exit-Code Programm vom Blatt 2, Aufgabe 3 soll so erweitert werden, dass
Kommandozeilenargumente von dem Crate `clap` verwaltet werden.

Es soll die folgenden Argumente geben:

```text
--help                   # gibt Hilfetext aus
-e <c>, --exit=<c>       # Exit-Code <c>
-d <s>, --delay=<s>      # Delay in Sekunden (default: 0 sec)
```

Wenn man das Programm ohne Argumente aufruft, soll der Hilfetext ausgegeben
werden.

Nach dem Aufruf beendet sich das Programm nach `<s>` Sekunden mit dem Exit-Code
`<c>`, ohne irgendetwas zu machen (das Programm "schläft" einfach solange).

Eine Beschreibung von "clap" findet man hier: <https://docs.rs/clap-v3>.
