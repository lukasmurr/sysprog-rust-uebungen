use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

const DATFILE: &str = "telbuch.dat";

type TelBuch = HashMap<String, Vec<String>>;

// input_loop.rs ===
#[derive(Debug)]
struct Cmd {
    args: Vec<String>,
}

fn input(prompt: &[u8]) -> Cmd {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let mut cmd = String::new();

    loop {
        let e = handle.write_all(prompt);
        match e {
            Ok(_) => (),
            Err(err) => panic!("error {:?}", err.kind()),
        }

        let e = handle.flush();
        match e {
            Ok(_) => (),
            Err(err) => panic!("error {:?}", err.kind()),
        }

        cmd.clear();
        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read line");

        let v: Vec<&str> = cmd.trim().split(' ').collect();
        match v[0] {
            "" => continue, // just newline
            _ => {
                let mut v2: Vec<String> = vec![];
                for e in v {
                    v2.push(e.to_string());
                }
                return Cmd { args: v2 };
            }
        }
    }
}

fn save(telbuch: &TelBuch) {
    let mut file = File::create(DATFILE).expect("Could not create file");
    for (name, numbers) in telbuch.iter() {
        for number in numbers {
            writeln!(file, "{}|{}", name, number).unwrap();
        }
    }
}

fn load() -> TelBuch {
    let mut telbuch = TelBuch::new();
    if let Ok(file) = File::open(DATFILE) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() == 2 {
                    telbuch
                        .entry(parts[0].to_string())
                        .or_insert_with(Vec::new)
                        .push(parts[1].to_string());
                }
            }
        }
    }
    telbuch
}

fn help() {
    println!("usage:");
    println!("! name number   # make new entry");
    println!("? name          # ask for number");
    println!("save            # save to file");
    println!("load            # load from file");
    println!(".               # quit");
}

fn main() {
    let mut telbuch = TelBuch::new();
    
    println!("Enter 'h' or 'help' for usage.");
    
    loop {
        let cmd = input(b"> ");
        
        // quit with "."
        if cmd.args[0] == "." {
            break;
        }
        
        // help
        if cmd.args[0] == "help" || cmd.args[0] == "h" {
            help();
            continue;
        }
        
        // ! name number
        if cmd.args[0] == "!" {
            if cmd.args.len() >= 3 {
                let name = &cmd.args[1];
                let number = &cmd.args[2];
                println!("name: {}, number: {}", name, number);
                telbuch
                    .entry(name.clone())
                    .or_insert_with(Vec::new)
                    .push(number.clone());
            }
            continue;
        }
        
        // ? name
        if cmd.args[0] == "?" {
            if cmd.args.len() >= 2 {
                let name = &cmd.args[1];
                println!("name: {}", name);
                if let Some(numbers) = telbuch.get(name) {
                    for number in numbers {
                        println!("  {}", number);
                    }
                } else {
                    println!(" not found");
                }
            }
            continue;
        }
        
        // save
        if cmd.args[0] == "save" {
            save(&telbuch);
            println!("saved");
            continue;
        }
        
        // load
        if cmd.args[0] == "load" {
            telbuch = load();
            println!("loaded");
            continue;
        }
    }
}
