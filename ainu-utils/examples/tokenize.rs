use ainu_utils::tokenizer::tokenize;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let text = &args[1];

    let tokens = tokenize(text);

    println!("{:?}", tokens);
}
