use anyhow::{Context, Result};
use clap::Parser;
use log::debug;
use minigrep::{search, search_case_insensitive};
use std::fs;
use std::io;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The string to search for
    query: String,

    /// The file path to search in
    file_path: String,

    /// Ignore case
    #[arg(short, long, env = "IGNORE_CASE")]
    ignore_case: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Output as JSON
    #[arg(long)]
    json: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut builder = env_logger::Builder::from_default_env();
    if args.verbose {
        builder.filter_level(log::LevelFilter::Debug);
    }
    builder.init();

    debug!("Starting minigrep with args: {:?}", args);

    run(args)
}

fn run(args: Args) -> Result<()> {
    let contents = fs::read_to_string(&args.file_path)
        .with_context(|| format!("could not read file `{}`", args.file_path))?;

    let results = if args.ignore_case {
        search_case_insensitive(&args.query, &contents)
    } else {
        search(&args.query, &contents)
    };

    if args.json {
        serde_json::to_writer(io::stdout(), &results)?;
        println!();
    } else {
        for line in results {
            println!("{line}");
        }
    }

    Ok(())
}
