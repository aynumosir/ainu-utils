extern crate ainu_utils as ainu_utils_rust;

use pyo3::prelude::*;

#[pyfunction]
fn segment(text: &str) -> Vec<String> {
    ainu_utils_rust::segmenter::segment(text)
}

#[pymodule]
fn ainu_utils(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(segment, m)?)?;
    m.add("test_number", 123)?;
    Ok(())
}
