use std::fs;
use std::env;

pub fn chunk(str: String, n: usize, func: fn(String)){
    let limit: usize = str.len() - n;

    for i in 0..limit {
        let part: String = str.chars().skip(i).take(n).collect();
       func(part);
    }
}

pub fn get_corpus(args: Vec<String>) -> Vec<String> {
    let corpus: Vec<String> = {
        let mut corpus: Vec<String> = Vec::new();
        for argument in args {
            let stream: String = fs::read_to_string(&argument).unwrap().parse().unwrap();
            println!("Found {} {}", &argument, stream.len());

            corpus.push(stream);
        }

        corpus
    };

    corpus
}

fn main() {
    let corpus = get_corpus(env::args().skip(1).collect());
    let size: usize = corpus.iter().map(|s| s.len()).sum();
    println!("Total size: {}", size);

    println!("Hello, world!");
}
