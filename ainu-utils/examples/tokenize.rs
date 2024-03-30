use ainu_utils::segmenter::segment;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let text = &args[1];

    let tokens = segment(text);

    println!("{:?}", tokens);
}
