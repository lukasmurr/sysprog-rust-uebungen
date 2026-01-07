use std::collections::HashMap;

macro_rules! hash_map {
    ( $( $key:expr => $val:expr ),* $(,)? ) => {{
        let mut map = HashMap::new();
        $(
            map.insert($key, $val);
        )*
        map
    }};
}

fn main() {
    let ages = hash_map! { "Maria" => 26, "Peter" => 32 };
    println!("{:#?}", ages);
}
