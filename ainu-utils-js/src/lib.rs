use ainu_utils::{
    kana::{self, IgnorePattern, IgnorePatternError, TransliterateToKanaOptions, Whitespace},
    numbers, syllables,
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

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransliterateToKanaOptionsJs {
    whitespace: Option<String>,
    ignore_pattern: Option<String>,
}

impl TryFrom<TransliterateToKanaOptionsJs> for TransliterateToKanaOptions {
    type Error = JsError;

    fn try_from(value: TransliterateToKanaOptionsJs) -> Result<Self, Self::Error> {
        let defaults = TransliterateToKanaOptions::default();

        let whitespace = match value.whitespace {
            Some(whitespace) => match whitespace.as_str() {
                "fullwidth" => Ok(Some(Whitespace::Fullwidth)),
                "halfwidth" => Ok(Some(Whitespace::Halfwidth)),
                _ => Err(JsError::new(&format!("Invalid whitespace: {}", whitespace))),
            },
            _ => Ok(None),
        }?;

        let ignore_pattern = value
            .ignore_pattern
            .map(|ignore_pattern| IgnorePattern::new(&ignore_pattern))
            .transpose()
            .map_err(|err| match err {
                IgnorePatternError::InvalidPattern => JsError::new("Invalid pattern provided"),
            })?;

        let value = Self {
            whitespace: whitespace.unwrap_or(defaults.whitespace),
            ignore_pattern,
        };

        Ok(value)
    }
}

#[wasm_bindgen(js_name = transliterateToKana)]
pub fn transliterate_to_kana(text: &str, options: Option<JsValue>) -> Result<String, JsError> {
    let transliterate_to_kana_options = options
        .map(serde_wasm_bindgen::from_value::<TransliterateToKanaOptionsJs>)
        .transpose()?
        .map(TryInto::<TransliterateToKanaOptions>::try_into)
        .transpose()?
        .unwrap_or_default();

    Ok(kana::transliterate_to_kana(
        text,
        &transliterate_to_kana_options,
    ))
}

#[wasm_bindgen(js_name = numberToWords)]
pub fn number_to_words(input: i32) -> String {
    numbers::parse(input).unwrap().to_string()
}
