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

pub enum NumberToWordsError {
    NoNounError,
    InvalidFormError,
    CoreError(numbers::NumberToWordsError),
}

impl From<NumberToWordsError> for JsValue {
    fn from(value: NumberToWordsError) -> Self {
        match value {
            NumberToWordsError::NoNounError => JsValue::from("No noun given"),
            NumberToWordsError::InvalidFormError => JsValue::from("Invalid form given"),
            NumberToWordsError::CoreError(err) => match err {
                numbers::NumberToWordsError::OutOfRange => JsValue::from("Number out of range"),
            },
        }
    }
}

#[wasm_bindgen]
pub fn number_to_words(input: i32, form: String, noun: Option<String>) -> Result<String, JsValue> {
    let numeral_form = match form.as_str() {
        "human" => Ok(numbers::NumeralForm::HumanCount),
        "thing" => Ok(numbers::NumeralForm::ThingCount),
        "serial" => Ok(numbers::NumeralForm::Serial),
        "adnominal" => match noun {
            Some(noun) => Ok(numbers::NumeralForm::Adnominal(noun)),
            None => Err(NumberToWordsError::NoNounError),
        },
        _ => Err(NumberToWordsError::InvalidFormError),
    }?;

    let words =
        numbers::number_to_words(input, &numeral_form).map_err(NumberToWordsError::CoreError)?;

    Ok(words)
}

#[wasm_bindgen]
pub fn syllabicate(text: &str) -> Vec<String> {
    syllables::parse(text)
}
