struct Collatz(u64);

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }

        let current = self.0;

        if current == 1 {
            self.0 = 0;
        } else if current.is_multiple_of(2) {
            self.0 = current / 2;
        } else {
            self.0 = 3 * current + 1;
        }

        Some(current)
    }
}

fn main() {
    println!("{:?}", Collatz(131).collect::<Vec<_>>());
}
