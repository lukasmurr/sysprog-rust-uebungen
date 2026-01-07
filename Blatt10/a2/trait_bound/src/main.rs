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
            first,
            last,
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

// jetzt mit Trait Bound statt Trait-Objekt:
fn draw_text<T: Draw>(txt: &T) {
    // Größen sind für (c) egal, du kannst sie weglassen
    txt.draw();
}

fn main() {
    let greeting = Text::from("Hello World");
    let boxed_greeting = BoxedText::with_text_and_borders("Hello", '[', ']');

    draw_text(&greeting);
    print!(", ");
    draw_text(&boxed_greeting);
    println!();
}
