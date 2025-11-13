use std::fmt;

#[derive(Debug, PartialEq)]
struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        Clock {
            minutes: Self::normalize(total_minutes),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            minutes: Self::normalize(self.minutes + minutes),
        }
    }

    fn normalize(minutes: i32) -> i32 {
        let minutes_per_day = 24 * 60;
        ((minutes % minutes_per_day) + minutes_per_day) % minutes_per_day
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hours = self.minutes / 60;
        let minutes = self.minutes % 60;
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}

fn main() {
    // Beispiele
    let clock = Clock::new(8, 0);
    println!("Start: {}", clock);

    let clock = clock.add_minutes(63);
    println!("Nach +63 Minuten: {}", clock);

    let clock = clock.add_minutes(-30);
    println!("Nach -30 Minuten: {}", clock);

    // Über Mitternacht
    let clock = Clock::new(23, 59).add_minutes(2);
    println!("23:59 + 2 Minuten: {}", clock);

    // Negative Zeiten
    let clock = Clock::new(0, 0).add_minutes(-40);
    println!("00:00 - 40 Minuten: {}", clock);

    // Gleichheit testen
    let clock1 = Clock::new(10, 0);
    let clock2 = Clock::new(9, 60);
    println!("\n10:00 == 9:60? {}", clock1 == clock2);
}
