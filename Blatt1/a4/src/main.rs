use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Verwendung: cargo run -- <zahl>");
        return;
    }

    let mut n: u64 = args[1].parse().expect("Keine gültige Zahl");

    println!("{}", n);

    while n != 1 {
        if n.is_multiple_of(2) {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        println!("{}", n);
    }
}
