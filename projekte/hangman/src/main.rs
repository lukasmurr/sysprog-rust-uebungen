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

// Spiellogik: Hauptfunktion
fn main() {
    // TODO: Liste der Wörter definieren
    // TODO: Zufälliges Wort auswählen
    // TODO: GameState initialisieren
    // TODO: Spiel-Schleife starten
    // TODO: Benutzereingabe lesen
    // TODO: Eingabe verarbeiten und GameState aktualisieren
    // TODO: Nach Sieg oder Niederlage ausgeben
}
