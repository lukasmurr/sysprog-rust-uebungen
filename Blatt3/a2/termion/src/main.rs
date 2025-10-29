use rand::Rng;
use std::io::{stdin, stdout, Write};
use std::thread;
use std::time::{Duration, Instant};
use termion::input::TermRead;

fn main() {
    println!("Reaktionszeit-Messer mit Rechenaufgaben");
    println!("========================================\n");
    println!("Lösen Sie die Rechenaufgabe so schnell wie möglich!\n");
    println!("Bereit? Drücken Sie Enter zum Starten...");

    let stdin = stdin();
    stdin.keys().next();

    let mut rng = rand::thread_rng();
    let delay_ms = rng.gen_range(1000..4000);

    println!("\nWarten...");
    thread::sleep(Duration::from_millis(delay_ms));

    let num1 = rng.gen_range(1..20);
    let num2 = rng.gen_range(1..20);
    let operation = rng.gen_range(0..2);

    let (task, result) = match operation {
        0 => (format!("{} + {}", num1, num2), num1 + num2),
        _ => (format!("{} - {}", num1 + num2, num2), num1),
    };

    println!("\n>>> RECHNEN SIE: {} = ? <<<", task);
    print!("Ihre Antwort: ");
    stdout().flush().unwrap();

    let start = Instant::now();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let reaction_time = start.elapsed();

    let user_answer = input.trim().parse::<i32>().unwrap_or(-1);

    if user_answer == result {
        println!("\n RICHTIG!");
        println!("Reaktionszeit: {:.0} ms", reaction_time.as_millis());

        let time_ms = reaction_time.as_millis();
        println!(
            "Bewertung: {}",
            match time_ms {
                0..=1000 => "Blitzschnell!",
                1001..=2000 => "Sehr gut!",
                2001..=3500 => "Gut! ✓",
                3501..=5000 => "Durchschnittlich",
                _ => "Langsam...",
            }
        );
    } else {
        println!("\n FALSCH! Die richtige Antwort war: {}", result);
        println!("Zeit: {:.0} ms", reaction_time.as_millis());
    }
}
