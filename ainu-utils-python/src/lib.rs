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

#[pymodule]
fn ainu_utils(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(tokenize, m)?)?;
    m.add_function(wrap_pyfunction!(to_kana, m)?)?;
    m.add("test_number", 123)?;
    Ok(())
}
