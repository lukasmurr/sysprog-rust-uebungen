use std::io::Read;
use std::io::Write;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut bin_path = None;

    for i in 1..args.len() {
        if args[i] == "--bin" && args.len() > i + 1 {
            bin_path = Some(&args[i + 1]);
        }
    }

    let bin = bin_path.map_or("./p2".to_string(), |s| s.clone());

    // Test: Exit-Code
    let exit_code = 42;
    let output = Command::new(&bin)
        .arg("-e")
        .arg(exit_code.to_string())
        .output()
        .unwrap();
    println!("exit code ok: {}", output.status.code() == Some(exit_code));

    // Test: Delay
    let delay = 2;
    let start = Instant::now();
    let output = Command::new(&bin)
        .arg("-d")
        .arg(delay.to_string())
        .arg("-e")
        .arg("0")
        .output()
        .unwrap();
    let elapsed = start.elapsed().as_secs();
    println!(
        "delay ok: {}",
        elapsed >= delay as u64 && elapsed < delay as u64 + 1
    );

    // Test: Echo
    let (_tx, _rx): (mpsc::Sender<()>, mpsc::Receiver<()>) = mpsc::channel();
    let mut child = Command::new(bin)
        .arg("--echo")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdin = child.stdin.take().unwrap();
    thread::spawn(move || {
        stdin.write_all(b"Test\n.\n").unwrap();
        stdin.flush().unwrap();
    });

    let mut stdout = child.stdout.take().unwrap();
    let mut buffer = String::new();
    let _ = stdout.read_to_string(&mut buffer);
    let echo_ok = buffer.contains("Test") && buffer.contains(".");
    println!("echo ok: {}", echo_ok);

    child.wait().unwrap();
}
