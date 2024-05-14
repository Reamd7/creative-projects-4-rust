use std::collections::HashMap;

use lazy_static::lazy_static;


lazy_static! {
    static ref DICTIONARY: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(11, "info");
        m.insert(22, "bar");
        println!("DICTIONARY initialize");
        m
    };
}

pub fn main() {
    println!("Start lazy_static main");
    println!("DICTIONARY contains {:?}", *DICTIONARY);
    println!("DICTIONARY contains {:?}", *DICTIONARY);
}