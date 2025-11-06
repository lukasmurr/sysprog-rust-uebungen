use rand::Rng;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

mod hangman_pics;
use hangman_pics::HANGMAN_PICS;

// Struct: Spielzustand
struct GameState {
    word: String,             // zu erratendes Wort
    guessed_chars: Vec<char>, // bereits geratene Buchstaben
    missed_guesses: usize,      // Anzahl Fehlversuche
    missing_chars_count: usize  ,// Anzahl nicht erratener Buchstaben
    char_array: Vec<char>
}

// Implementierung: Methoden für GameState
impl GameState {
    // Initialisierung
    fn new(word: &str) -> Self {
        GameState {
            word: word.to_string(),
            guessed_chars: Vec::new(),
            missed_guesses: 0,
            missing_chars_count: word.chars().count(),
            char_array: vec!['_'; word.chars().count()],
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
        println!("{}", HANGMAN_PICS[self.missed_guesses as usize]);
    }

    // Autor: David
    fn update_array(&mut self, c: char) -> bool{
        println!("{}", self.word);
        let mut positions = Vec::<usize>::new();
        for (i, word_char) in self.word.chars().enumerate(){
            if word_char == c{
                println!("{}", word_char);
                positions.push(i);
            }
        }
        if positions.len() == 0{
            return false;
        }
        for position in positions{
            self.char_array[position] = c; 
        }
        return true;
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
    words[random_index].to_lowercase().clone()
}

fn get_chars_as_string(chars: &Vec<char>) -> String{
    let joined: String = chars.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(" ");
    return joined;
}

fn user_guess(wrong_input: bool) -> char{
    if wrong_input{
        println!("Das war kein einzelner Buchstabe! Versuchs nochmal: ")
    }
    else {
        println!("Rate einen Buchstaben");
    }
    let mut c = String::new();
    io::stdin().read_line(&mut c).expect("Da ist leider etwas schiefgelaufen!");

    let guess: char = match c.trim().parse() {
        Ok(c) => c,
        Err(_) => user_guess(true),
    };

    return  guess;
}


// Spiellogik: Hauptfunktion
fn main() {
    // ToDo Lukas: Game Menu
        // TODO: Menü anzeigen
        // 1. Neues Spiel starten
        // 2. Leaderboard anzeigen
        // 3. Spiel beenden

    let selected_word = get_random_word();
    println!("{}", selected_word);

    
    let mut game_state = GameState::new(&selected_word);
    // Platzhalter array erstellen und mit Platzhaltern füllen.
    loop {
        // Aktuellen Spielstand ausgeben und Array mit Spielstand anzeigen
        println!("{}", HANGMAN_PICS[game_state.missed_guesses]);
        println!("{}", get_chars_as_string(&game_state.char_array));
        // TODO David: Benutzereingabe lesen
        let c = user_guess(false);
        let right_guess = game_state.update_array(c);
        if !right_guess{
            game_state.missed_guesses += 1;
        }
        // TODO David: Eingabe verarbeiten und GameState aktualisieren
        // TODO David: Nach Sieg oder Niederlage ausgeben
        println!("{}", get_chars_as_string(&game_state.char_array));
        return
    }
    // TODO Lukas: Leaderboard anzeigen
        // Wenn gewonnen Namen anlegen und im Leaderboard speichern
}
