use ferris_says::say;
use std::io::{BufWriter, stdout};

fn main() {
    let stdout = stdout();
    let message = String::from("Hallo ich bin ferris!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}

// __________________________
// < Hello fellow Rustaceans! >
//  --------------------------
//         \
//          \
//             _~^~^~_
//         \) /  o o  \ (/
//           '_   -   _'
//           / '-----' \
