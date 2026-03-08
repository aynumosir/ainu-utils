extern crate ainu_utils as ainu_utils_rust;

use ainu_utils_rust::{
    kana::{IgnorePattern, IgnorePatternError, TransliterateToKanaOptions},
    tokens::TokenizeOptions,
};
use pyo3::{exceptions::PyValueError, prelude::*};

#[pyfunction]
#[pyo3(signature = (text, *, keep_whitespace = None))]
fn tokenize(text: &str, keep_whitespace: Option<bool>) -> Vec<String> {
    let tokenize_options_default = TokenizeOptions::default();
    let tokenize_options = TokenizeOptions {
        keep_whitespace: keep_whitespace.unwrap_or(tokenize_options_default.keep_whitespace),
    };
    ainu_utils_rust::tokens::tokenize(text, &tokenize_options)
}

#[pyclass(eq, from_py_object)]
#[derive(PartialEq, Clone)]
pub enum Whitespace {
    Fullwidth,
    Halfwidth,
}

#[pyfunction]
#[pyo3(signature = (text, *, whitespace = None, ignore_pattern = None))]
fn transliterate_to_kana(
    text: &str,
    whitespace: Option<Whitespace>,
    ignore_pattern: Option<&str>,
) -> Result<String, PyErr> {
    let defaults = TransliterateToKanaOptions::default();

    let whitespace = match whitespace {
        Some(Whitespace::Fullwidth) => ainu_utils_rust::kana::Whitespace::Fullwidth,
        Some(Whitespace::Halfwidth) => ainu_utils_rust::kana::Whitespace::Halfwidth,
        None => defaults.whitespace,
    };

    let ignore_pattern = ignore_pattern
        .map(|p| IgnorePattern::new(p))
        .transpose()
        .map_err(|e| match e {
            IgnorePatternError::InvalidPattern => PyValueError::new_err("Invalid pattern proivded"),
        })?;

    let options = TransliterateToKanaOptions {
        ignore_pattern,
        whitespace,
    };

    Ok(ainu_utils_rust::kana::transliterate_to_kana(text, &options))
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
    m.add_class::<Whitespace>()?;
    m.add("test_number", 123)?;
    Ok(())
}
