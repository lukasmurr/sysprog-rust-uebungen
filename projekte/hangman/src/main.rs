use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader};

const HANGMAN_PICS: [&str; 7] = [
    "
  +---+
  |   |
      |
      |
      |
      |
=========",
    "
  +---+
  |   |
  O   |
      |
      |
      |
=========",
    "
  +---+
  |   |
  O   |
  |   |
      |
      |
=========",
    "
  +---+
  |   |
  O   |
 /|   |
      |
      |
=========",
    "
  +---+
  |   |
  O   |
 /|\\  |
      |
      |
=========",
    "
  +---+
  |   |
  O   |
 /|\\  |
 /    |
      |
=========",
    "
  +---+
  |   |
  O   |
 /|\\  |
 / \\  |
      |
=========",
];

// Struct: Spielzustand
struct GameState {
    word: String,             // zu erratendes Wort
    guessed_chars: Vec<char>, // bereits geratene Buchstaben
    missed_guesses: i32,      // Anzahl Fehlversuche
}

// Implementierung: Methoden für GameState
impl GameState {
    // Initialisierung
    fn new(word: &str) -> Self {
        GameState {
            word: word.to_string(),
            guessed_chars: Vec::new(),
            missed_guesses: 0,
        }
    }

    // Buchstaben prüfen
    fn guess_letter(&mut self, letter: char) -> bool {
        // TODO: Prüfe, ob Buchstabe bereits geraten
        // TODO: Prüfe, ob Buchstabe im Wort enthalten und füge zu guessed_chars hinzu
        // TODO: Erhöhe missed_guesses, falls nicht gefunden
        // TODO: Gib true, falls Buchstabe im Wort, sonst false
        true
    }

    // Gewinn-Prüfung
    fn has_won(&self) -> bool {
        // TODO: Prüfe, ob alle Buchstaben gefunden sind
        false
    }

    // Verlust-Prüfung
    fn has_lost(&self) -> bool {
        // TODO: Prüfe, ob Anzahl Fehlversuche zu hoch
        false
    }

    // Ausgabe des aktuellen Wortes (mit Unterstrichen für ungeratene Buchstaben)
    fn display_word(&self) -> String {
        // TODO: Zeige Wort mit _ für ungeratene Buchstaben
        String::new()
    }

    // Galgen anzeigen
    fn display_hangman(&self) {
        // TODO: Zeige den aktuellen Galgenstand anhand von missed_guesses
        println!("{}", HANGMAN_PICS[self.missed_guesses as usize]);
    }
}

// Funktion: Zufälliges Wort aus wordlist auslesen
// Autor: Lukas Murr
fn get_random_word() -> String {
    let file = File::open("src/wordlist").expect("Wörterliste nicht gefunden");
    let reader = BufReader::new(file);

    let mut words: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Zeile könnte nicht gelesen werden");
        if !line.is_empty() {
            words.push(line);
        }
    }

    if words.is_empty() {
        panic!("Keine Wörter in der Wörterliste gefunden");
    }

    let random_index = rand::rng().random_range(0..words.len());
    words[random_index].clone()
}

// Spiellogik: Hauptfunktion
fn main() {
    let selected_word = get_random_word();
    println!("{}", selected_word);

    let mut game_state = GameState::new(&selected_word);
    loop {
        // TODO David: Benutzereingabe lesen
        // TODO David: Eingabe verarbeiten und GameState aktualisieren
        // TODO David: Nach Sieg oder Niederlage ausgeben
    }
}
