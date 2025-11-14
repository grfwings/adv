// This is free and unencumbered software released into the public domain.
// Author: Griffin Evans <griffinevans@protonmail.com>
use rand::Rng;

fn main() {

    let bytes = include_bytes!("advice");
    let contents = String::from_utf8_lossy(bytes);
    
    let advice: Vec<&str> = contents.trim().split("\n%\n").collect();

    let rng: usize = rand::rng().random_range(0..advice.len());

    println!("{}", advice[rng]);
}
