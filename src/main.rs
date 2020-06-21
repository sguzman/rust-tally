use std::fs;
use std::env;

pub fn chunk(str: String, n: u8) -> Vec<String> {
    let vecky: Vec<String> = vec![String::from("stub")];

    return vecky;
}

fn main() {
    for argument in env::args() {
        println!("Found {}", argument);
        let out: Vec<String> = chunk(argument, 2);
    }

    println!("Hello, world!");
}
