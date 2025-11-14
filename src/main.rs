// This is free and unencumbered software released into the public domain.
// Author: Griffin Evans <griffinevans@protonmail.com>
mod advice;

use advice::ADVICE;


fn main() {
    let rng: usize = fastrand::usize(..ADVICE.len());

    println!("{}", ADVICE[rng]);
}
