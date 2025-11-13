use rand::Rng;

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Debug)]
enum Multitype {
    Integer(i32),
    Float(f32),
    Text(String),
    Custom(Person),
}

fn read_value(v: &mut Multitype) {
    let mut rng = rand::thread_rng();
    *v = match rng.gen_range(0..4) {
        0 => Multitype::Integer(rng.gen_range(-100..100)),
        1 => Multitype::Float(rng.gen_range(-100.0..100.0)),
        2 => Multitype::Text(format!("Text_{}", rng.gen_range(1..100))),
        _ => Multitype::Custom(Person {
            name: format!("Person_{}", rng.gen_range(1..100)),
            age: rng.gen_range(18..80),
        }),
    };
}

fn main() {
    let mut value = Multitype::Integer(0);

    for i in 1..=10 {
        read_value(&mut value);
        print!("Durchlauf {}: ", i);
        match &value {
            Multitype::Integer(x) => println!("Integer: {}", x),
            Multitype::Float(x) => println!("Float: {}", x),
            Multitype::Text(x) => println!("String: {}", x),
            Multitype::Custom(x) => println!("Person: {} ({} Jahre)", x.name, x.age),
        }
    }
}
