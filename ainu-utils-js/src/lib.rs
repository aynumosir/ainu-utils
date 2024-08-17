use ainu_utils::{kana, tokenizer};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn tokenize(text: &str, keep_whitespace: bool) -> Vec<String> {
    tokenizer::tokenize(text, keep_whitespace)
}

#[wasm_bindgen]
pub fn to_kana(text: &str) -> String {
    kana::to_kana(text)
}
