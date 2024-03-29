extern crate ainu_utils as ainu;

use pyo3::prelude::*;

#[pyfunction]
fn tokenize(text: &str) -> Vec<String> {
    ainu::tokenizer::tokenize(text)
}

#[pymodule]
fn ainu_utils(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(tokenize, m)?)?;
    Ok(())
}
