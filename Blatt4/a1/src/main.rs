use rand::prelude::*;
use std::fs::read_to_string;

fn choose_line(filename: &str) -> String {
    let content = read_to_string(filename).expect("Datei konnte nicht gelesen werden.");
    let lines: Vec<_> = content.lines().collect();
    let mut rng = thread_rng();
    lines.choose(&mut rng).unwrap().to_string()
}

fn main() {
    println!("{}", choose_line("rust_statements.txt"));
}
