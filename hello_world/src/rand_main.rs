use rand::prelude::*;

pub fn rand_main() {
    let mut rng = thread_rng();

    println!("0..=120 {}", rng.gen_range(0..=120));
    println!("F64 {}", rng.gen::<f64>());
    println!("{}", if rng.gen() { "Heads" } else {  "Tails" });
    println!("hello");
}