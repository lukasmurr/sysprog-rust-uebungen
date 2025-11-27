use clap::Parser;
use std::process;

#[derive(Parser)]
struct Args {
    #[arg(short = 'e', long)]
    exit: Option<i32>,

    #[arg(short = 'm', long)]
    message: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(message) = args.message {
        println!("{}", message);
    }

    if let Some(code) = args.exit {
        process::exit(code);
    }
}
