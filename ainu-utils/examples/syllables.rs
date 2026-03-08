use ainu_utils::syllables::syllabicate;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let text = &args[1];

    let syllables = syllabicate(text);

    println!("{:?}", syllables);
}
