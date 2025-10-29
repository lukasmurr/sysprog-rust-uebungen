use rand::Rng;
use std::io::{Write, stdin, stdout};
use std::time::{Duration, Instant};
use termion::event::Key;
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
    let correct_answer = a + b;
    let mut input = String::new();
    write!(stdout, "{} + {} = ", a, b).unwrap();
    stdout.flush().unwrap();

    let stdin = stdin();
    for key in stdin.keys() {
        let key = key.unwrap();
        match key {
            Key::Enter => {
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
                        write!(
                            stdout,
                            "\r\nFalsch! Die Antwort war: {}\r\n",
                            correct_answer
                        )
                        .unwrap();
                    }
                } else {
                    write!(stdout, "\r\nFehlerhafte Eingabe!\r\n").unwrap();
                }
                stdout.flush().unwrap();
                break;
            }
        }
        Key::Char(c) => {
            write!(stdout, "{}", c).unwrap();
            stdout.flush().unwrap();
            input.push(c);
        }
        Key::Backspace => {
            input.pop();
        }
    }
}
