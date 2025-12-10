use clap::Parser;
use rand::Rng;
use std::thread;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of threads to spawn
    #[arg(value_name = "THREADS")]
    threads: usize,
}

fn print_random_numbers(thread_number: usize) {
    // In Rust, thread_rng() gives a thread-local random number generator
    // that is automatically seeded from the OS.
    // This ensures each thread has its own independent sequence of random numbers.
    let mut rng = rand::thread_rng();

    let r1: i32 = rng.gen();
    println!(
        "Thread number {}: my first random number is: {}",
        thread_number, r1
    );

    let r2: i32 = rng.gen();
    println!(
        "Thread number {}: my second random number is: {}",
        thread_number, r2
    );

    let r3: i32 = rng.gen();
    println!(
        "Thread number {}: my third random number is: {}",
        thread_number, r3
    );
}

fn main() {
    let args = Args::parse();
    let num_threads = args.threads;

    println!(
        "Main thread: Starting {} threads to calculate many different random numbers!",
        num_threads
    );

    let mut handles = vec![];

    for i in 0..num_threads {
        // Spawn a new thread.
        // The `move` keyword moves the captured variable `i` into the closure,
        // so each thread gets its own copy of the loop index.
        let handle = thread::spawn(move || {
            print_random_numbers(i + 1);
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
}
