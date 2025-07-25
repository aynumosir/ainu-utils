use unicode_normalization::char::is_combining_mark;
use unicode_normalization::UnicodeNormalization;

pub fn strip_accents(input: &str) -> String {
    input.nfkd().filter(|c| !is_combining_mark(*c)).collect()
}

pub fn normalize(input: &str) -> String {
    let mut result: String;

    result = input.to_lowercase();
    result = strip_accents(result.as_str()).to_string();

    result
}
