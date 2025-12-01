mod archive;
mod compression;
mod crypto;
mod error;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

// [David]: Define CLI arguments using `clap`.
// Structure:
// securezip [COMMAND] [ARGS]
// Commands:
// - compress:
//   - files: Vec<PathBuf> (required)
//   - --output: PathBuf (optional, default to archive.zip)
//   - --encrypt: bool (optional)
//   - --password: String (optional, prompt if missing and encrypt is true)
// - decompress:
//   - file: PathBuf (required)
//   - --output: PathBuf (optional, default to current dir)
//   - --password: String (optional, prompt if missing and file is encrypted)

/// Prompts for password input from stdin
fn prompt_password(prompt: &str) -> Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    Ok(password.trim().to_string())
}

#[derive(Parser)]
#[command(name = "securezip")]
#[command(about = "A secure archive tool with compression and encryption", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compress files into an archive
    Compress {
        /// Files to compress
        #[arg(required = true)]
        files: Vec<PathBuf>,

        /// Output file path
        #[arg(short, long, default_value = "archive.tar")]
        output: PathBuf,

        /// Encrypt the archive
        #[arg(short, long, default_value_t = false)]
        encrypt: bool,

        /// Password for encryption (prompted if not provided and encrypt is true)
        #[arg(short, long)]
        password: Option<String>,
    },
    /// Decompress an archive
    Decompress {
        /// Archive file to decompress
        #[arg(required = true)]
        file: PathBuf,

        /// Output directory
        #[arg(short, long, default_value = ".")]
        output: PathBuf,

        /// Password for decryption (prompted if file is encrypted)
        #[arg(short, long)]
        password: Option<String>,
    },
}

// [David]: Implement `main` function.
// - Parse arguments.
// - Dispatch to appropriate functions in `archive`, `compression`, and `crypto` modules.
// - Handle errors gracefully.

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compress {
            files,
            output,
            encrypt,
            password,
        } => {
            // Create tar archive
            let archive_data = archive::create_archive(files)?;

            // Compress the archive
            let compressed_data = compression::compress(&archive_data)?;

            // Optionally encrypt
            let final_data = if encrypt {
                let pwd = match password {
                    Some(p) => p,
                    None => prompt_password("Enter password: ")?,
                };
                crypto::encrypt(&compressed_data, &pwd)?
            } else {
                compressed_data
            };

            // Write to output file
            fs::write(&output, final_data)?;
            println!("Archive created: {}", output.display());
        }
        Commands::Decompress {
            file,
            output,
            password,
        } => {
            // Read archive file
            let data = fs::read(&file)?;

            // Try to decrypt if password provided or prompt if decryption fails
            let decrypted_data = match password {
                Some(pwd) => crypto::decrypt(&data, &pwd)?,
                None => {
                    // Try without decryption first, if it fails, prompt for password
                    match compression::decompress(&data) {
                        Ok(decompressed) => {
                            archive::extract_archive(&decompressed, output.clone())?;
                            println!("Archive extracted to: {}", output.display());
                            return Ok(());
                        }
                        Err(_) => {
                            // Likely encrypted, prompt for password
                            let pwd = prompt_password("Enter password: ")?;
                            crypto::decrypt(&data, &pwd)?
                        }
                    }
                }
            };

            // Decompress
            let decompressed_data = compression::decompress(&decrypted_data)?;

            // Extract archive
            archive::extract_archive(&decompressed_data, output.clone())?;
            println!("Archive extracted to: {}", output.display());
        }
    }

    Ok(())
}
