extern crate ainu_utils as ainu_utils_rust;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyfunction]
fn tokenize(text: &str, keep_whitespace: bool) -> Vec<String> {
    ainu_utils_rust::tokenizer::tokenize(text, keep_whitespace)
}

#[pyfunction]
fn to_kana(text: &str) -> String {
    ainu_utils_rust::kana::to_kana(text)
}

pub enum NumberToWordsError {
    NoNounError,
    InvalidFormError,
    CoreError(ainu_utils_rust::numbers::NumberToWordsError),
}

impl From<NumberToWordsError> for PyErr {
    fn from(value: NumberToWordsError) -> Self {
        match value {
            NumberToWordsError::NoNounError => PyValueError::new_err("No noun given"),
            NumberToWordsError::InvalidFormError => PyValueError::new_err("Invalid form given"),
            NumberToWordsError::CoreError(err) => match err {
                ainu_utils_rust::numbers::NumberToWordsError::OutOfRange => {
                    PyValueError::new_err("Number out of range")
                }
            },
        }
    }
}

#[pyfunction]
#[pyo3(signature = (input, form, noun=None))]
fn number_to_words(
    input: i32,
    form: String,
    noun: Option<String>,
) -> Result<String, NumberToWordsError> {
    let numeral_form = match form.as_str() {
        "human" => Ok(ainu_utils_rust::numbers::NumeralForm::HumanCount),
        "thing" => Ok(ainu_utils_rust::numbers::NumeralForm::ThingCount),
        "serial" => Ok(ainu_utils_rust::numbers::NumeralForm::Serial),
        "adnominal" => match noun {
            Some(noun) => Ok(ainu_utils_rust::numbers::NumeralForm::Adnominal(noun)),
            None => Err(NumberToWordsError::NoNounError),
        },
        _ => Err(NumberToWordsError::InvalidFormError),
    }?;

    let words = ainu_utils_rust::numbers::number_to_words(input, &numeral_form)
        .map_err(NumberToWordsError::CoreError)?;

    Ok(words)
}

#[pyfunction]
fn syllabicate(text: &str) -> Vec<String> {
    ainu_utils_rust::syllables::parse(text)
}

#[pymodule]
fn ainu_utils(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(tokenize, m)?)?;
    m.add_function(wrap_pyfunction!(to_kana, m)?)?;
    m.add_function(wrap_pyfunction!(number_to_words, m)?)?;
    m.add_function(wrap_pyfunction!(syllabicate, m)?)?;
    m.add("test_number", 123)?;
    Ok(())
}
