use rand::Rng;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

mod hangman_pics;
use hangman_pics::HANGMAN_PICS;

// Struct: Spielzustand
struct GameState {
    word: String,               // zu erratendes Wort
    guessed_chars: Vec<char>,   // bereits geratene Buchstaben
    missed_guesses: usize,      // Anzahl Fehlversuche
    missing_chars_count: usize, // Anzahl nicht erratener Buchstaben
    char_array: Vec<char>,
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
    // Autor: David
    fn guess_letter(&mut self, letter: char) -> bool {
        if self.guessed_chars.contains(&letter) {
            println!("Der Buchstabe wurde bereits versucht");
            return false;
        }
        // Prüfe, ob Buchstabe im Wort enthalten und füge zu guessed_chars hinzu
        self.guessed_chars.push(letter);
        let rigth_guess = self.update_array(letter);
        // Erhöhe missed_guesses, falls nicht gefunden
        if !rigth_guess {
            self.missed_guesses += 1;
        }
        // Gib true, falls Buchstabe im Wort, sonst false
        rigth_guess
    }

    // Gewinn-Prüfung
    // Autor: David
    fn has_won(&self) -> bool {
        !self.char_array.contains(&'_')
    }

    // Verlust-Prüfung
    // Autor: David
    fn has_lost(&self) -> bool {
        self.missed_guesses >= HANGMAN_PICS.len()
    }

    // Ausgabe des aktuellen Wortes (mit Unterstrichen für ungeratene Buchstaben)
    // Autor: David
    fn display_word(&self) -> String {
        let joined: String = self
            .char_array
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        joined
    }

    // Galgen anzeigen
    fn display_hangman(&self) {
        println!("{}", HANGMAN_PICS[self.missed_guesses]);
    }

    // Autor: David
    // geratene Buchstaben aktualisieren
    fn update_array(&mut self, c: char) -> bool {
        let mut positions = Vec::<usize>::new();
        for (i, word_char) in self.word.chars().enumerate() {
            if word_char == c {
                positions.push(i);
            }
        }
        if positions.is_empty() {
            return false;
        }
        self.missing_chars_count -= positions.len();
        for position in positions {
            self.char_array[position] = c;
        }
        true
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

// Speichere einen Eintrag ins Leaderboard (Datei: src/leaderboard)
fn save_leaderboard_entry(name: &str, missed_guesses: usize) {
    let path = "src/leaderboard";
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("Konnte Leaderboard-Datei nicht öffnen");
    writeln!(file, "{}:{}", name, missed_guesses).expect("Konnte Eintrag nicht schreiben");
}

// Zeige das Leaderboard an, falls vorhanden
fn display_leaderboard() {
    let path = "src/leaderboard";
    let file = File::open(path);
    match file {
        Ok(f) => {
            let reader = BufReader::new(f);
            println!("\n--- Leaderboard ---");
            for (i, line) in reader.lines().enumerate() {
                if let Ok(entry) = line {
                    // Format: name:missed
                    let parts: Vec<&str> = entry.splitn(2, ':').collect();
                    if parts.len() == 2 {
                        println!("{}. {} (Fehler: {})", i + 1, parts[0], parts[1]);
                    } else {
                        println!("{}. {}", i + 1, entry);
                    }
                }
            }
            println!("-------------------\n");
        }
        Err(_) => println!("Kein Leaderboard vorhanden."),
    }
}

fn user_guess(wrong_input: bool) -> char {
    if wrong_input {
        println!("Das war kein einzelner Buchstabe! Versuchs nochmal: ")
    } else {
        println!("Rate einen Buchstaben");
    }
    let mut c = String::new();
    io::stdin()
        .read_line(&mut c)
        .expect("Da ist leider etwas schiefgelaufen!");

    let guess: char = match c.trim().parse() {
        Ok(c) => c,
        Err(_) => user_guess(true),
    };

    guess
}

// Spiellogik: Hauptfunktion
fn main() {
    // ToDo Lukas: Game Menu
    // TODO: Menü anzeigen
    // 1. Neues Spiel starten
    // 2. Leaderboard anzeigen
    // 3. Spiel beenden

    let selected_word = get_random_word();
    // println!("{}", selected_word);

    let mut game_state = GameState::new(&selected_word);
    // Platzhalter array erstellen und mit Platzhaltern füllen.
    loop {
        // Aktuellen Spielstand ausgeben und Array mit Spielstand anzeigen
        game_state.display_hangman();
        println!("{}", game_state.display_word());
        // println!("{}", game_state.display_word());
        // David: Benutzereingabe lesen
        let c = user_guess(false);
        game_state.guess_letter(c);
        // David: Eingabe verarbeiten und GameState aktualisieren
        // David: Nach Sieg oder Niederlage ausgeben
        if game_state.has_won() {
            println!("gewonnen");
            println!("Das Wort war: {}", game_state.word);
            
            println!("Gib deinen Namen fürs Leaderboard ein (leer lassen = nicht speichern):");
            let mut name = String::new();
            io::stdin()
                .read_line(&mut name)
                .expect("Fehler beim Lesen des Namens");
            let name = name.trim();
            if !name.is_empty() {
                save_leaderboard_entry(name, game_state.missed_guesses);
                println!("Eintrag gespeichert.");
            } else {
                println!("Kein Name eingegeben, nicht gespeichert.");
            }
            display_leaderboard();
            break;
        }
        if game_state.has_lost() {
            println!("verloren");
            println!("Das Wort war: {}", game_state.word);
            break;
        }
    }
}
