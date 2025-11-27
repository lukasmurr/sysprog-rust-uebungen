// TODO [David]: Define CLI arguments using `clap`.
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

// TODO [David]: Implement `main` function.
// - Parse arguments.
// - Dispatch to appropriate functions in `archive`, `compression`, and `crypto` modules.
// - Handle errors gracefully.

fn main() {
    println!("Hello, world!");
}
