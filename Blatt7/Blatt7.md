# Übungen zur Systemnahen Programmierung in Rust 

**Blatt 7** 

Wintersemester 2025/2026 | <Hubert.Hoegl@tha.de>

<https://tha.de/~hhoegl/home/SysProgRust>


## Aufgabe 0 (a0)

Im Verzeichnis `cli_integration_test` finden Sie in `tests/integration.rs`
einen **Integrationstest**,  den Sie mit `cargo test` laufen lassen können.
Das Programm das dabei getestet wird ist in  `src/main.rs`. Es ist noch leer,
deswegen melden drei der vier Tests noch einen Fehler. Schreiben Sie das
Hauptprogramm, so dass alle Tests bestanden werden.  Die Ausgabe soll also
in etwa so aussehen (die Reihenfolge kann auch anders sein):

```
...
running 4 tests
test exit_fail ... ok
test exit_ok ... ok
test msg_ok_1 ... ok
test msg_ok_2 ... ok
...
```

 

## Aufgabe 1 (a1)

Schreiben Sie die Collatz Aufgabe vom Blatt 1 nun indem sie den 
Iterator Trait implementieren. Der Aufruf soll dann z.B. wie folgt möglich 
sein:

```
println!("{:?}", Collatz(131).collect::<Vec<_>>());
```



## Aufgabe 2 (a2)

1. Finden Sie heraus, was die folgende Funktion macht.

2. Schreiben Sie ein paar Testfälle (Unit-Tests) für die Funktion, die sowohl
   den Ok-Fall als auch den Fehler-Fall abdecken.

3. Kann man die Funktion noch kürzer schreiben?

```rust
fn parse_list(input: &str) -> Result<Vec<Vec<u32>>, <u32 as FromStr>::Err> {    
    input                                                                       
        .split(';')                                                             
        .map(|s| s.trim())                                                      
        .filter(|s| !s.is_empty())                                              
        .map(|s| {                                                              
            s.split(',')                                                        
            .map(|s| s.trim())                                              
            .filter(|s| !s.is_empty())                                      
            .map(|s| s.parse())                                             
            .collect()                                                      
        })                                                                      
        .collect()                                                              
}                
```


## Aufgabe 3 (a3)


Beim Blatt 3, Aufgabe 2 (Mittelwert, Median, häufigste Zahl) gab es eine Lösung
zur häufigsten Zahl die wie folgt war (`numbers` ist ein sortierter Vektor mit
ganzen Zahlen). Ihre Aufgabe ist es, die Aufrufe der Methoden zu dokumentieren
(was macht jeder Aufruf?) und ein paar Testfälle dafür zu schreiben. Sie müssen
in ihrem Crate die Abhängigkeit zu dem Crate `itertools` aufnehmen.


```
numbers
    .iter()
    .copied()
    .group_by(|x| *x)
    .into_iter()
    .map(|(k, v)| (k, v.count()))
    .max_by_key(|(_, v)| *v)
    .unwrap()
    .0,
```


