use ainu_utils::{
    kana, numbers, syllables,
    tokens::{self, TokenizeOptions},
};
use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TokenizeOptionsJs {
    keep_whitespace: Option<bool>,
}

impl From<TokenizeOptionsJs> for TokenizeOptions {
    fn from(value: TokenizeOptionsJs) -> Self {
        let defaults = TokenizeOptions::default();
        Self {
            keep_whitespace: value.keep_whitespace.unwrap_or(defaults.keep_whitespace),
        }
    }
}

#[wasm_bindgen]
pub fn tokenize(text: &str, options: Option<JsValue>) -> Result<Vec<String>, JsError> {
    let tokenize_options: TokenizeOptions = options
        .map(serde_wasm_bindgen::from_value::<TokenizeOptionsJs>)
        .transpose()?
        .map(Into::into)
        .unwrap_or_default();
    Ok(tokens::tokenize(text, &tokenize_options))
}

#[wasm_bindgen]
pub fn syllabicate(text: &str) -> Vec<String> {
    syllables::syllabicate(text)
}

#[wasm_bindgen(js_name = transliterateToKana)]
pub fn transliterate_to_kana(text: &str) -> String {
    kana::transliterate_to_kana(text)
}

#[wasm_bindgen(js_name = numberToWords)]
pub fn number_to_words(input: i32) -> String {
    numbers::parse(input).unwrap().to_string()
}
