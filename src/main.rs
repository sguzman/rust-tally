use std::fs;
use std::env;

pub fn chunk(str: String, n: usize) -> Vec<String> {
    let vecky: Vec<String> = vec![String::from("stub")];
    let partition: Vec<String> = {
        let mut partition: Vec<String> = Vec::new();
        let limit: usize = str.len() - n;

        for i in 0..limit {
            let part: String = str.chars().skip(i).take(n).collect();
            partition.push(part);
        }

        partition
    };

    partition
}

fn main() {
    for argument in env::args() {
        println!("Found {}", argument);
        let out: Vec<String> = chunk(argument, 3);
        for o in out {
            println!("\t{}", o);
        }
    }

    println!("Hello, world!");
}
