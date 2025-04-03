use ainu_utils::numbers::parse;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let int = args[1].parse::<i32>().unwrap();
    let words = parse(int).unwrap();

    println!("{:?}", words.to_string());
}
