# Übungen zur Systemnahen Programmierung in Rust 

**Blatt 10**  (letztes Übungsblatt)

Wintersemester 2025/2026 | <Hubert.Hoegl@tha.de>

<https://tha.de/~hhoegl/home/SysProgRust>


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


## Aufgabe 2 (a2)

Sehen Sie sich den Code in `size_of_traitobj.rs` an (ist auch in diesem
Verzeichnis). Wenn man das Programm ausführt, wird nacheinander die Grösse von
bestimmten Werten im Programm ausgegeben. Die Ausgaben haben eine Marke A, B,
C, ..., der Ablauf ist wie folgt:

```
A, B, C, A, B, C, D, E, F, G, H, I
```

*Teilaufgabe 1:* Hinter den Marken sehen sie immer eine Zahl, die der Grösse im
Speicher in Bytes entspricht. Nur die Sachen auf dem Stack werden gezählt.
Beschreiben Sie für jede Ausgabe wie sich die Grösse genau zusammensetzt.


*Teilaufgabe 2:* Formulieren Sie `draw_text()` so um, dass nun kein Trait
Object mehr als Parameter verwendet wird, sondern ein Trait Bound: 
`fn draw_text<T>(txt: T)`. Der Parameter `T` soll den Trait `Draw` 
implementieren. Die Grösse der Werte spielen bei dieser Teilaufgabe keine 
Rolle mehr.



## Aufgabe 3 (a3)

Schreiben Sie ein `macro_rules!` Makro `hash_map!`, so dass der folgende Code
kompiliert:

```rust
fn main() {
    let ages = hash_map! { "Maria" => 26, "Peter" => 32 };
    println!("{:#?}", ages);
}
```

Literatur: "The Rust Book", Kap. 19.5

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

Tipps für die Weihnachtszeit 

* Sie sollten wieder mal bei ihrer **Rustlings** Installation nachsehen, welche
  Aufgaben noch offen sind. Man kann dazu das Kommando `rustlings list` eingeben, 
  es gibt `Done` oder `Pending` aus.  Es gibt schöne Aufgaben zu 

  - Error Handling
  - Traits
  - Generics
  - Smart Pointer (Box, Arc, Rc)
  - Threads 
  - Makros
  - Typumwandlung (from, into)


* **Rust by Example** bietet auch viele Aufgaben, ähnlich wie Rustlings:

  <https://doc.rust-lang.org/rust-by-example>


* Eine weitere Quelle für Aufgaben ist **Rust by Practice**, auch gerne mal
  reinschauen, was es dort so gibt:

  <https://practice.course.rs>


* Die Website <https://turbo.fish> hat mir neulich jemand gezeigt. Die ist 
  nur zum Spaß da und zeigt vorbeifliegende "Turbofish" Symbole. Sie 
  erinnern sich noch an `::<>` Operator? Den Quelltext dazu findet man auf
  Github unter <https://github.com/jplatte/turbo.fish>, es ist eine 
  Webanwendung.
  


