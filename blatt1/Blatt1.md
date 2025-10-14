# Übungen zur Systemnahen Programmierung in Rust 

**Blatt 1** 

Wintersemester 2025/2026 | <Hubert.Hoegl@tha.de>

<https://tha.de/~hhoegl/home/SysProgRust>


**Hinweise**

* Legen sie ein Repository `sysprog_uebungen` auf
  `gitlab.informatik.hs-augsburg.de`. Nehmen sie mich (@hhoegl) und Johannes
  Knoll (Tuto @knolljo) als Maintainer auf.

* Schicken Sie mir den Link auf das Repository mit dem `curl` Tool nach
  folgendem Beispiel (Name, Matr-Nr, URL auf Repo): 

  ```
  curl -u api:trick17 "hhoegl.informatik.hs-augsburg.de:8881/hsa/send_snp_url.py?name=Hans_Muster&mat=12345&url=https://hier/den/url/angeben"
  ```

  Achtung: Leerzeichen im URL muss man mit `%20` schreiben, vermeiden sie diese
  am besten.

* Die Übungsaufgaben geben Sie in diesem Repository bis zur wöchentlichen
  Deadline ab. Sie sollten dazu die einzelnen Blätter in separate Unterverzeichnisse und diese in einzelne
  Aufgaben a1, a2, ...  wie folgt untergliedern:

  ```
  README.md
  Blatt1/
      a1/
      a2/
      ...
  Blatt2/
      a1/
      ...
  Blatt3/
      a1/
      ...
  ...
  ```
  
  In die Datei `README.md`, die an oberster Stelle in jedem Repository liegt,
  tragen Sie sich als Teilnehmer ein mit Vorname, Name,  Matrikelnummer und
  E-mail Adresse. Zur Formatierung wählen Sie das "Markdown" Format, wie auch
  bei anderen Texten, die sie erstellen müssen (Endung .md).

* Lesen Sie in **The Book** (<https://doc.rust-lang.org/stable/book/>)
  die ersten drei Kapitel.

* Rustlings" Projekt unter <https://rustlings.rust-lang.org>

  Es präsentiert ihnen Aufgaben lokal auf ihrem Rechner in einer Shell. Wie
  man es installiert finden sie unter "Getting Started".  Ihre Aufgabe ist
  es, das Programm startklar zu machen und begleitend zum Buch die Beispiele
  durchzuarbeiten.
  
  **Den Rustlings Quelltext bitte NICHT abgeben!**. Es genügt wenn das Programm
  auf ihrem Rechner zur Verwendung liegt.



## Aufgabe 1 (a1) 

Lesen Sie den Inhalt der folgenden Seite

<https://www.rust-lang.org/learn/get-started>

und fassen Sie die wesentlichen Aussagen in einer Datei
`Blatt1/a1/getting-started.md` zusammen. 

Vollziehen Sie das Beispiel mit
`ferris-says` nach. Legen Sie den Code in den
Ordner `Blatt1/a1/hello-rust/`.



## Aufgabe 2 (a2)

Vollziehen Sie das Kapitel 2 mit dem "Guessing Game" von Anfang bis Ende
genau nach und legen Sie das Crate in den Ordner `a2`:
`Blatt1/a2/guessing_game/`.



## Aufgabe 3 (a3)

Sehen Sie sich die Website "Rust by Example" an:

<https://doc.rust-lang.org/rust-by-example/>

Vollziehen Sie die Beispiele in 1 "Hello World" und 2 "Primitives" 
nach. Legen Sie den Quelltext jeweils in einem eigenen Crate ab, so dass man
in jedes Crate gehen und mit `cargo run` das Programm aufrufen kann. 

```
a3/
   hello_world/
              comments/
              printf
   primitives/
              literals/
              tuples/
              arrays/
```


## Aufgabe 4 (a4)

Schreiben Sie ein Programm, das für eine bestimmte Anfangszahl `n` die Folge
der Collatz-Zahlen berechnet. Die Anfangszahl holen sie von der Kommandozeile
als erstes Argument.

<https://de.wikipedia.org/wiki/Collatz-Problem>

Den Quelltext legen Sie in `Blatt1/a6/`, so dass man dort `cargo run -- <n>`
auführen kann. 



