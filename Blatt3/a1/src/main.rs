use std::collections::HashMap;

mod input;

fn main() {
    let line = input::input(Some("Geben Sie Zahlen getrennt durch Leerzeichen ein: "));

    let numbers: Vec<i32> = line
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    if numbers.is_empty() {
        println!("Keine gültigen Zahlen eingegeben!");
        return;
    }

    let sum: i32 = numbers.iter().sum();
    let mean = sum as f64 / numbers.len() as f64;

    let mut sorted = numbers.clone();
    sorted.sort();
    let median = if sorted.len().is_multiple_of(2) {
        let mid = sorted.len() / 2;
        (sorted[mid - 1] + sorted[mid]) as f64 / 2.0
    } else {
        sorted[sorted.len() / 2] as f64
    };

    let mut frequency: HashMap<i32, usize> = HashMap::new();
    for &num in &numbers {
        *frequency.entry(num).or_insert(0) += 1;
    }

    let mode = frequency
        .iter()
        .max_by_key(|&(_, count)| count)
        .map(|(&num, _)| num)
        .unwrap();

    println!("\nErgebnisse:");
    println!("Mittelwert: {:.2}", mean);
    println!("Medianwert: {:.2}", median);
    println!("Häufigster Wert: {}", mode);
}
