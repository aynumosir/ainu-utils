use ainu_utils::tokenizer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn tokenize(text: &str) -> Vec<String> {
    tokenizer::tokenize(text)
}
