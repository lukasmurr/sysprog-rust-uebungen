// hhoegl, 2021-10-14

use std::io;
use std::io::Write; // write_all(), flush()

#[derive(Debug)]
struct Cmd {
    args: Vec<String>,
}

fn input(prompt: &[u8]) -> Cmd {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let mut cmd = String::new();
    loop {
        let e = handle.write_all(prompt); // -> Result<(), std::io::Error>
        match e {
            Ok(_) => (),
            Err(err) => panic!("error {:?}", err.kind()),
        }
        let e = handle.flush(); // -> Result<(), std::io::Error>
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

fn main() {
    let mut c: Cmd;
    loop {
        c = input(b"Prompt> "); // byte string &[u8]
        println!("c = {:?}", c);
        if c.args[0] == "." {
            break;
        }
    }
}
