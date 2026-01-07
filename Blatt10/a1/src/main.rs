// a1/src/main.rs
trait Times {
    fn times<F>(self, f: F)
    where
        F: FnMut(usize);
}

impl Times for usize {
    fn times<F>(self, mut f: F)
    where
        F: FnMut(usize),
    {
        for i in 1..=self {
            f(i);
        }
    }
}

fn main() {
    5.times(|i| {
        println!("Ferris ate {} cookies", i);
    });
}
