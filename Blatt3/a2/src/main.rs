use rand::Rng;
use std::io::{Write, stdin, stdout};
use std::time::{Duration, Instant};
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
    let stdin = stdin();
    if stdin.keys().next().is_some() {
        let reaction = start_time.elapsed();
        write!(
            stdout,
            "Reaktionszeit: {:.3} Sekunden\r\n",
            reaction.as_secs_f64()
        )
        .unwrap();
        stdout.flush().unwrap();
    }
}
