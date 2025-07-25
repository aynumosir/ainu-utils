use ainu_utils::syllables::parse;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let text = &args[1];

    let syllables = parse(text);

    println!("{:?}", syllables);
}
