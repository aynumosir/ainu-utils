use ainu_utils::{kana, numbers, syllables, tokenizer};
use js_sys::Reflect;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn tokenize(text: &str, options: JsValue) -> Vec<String> {
    let keep_whitespace = Reflect::get(&options, &JsValue::from_str("keepWhitespace"))
        .ok()
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    tokenizer::tokenize(text, keep_whitespace)
}

#[wasm_bindgen(js_name = toKana)]
pub fn to_kana(text: &str) -> String {
    kana::to_kana(text)
}

#[wasm_bindgen(js_name = numberToWords)]
pub fn number_to_words(input: i32) -> String {
    numbers::parse(input).unwrap().to_string()
}

#[wasm_bindgen]
pub fn syllabicate(text: &str) -> Vec<String> {
    syllables::parse(text)
}
