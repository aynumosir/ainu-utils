use ainu_utils::segmenter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn segment(text: &str, keep_whitespace: bool) -> Vec<String> {
    segmenter::segment(text, keep_whitespace)
}
