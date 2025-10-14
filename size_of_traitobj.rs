// aus: Carlo Milanesi, Beginning Rust, Packt Publishing, 2. Auflage 2022.
// https://link-springer-com.ezproxy.hs-augsburg.de/book/10.1007/978-1-4842-7208-4
// Kapitel 20: Object-oriented Programming

use std::mem::size_of_val;

trait Draw {
    fn draw(&self);
}

struct Text {
    characters: String,
}

impl Text {
    fn from(text: &str) -> Text {
        Text {
            characters: text.to_string(),
        }
    }
}

impl Draw for Text {
    fn draw(&self) {
        print!("{}", self.characters);
    }
}

struct BoxedText {
    text: Text,
    first: char,
    last: char,
}

impl BoxedText {
    fn with_text_and_borders(text: &str, first: char, last: char) -> BoxedText {
        BoxedText {
            text: Text::from(text),
            first: first,
            last: last,
        }
    }
}

impl Draw for BoxedText {
    fn draw(&self) {
        print!("{}", self.first);
        self.text.draw();
        print!("{}", self.last);
    }
}

fn main() {
    let greeting = Text::from("Hello World");
    let boxed_greeting = BoxedText::with_text_and_borders("Hello", '[', ']');

    fn draw_text(txt: &dyn Draw) {
        println!("A: {}", size_of_val(txt)); 
        println!("B: {}", size_of_val(&txt)); 
        println!("C: {}", size_of_val(&&txt)); 
        txt.draw();
    }

    draw_text(&greeting);
    print!(", ");
    draw_text(&boxed_greeting);
    println!();
    println!("D: {}", size_of_val(&greeting)); 
    println!("E: {}", size_of_val(&&greeting)); 
    println!("F: {}", size_of_val(&&&greeting)); 
    println!("G: {}", size_of_val(&boxed_greeting)); 
    println!("H: {}", size_of_val(&&boxed_greeting)); 
    println!("I: {}", size_of_val(&&&boxed_greeting)); 
}

