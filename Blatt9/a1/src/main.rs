use clap::Parser;
use nix::sys::wait::wait;
use nix::unistd::{fork, getpid, ForkResult};
use rand::Rng;
use std::io::{self, Write};
use std::process::exit;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of children to spawn
    #[arg(value_name = "CHILDREN")]
    children: u32,
}

fn print_random_numbers(child_number: u32) {
    // Initialize the random number generator.
    // Since we are in a forked child process, and the parent (likely) hasn't initialized
    // thread_rng yet, this will seed from the OS entropy source for each child independently.
    // Thus, each child will generate a unique sequence of random numbers.
    let mut rng = rand::thread_rng();
    let pid = getpid();

    let r1: i32 = rng.gen();
    println!(
        "Child number {}: My PID is {} - my first random number is: {}",
        child_number, pid, r1
    );

    let r2: i32 = rng.gen();
    println!(
        "Child number {}: My PID is {} - my second random number is: {}",
        child_number, pid, r2
    );

    let r3: i32 = rng.gen();
    println!(
        "Child number {}: My PID is {} - my third random number is: {}",
        child_number, pid, r3
    );
}

fn main() {
    let args = Args::parse();
    let children = args.children;

    for i in 0..children {
        match unsafe { fork() } {
            Ok(ForkResult::Parent { .. }) => {
                // Parent process, continue to spawn next child
            }
            Ok(ForkResult::Child) => {
                // Child process
                print_random_numbers(i + 1);
                exit(0);
            }
            Err(_) => {
                eprintln!("Fork failed");
                exit(1);
            }
        }
    }

    // Wait for all children to finish
    for _ in 0..children {
        match wait() {
            Ok(_) => {}
            Err(e) => eprintln!("Error waiting for child: {}", e),
        }
    }

    print!("\n\nFinished. Press Enter to quit the program.");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
