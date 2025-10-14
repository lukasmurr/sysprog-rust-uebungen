# Übungen zur Systemnahen Programmierung in Rust 

**Blatt 9** 

Wintersemester 2025/2026 | <Hubert.Hoegl@tha.de>

<https://tha.de/~hhoegl/home/SysProgRust>


Siehe Foliensatz
<https://tha.de/~hhoegl/home/sysprog/Folien/error_hdl.pdf>
(mit Literaturangaben am Ende)


# Aufgaben (Fehlerbehandlung)


1. Wandeln sie den Rückgabewert von `read_to_string` im Fehlerfall in den
   Default-String "root" (fallback) mit Hilfe eines Kombinators um:

   ```rs
   f.read_to_string(&mut s).TODO(...)
   ```

   <!-- unwrap_or() -->


2. Wählen Sie an der Stelle TODO einen geeigneten Umwandler:

   ```rs
   fn main() -> Result<(), &'static str> {
           let s = vec!["apple", "mango", "banana"];
           let fourth = s.get(4).TODO("I got only 3 fruits")?;
           Ok(())
   }
   ```

   <!-- ok_or(), TCRPR, Kap. 6 -->


3. Wie kann man den folgenden Code wesentlich kürzer durch mehrfache 
   Verwendung von `unwrap_or_else()` schreiben?

   ```rs
   use std::fs::File;
   use std::io::ErrorKind;

   fn main() {
      let greeting_file_result = File::open("hallo.txt");

      let greeting_file = match greeting_file_result {
          Ok(file) => file,
          Err(error) => match error.kind() {
              ErrorKind::NotFound => match File::create("hallo.txt") {
                  Ok(fc) => fc,
                  Err(e) => panic!("Problem beim Erstellen der Datei: {:?}", e),
              },
              other_error => {
                  panic!("Problem beim Öffnen der Datei: {:?}", other_error)
              }
          },
      };
   }
   ```
   <!-- Siehe The Book, Kap. 9.2 -->


4. Was macht der folgende Code?

   ```rs                                                                           
   fn double_arg(mut argv: env::Args) -> Result<i32, String> {                     
       argv.nth(1)                                                                 
           .ok_or("Please give at least one argument".to_owned())                  
           .and_then(|arg| arg.parse::<i32>().map_err(|err| err.to_string()))      
           .map(|i| i * 2)                                                         
   }                                                                               
   ```       

   <!-- Aus Andrew Gallants Blogeintrag -->



5. Schreiben Sie eine Funktion `f5() -> Result<i32, io::Error>`, die an 
   zwei verschiedenen Zeilen einen `io::Error` ihrer Wahl zurückgibt. Behandeln
   sie diesen Fehler in der `main()` Funktion (zumindest Fehler ausgeben). 
   `f5()` wird in `main()` aufgerufen. Zum Propagieren des Fehlers nehmen Sie 
   den `?` Operator.

6. Schreiben Sie eine Funktion `f6() -> Result<i32, Box<dyn
   std::error::Error>>` in der zwei verschiedene Fehlertypen, z.B. `io::Error`
   und `num::ParseIntError` zurückgegeben werden. Zum Propagieren des Fehlers
   nehmen Sie wieder den `?` Operator. Behandeln Sie die Fehler in `main()`, 
   im einfachsten Fall geben sie einfach den Fehler am Bildschirm aus.

   Nicht unbedingt nötig, kann man aber mal ausprobieren: Der Aufrufer kann den
   originalen Typ herausfinden mit `downcast_ref`, z.B. wie folgt:

   `if let Some(io_err) = err.downcast_ref::<std::io::Error>() { ... }`

   
7. Schreiben Sie eine Funktion `f7() -> Result<i32, MeinFehlertyp>`, die einen   
   anwendungsdefinierten Fehlertyp zurück gibt.  In der Funktion werden zwei 
   verschiedenen Fehlertypen zurück gegeben (wie bei `f6()`). 
   `MeinFehlertyp` sieht wie folgt aus:

   ```rs
   #[derive(Debug)]
   enum MeinFehlertyp {
       IO(std::io::Error),
       Parsing(std::num::ParseIntError),
   }
   ```
   Verwenden Sie zum Umwandeln und Propagieren der Fehler `...map_err()?`


8. Wie 7., jedoch nun mit der Implementierung des `From` Traits für die 
   anwendungsdefinierten Fehler, so dass Sie sich `map_err()` sparen 
   können. Der `?` Operator alleine reicht in dem Fall.



<!--
Aufgabenideen:

Den Rueckgabewert von read_to_string() in einen       
anderen Typ abbilden mit Kombinatoren bzw. Umwandlern, z.B. in Ok(&str). 

let r = f.read_to_string(&mut s); 

~~~

Anyhow (https://crates.io/crates/anyhow)

`Result<T, anyhow::Error>`, `anyhow::Result<T>`

-->

