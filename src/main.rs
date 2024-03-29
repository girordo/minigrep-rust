use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query: &String = &args[1];
    let filename: &String = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents: String =
        fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query: String = args[1].clone();
    let filename: String = args[2].clone();

    Config { query, filename }
}
