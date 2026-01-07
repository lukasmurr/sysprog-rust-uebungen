# Übungen zur Systemnahen Programmierung in Rust 

**Blatt 10**  (letztes Übungsblatt)

Wintersemester 2025/2026 | <Hubert.Hoegl@tha.de>

<https://tha.de/~hhoegl/home/SysProgRust>

2025-12-11


**Hinweise**

Für dieses Blatt haben Sie bis 7. Januar 2026 Zeit.

Tipps für die Weihnachtszeit 

* Sie sollten wieder mal bei ihrer **Rustlings** Installation
  (<https://rustlings.rust-lang.org>) nachsehen, welche Aufgaben noch offen
  sind. Hier sind die verschiedenen Themen in `rustlings/exercises`:

  ```
  00_intro/            09_strings/         18_iterators/
  01_variables/        10_modules/         19_smart_pointers/
  02_functions/        11_hashmaps/        20_threads/
  03_if/               12_options/         21_macros/
  04_primitive_types/  13_error_handling/  22_clippy/
  05_vecs/             14_generics/        23_conversions/
  06_move_semantics/   15_traits/          quizzes/
  07_structs/          16_lifetimes/       README.md
  08_enums/            17_tests/
  ```

  Die aktuelle Version von Rustlings ist 6.5.0.  


* **Rust by Example** bietet auch viele funktionierende Code-Schnipsel und
  Aufgaben. Threads findet man in Abschnitt *20. Std misc*.

  <https://doc.rust-lang.org/rust-by-example>

<!--
* Eine weitere Quelle für Aufgaben ist **Rust by Practice**, auch gerne mal
  reinschauen, was es dort so gibt:

  <https://practice.course.rs>
-->



## Aufgabe 1 (a1)

Was müssen Sie dazufügen, damit der folgende Code kompiliert?

```rust
5.times(|i| {
        println!("Ferris ate {} cookies", i);
    });
```

Die Ausgabe soll sein:

```
Ferris ate 1 cookies
Ferris ate 2 cookies
Ferris ate 3 cookies
Ferris ate 4 cookies
Ferris ate 5 cookies
```

Legen Sie ein funktionierendes Crate direkt in `a1/`.


## Aufgabe 2 (a2)

Sehen Sie sich den Code in [size_of_traitobj.rs](size_of_traitobj.rs) an.  Wenn
man das Programm ausführt, wird nacheinander die Grösse von bestimmten Werten im
Programm ausgegeben. Die Ausgaben haben eine Marke A, B, C, ..., der Ablauf ist
wie folgt:

```
A, B, C, A, B, C, D, E, F, G, H, I
```

**(a)** Legen Sie ein Crate an unter `a2/size_of_traitobj/` so dass sie
den Quelltext von `size_of_traitobj.rs` kompilieren können. 

**(b)** Hinter den Marken sehen sie immer eine Zahl, die der Grösse im
Speicher in Bytes entspricht. Nur die Sachen auf dem Stack werden gezählt.
Beschreiben Sie für jede Ausgabe wie sich die Grösse genau zusammensetzt.
Schreiben Sie ihre Antwort in `a2/groessen.md`.

**(c)** Formulieren Sie `draw_text()` so um, dass nun kein Trait
Object mehr als Parameter verwendet wird, sondern ein Trait Bound: 
`fn draw_text<T>(txt: T)`. Der Parameter `T` soll den Trait `Draw` 
implementieren. Die Grösse der Werte spielen bei dieser Teilaufgabe keine 
Rolle mehr. Legen Sie den veränderten Code in ein neues Crate
`a2/trait_bound/`.  

Eine weitere schöne Erklärung von Trait-Objects (auch mit Bild) finden
Sie in dem Kurs "Comprehensive Rust" unter
<https://google.github.io/comprehensive-rust/smart-pointers/trait-objects.html>.



## Aufgabe 3 (a3)

Schreiben Sie ein deklaratives `hash_map!` Makro (also `macro_rules!`
verwenden), so dass der folgende Code kompiliert:

```rust
fn main() {
    let ages = hash_map! { "Maria" => 26, "Peter" => 32 };
    println!("{:#?}", ages);
}
```

Literatur:

* "The Rust Book", Kap. 20.5 (Macros)

* Linux Magazin, Planet Rust #11, `LM-01-2026-Planet-Rust-11-Makros.pdf`
  <https://tha.de/homes/hhoegl/home/sysprog/doc> (mit THA Login)



<!--
macro_rules! hash_map {
     ($($key:expr => $val:expr), *) => {
             {
                  HashMap::from([$(($key, $val),)*])
             }
     };
}
--> 



## Aufgabe 4 (a4)

Vollziehen Sie den Code im Kap. 21 "Final Project - Building a Multithreaded
Web Server" nach. Legen Sie den funktionierenden Code in `a4/mt-web-server`. 

