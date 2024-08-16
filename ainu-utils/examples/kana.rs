use ainu_utils::kana::to_kana;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let text = &args[1];

    let kana = to_kana(text);

    println!("{}", kana);
}
