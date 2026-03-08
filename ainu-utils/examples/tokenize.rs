use ainu_utils::tokens::tokenize;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let text = &args[1];

    let tokens = tokenize(text, &Default::default());

    println!("{:?}", tokens);
}
