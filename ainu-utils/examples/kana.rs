use ainu_utils::kana::{KanaConfig, transliterate};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let text = &args[1];

    let kana = transliterate(text, &KanaConfig::default());

    println!("{}", kana);
}
