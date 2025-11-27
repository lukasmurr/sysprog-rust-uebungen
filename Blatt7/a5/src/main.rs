use rand::Rng;

struct Passwords {
    length: usize,
}

impl Passwords {
    fn new() -> Self {
        Passwords { length: 10 }
    }

    fn with_length(length: usize) -> Self {
        Passwords { length }
    }
}

impl IntoIterator for Passwords {
    type Item = String;
    type IntoIter = PasswordsIterator;

    fn into_iter(self) -> Self::IntoIter {
        PasswordsIterator {
            length: self.length,
        }
    }
}

struct PasswordsIterator {
    length: usize,
}

impl Iterator for PasswordsIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut rng = rand::thread_rng();
        let password: String = (0..self.length).map(|_| rng.gen_range('a'..='z')).collect();
        Some(password)
    }
}

fn main() {
    // Drei Passwoerter der Laenge 10 (default) ausgeben
    for p in Passwords::new().into_iter().take(3) {
        println!("{}", p);
    }

    // Drei Passwoerter der Laenge 5 ausgeben
    Passwords::with_length(5)
        .into_iter()
        .take(3)
        .for_each(|p| println!("{}", p));
}
