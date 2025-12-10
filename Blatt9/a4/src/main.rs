use clap::Parser;
use rand::Rng;
use serde::Deserialize;
use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the configuration file
    #[arg(short = 'c', long, default_value = "config.toml")]
    config: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    num_konten: usize,
    threads: usize,
    transfers_pro_thread: usize,
}

fn ueberweisung(konten: &[Mutex<i32>], von: usize, nach: usize, betrag: i32) {
    if von == nach {
        return;
    }

    // Deadlock prevention: Always lock the account with the lower index first.
    let (first_lock_idx, second_lock_idx) = if von < nach { (von, nach) } else { (nach, von) };

    let mut first_guard = konten[first_lock_idx].lock().unwrap();
    let mut second_guard = konten[second_lock_idx].lock().unwrap();
    if von == first_lock_idx {
        if *first_guard >= betrag {
            *first_guard -= betrag * second_guard += betrag;
        }
    } else {
        if *second_guard >= betrag {
            *second_guard -= betrag;
            *first_guard += betrag;
        }
    }
}

fn main() {
    let args = Args::parse();

    // Read configuration
    let config_content = fs::read_to_string(&args.config).expect("Failed to read config file");
    let config: Config = toml::from_str(&config_content).expect("Failed to parse config file");

    println!("Configuration: {:?}", config);

    let mut start_geld = 0;
    let mut konten = Vec::with_capacity(config.num_konten);

    // Initialize accounts with random amounts
    let mut rng = rand::thread_rng();
    for _ in 0..config.num_konten {
        let value = rng.gen_range(0..100000);
        start_geld += value;
        konten.push(Mutex::new(value));
    }

    let konten = Arc::new(konten);
    let mut handles = vec![];

    println!("Total start amount: {}", start_geld);

    for _ in 0..config.threads {
        let konten_clone = Arc::clone(&konten);
        let transfers = config.transfers_pro_thread;
        let num_konten = config.num_konten;

        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            for _ in 0..transfers {
                let von = rng.gen_range(0..num_konten);
                let nach = rng.gen_range(0..num_konten);
                let betrag = rng.gen_range(0..100);

                ueberweisung(&konten_clone, von, nach, betrag);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut end_geld = 0;
    println!("---------------------");
    for (konto) in konten.iter().enumerate() {
        let saldo = *konto.lock().unwrap();
        end_geld += saldo;
    }

    println!("---------------------");
    println!("Summe am Ende:  {:16}", end_geld);
    println!("Summe zu Begin: {:16}", start_geld);

    if start_geld == end_geld {
        println!("SUCCESS: Sums match!");
    } else {
        println!("ERROR: Sums do not match!");
    }
}
