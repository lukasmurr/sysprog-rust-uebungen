use clap::Parser;
use rand::Rng;
use serde::Deserialize;
use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

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

fn ueberweisung(konten: &Vec<Mutex<i32>>, von: usize, nach: usize, betrag: i32) {
    if von == nach {
        return;
    }

    // Deadlock prevention: Always lock the account with the lower index first.
    let (first_lock_idx, second_lock_idx) = if von < nach { (von, nach) } else { (nach, von) };

    let mut first_guard = konten[first_lock_idx].lock().unwrap();
    let mut second_guard = konten[second_lock_idx].lock().unwrap();

    // Now we have both locks, we can safely perform the transfer.
    // We need to be careful about which guard corresponds to 'von' and 'nach'.
    // Since we have mutable references to the data inside the mutexes, we can just modify them.
    // However, we need to know which value belongs to 'von' and which to 'nach'.

    // To make it cleaner, let's just modify the values directly via the guards.
    // But wait, first_guard is for first_lock_idx.

    let von_balance = if von == first_lock_idx {
        &mut *first_guard
    } else {
        &mut *second_guard
    };

    // We can't borrow from both guards simultaneously if we do it like above because of lifetimes/borrow checker potentially?
    // Actually, we can. But let's do the check logic first.

    // We need to check if 'von' has enough money.
    // But we can't easily get 'von_balance' and 'nach_balance' as mutable references at the same time
    // if we try to do it conditionally in a complex way.

    // Let's do it simply:
    if von == first_lock_idx {
        // first_guard is 'von', second_guard is 'nach'
        if *first_guard >= betrag {
            *first_guard -= betrag;
            *second_guard += betrag;
        }
    } else {
        // first_guard is 'nach', second_guard is 'von'
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

    for t_id in 0..config.threads {
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
    for (i, konto) in konten.iter().enumerate() {
        let saldo = *konto.lock().unwrap();
        // println!("Konto #{:03}: {:7}", i, saldo); // Optional: print all accounts
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
