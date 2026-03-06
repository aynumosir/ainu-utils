use ainu_utils::numbers::{NumeralForm, parse};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let int = args[1].parse::<i32>().unwrap();
    let kind = match args[2].as_str() {
        "humans" => NumeralForm::HumanCount,
        "things" => NumeralForm::ThingCount,
        "serial" => NumeralForm::Serial,
        nom => NumeralForm::Adnominal(nom.to_string()),
    };
    let words = parse(int).unwrap();

    println!("{:?}", words.format(&kind));
}
