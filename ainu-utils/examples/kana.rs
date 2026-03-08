use ainu_utils::kana::transliterate_to_kana;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let text = &args[1];

    let kana = transliterate_to_kana(text, &Default::default());

    println!("{}", kana);
}
