extern crate ainu_utils as ainu_utils_rust;

use ainu_utils_rust::tokens::TokenizeOptions;
use pyo3::prelude::*;

#[pyfunction]
#[pyo3(signature = (text, *, keep_whitespace = None))]
fn tokenize(text: &str, keep_whitespace: Option<bool>) -> Vec<String> {
    let tokenize_options_default = TokenizeOptions::default();
    let tokenize_options = TokenizeOptions {
        keep_whitespace: keep_whitespace.unwrap_or(tokenize_options_default.keep_whitespace),
    };
    ainu_utils_rust::tokens::tokenize(text, &tokenize_options)
}

#[pyfunction]
fn transliterate_to_kana(text: &str) -> String {
    ainu_utils_rust::kana::transliterate_to_kana(text)
}

#[pyfunction]
fn number_to_words(input: i32) -> String {
    ainu_utils_rust::numbers::parse(input).unwrap().to_string()
}

#[pyfunction]
fn syllabicate(text: &str) -> Vec<String> {
    ainu_utils_rust::syllables::syllabicate(text)
}

#[pymodule]
fn ainu_utils(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(tokenize, m)?)?;
    m.add_function(wrap_pyfunction!(transliterate_to_kana, m)?)?;
    m.add_function(wrap_pyfunction!(number_to_words, m)?)?;
    m.add_function(wrap_pyfunction!(syllabicate, m)?)?;
    m.add("test_number", 123)?;
    Ok(())
}
