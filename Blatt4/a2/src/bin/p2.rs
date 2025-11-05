use std::time::Duration;
use std::thread;
use std::io::{self, BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut delay = 0;
    let mut exit_code = 0;
    let mut echo = false;

    for i in 1..args.len() {
        match args[i].as_str() {
            "--help" => {
                println!("--help\n-e <c>, --exit <c>\n-d <s>, --delay <s>\n--echo");
                return;
            }
            "-e" | "--exit" => {
                if args.len() > i + 1 {
                    exit_code = args[i + 1].parse::<i32>().unwrap_or(0);
                }
            }
            "-d" | "--delay" => {
                if args.len() > i + 1 {
                    delay = args[i + 1].parse::<u64>().unwrap_or(0);
                }
            }
            "--echo" => {
                echo = true;
            }
            _ => {}
        }
    }

    if echo {
        let stdin = io::stdin();
        let reader = BufReader::new(stdin);
        for line in reader.lines() {
            let line = line.unwrap();
            println!("{}", line);
            if line.trim() == "." {
                break;
            }
        }
        return;
    }

    if delay > 0 {
        thread::sleep(Duration::from_secs(delay));
    }

    std::process::exit(exit_code);
}
