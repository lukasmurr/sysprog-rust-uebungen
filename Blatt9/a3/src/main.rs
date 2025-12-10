use clap::Parser;
use rand::Rng;
use std::sync::mpsc;
use std::thread;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of threads to spawn
    #[arg(value_name = "THREADS")]
    threads: usize,
}

struct RandomNumberMsg {
    thread_id: usize,
    iteration: usize, // 1, 2, or 3
    number: i32,
}

fn generate_random_numbers(thread_number: usize, tx: mpsc::Sender<RandomNumberMsg>) {
    let mut rng = rand::thread_rng();

    for i in 1..=3 {
        let num: i32 = rng.gen();
        let msg = RandomNumberMsg {
            thread_id: thread_number,
            iteration: i,
            number: num,
        };
        // Send the message to the main thread.
        // Unwrap is safe here because we know the receiver is alive in main.
        tx.send(msg).unwrap();
    }
}

fn main() {
    let args = Args::parse();
    let num_threads = args.threads;

    println!(
        "Main thread: Starting {} threads to calculate many different random numbers!",
        num_threads
    );

    // Create a channel for communication
    let (tx, rx) = mpsc::channel();

    let mut handles = vec![];

    for i in 0..num_threads {
        // Clone the sender for each thread
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            generate_random_numbers(i + 1, tx_clone);
        });
        handles.push(handle);
    }

    // Drop the original sender in the main thread.
    // This is crucial! If we don't drop it, the receiver will keep waiting forever
    // because there's still one open sender (this one).
    drop(tx);

    // Process messages as they arrive
    for msg in rx {
        let ordinal = match msg.iteration {
            1 => "first",
            2 => "second",
            3 => "third",
            _ => "nth",
        };
        println!(
            "Thread number {}: my {} random number is: {}",
            msg.thread_id, ordinal, msg.number
        );
    }

    // Wait for all threads to complete (good practice, though they should be done if rx is closed)
    for handle in handles {
        handle.join().unwrap();
    }
}
