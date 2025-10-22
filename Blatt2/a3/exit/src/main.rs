use std::env;
use std::process;

fn print_help() {
    println!("Usage: exit [OPTIONS] [EXIT_CODE]");
    println!();
    println!("Set the exit code of the program.");
    println!();
    println!("Arguments:");
    println!("  [EXIT_CODE]              Exit code as number (0-255)");
    println!();
    println!("Options:");
    println!("  -e, --exit-code <CODE>   Exit code as number (0-255)");
    println!("  -h, --help               Print this help message");
    println!();
    println!("Examples:");
    println!("  exit 2                   Exit with code 2");
    println!("  exit --exit-code 5       Exit with code 5");
    println!("  exit -e 42               Exit with code 42");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        process::exit(5);
    }

    if args[1] == "-h" || args[1] == "--help" {
        print_help();
        process::exit(0);
    }

    if args[1] == "--exit-code" || args[1] == "-e" {
        if args.len() < 3 {
            eprintln!("Error: --exit-code requires an argument");
            eprintln!("Try 'exit --help' for more information.");
            process::exit(1);
        }

        match args[2].parse::<i32>() {
            Ok(code) => {
                if !(0..=255).contains(&code) {
                    eprintln!("Error: exit code must be between 0 and 255");
                    process::exit(1);
                }
                process::exit(code);
            }
            Err(_) => {
                eprintln!("Error: '{}' is not a valid number", args[2]);
                process::exit(1);
            }
        }
    }

    match args[1].parse::<i32>() {
        Ok(code) => {
            if !(0..=255).contains(&code) {
                eprintln!("Error: exit code must be between 0 and 255");
                process::exit(1);
            }
            process::exit(code);
        }
        Err(_) => {
            eprintln!("Error: '{}' is not a valid number", args[1]);
            eprintln!("Try 'exit --help' for more information.");
            process::exit(1);
        }
    }
}
