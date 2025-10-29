use std::io::{stdin, stdout, Write};
use std::time::{Duration, Instant};
use rand::Rng;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(
        stdout,
        "Drücke eine beliebige Taste, wenn GO erscheint...\r\n"
    )
    .unwrap();
    stdout.flush().unwrap();

    let mut rng = rand::thread_rng();
    let delay = rng.gen_range(2..=5);
    std::thread::sleep(Duration::from_secs(delay));
    write!(stdout, "GO!\r\n").unwrap();
    stdout.flush().unwrap();

    let start_time = Instant::now();

    let a = rng.gen_range(1..=10);
    let b = rng.gen_range(1..=10);
    let problem = format!("{} + {} = ", a, b);
    let correct_answer = a + b;

    write!(stdout, "{}", problem).unwrap();
    stdout.flush().unwrap();

    let stdin = stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    if let Ok(user_answer) = input.trim().parse::<i32>() {
        if user_answer == correct_answer {
            let reaction = start_time.elapsed();
            write!(
                stdout,
                "\r\nRichtig! Reaktionszeit: {:.3} Sekunden\r\n",
                reaction.as_secs_f64()
            )
            .unwrap();
        } else {
            write!(stdout, "\r\nFalsch! Die Antwort war: {}\r\n", correct_answer).unwrap();
        }
    } else {
        write!(stdout, "\r\nFehlerhafte Eingabe!\r\n").unwrap();
    }
    stdout.flush().unwrap();
}
