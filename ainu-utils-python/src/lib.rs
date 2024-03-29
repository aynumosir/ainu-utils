use ainu_utils::tokenizer;
use pyo3::prelude::*;

#[pyfunction]
fn tokenize(text: &str) -> Vec<String> {
    tokenizer::tokenize(text)
}

#[pymodule]
fn ainu_utils_python(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(tokenize, m)?)?;
    Ok(())
}
