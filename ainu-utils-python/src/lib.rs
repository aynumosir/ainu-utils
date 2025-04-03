extern crate ainu_utils as ainu_utils_rust;

use pyo3::prelude::*;

#[pyfunction]
fn tokenize(text: &str, keep_whitespace: bool) -> Vec<String> {
    ainu_utils_rust::tokenizer::tokenize(text, keep_whitespace)
}

#[pyfunction]
fn to_kana(text: &str) -> String {
    ainu_utils_rust::kana::to_kana(text)
}

#[pyfunction]
fn number_to_words(input: i32) -> String {
    ainu_utils_rust::numbers::parse(input).unwrap().to_string()
}

#[pymodule]
fn ainu_utils(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(tokenize, m)?)?;
    m.add_function(wrap_pyfunction!(to_kana, m)?)?;
    m.add_function(wrap_pyfunction!(number_to_words, m)?)?;
    m.add("test_number", 123)?;
    Ok(())
}
