use ainu_utils::segmenter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn segment(text: &str) -> Vec<String> {
    segmenter::segment(text)
}
