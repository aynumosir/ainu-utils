use ainu_utils::{kana, numbers, syllables, tokenizer};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn tokenize(text: &str, keep_whitespace: bool) -> Vec<String> {
    tokenizer::tokenize(text, keep_whitespace)
}

#[wasm_bindgen]
pub fn to_kana(text: &str) -> String {
    kana::to_kana(text)
}

#[wasm_bindgen]
pub fn number_to_words(input: i32) -> String {
    numbers::parse(input).unwrap().to_string()
}

#[wasm_bindgen]
pub fn syllabicate(text: &str) -> Vec<String> {
    syllables::parse(text)
}
