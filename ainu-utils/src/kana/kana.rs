use crate::phoneme::Phoneme;
use regex::Regex;
use unicode_normalization::UnicodeNormalization;
use unicode_normalization::char::is_combining_mark;

use super::kana_linking::link;
use super::kana_map_c::map_c;
use super::kana_map_cv::map_cv;
use super::kana_map_punc::map_punc;
use super::kana_map_v::map_v;

pub enum Whitespace {
    Fullwidth,
    Halfwidth,
}

impl ToString for Whitespace {
    fn to_string(&self) -> String {
        match self {
            Self::Fullwidth => "　".to_string(),
            Self::Halfwidth => " ".to_string(),
        }
    }
}

impl Default for Whitespace {
    fn default() -> Self {
        Self::Fullwidth
    }
}

#[derive(Debug)]
pub enum IgnorePatternError {
    InvalidPattern,
}

pub struct IgnorePattern(Regex);

impl IgnorePattern {
    pub fn new(value: &str) -> Result<Self, IgnorePatternError> {
        let regex = Regex::new(value).map_err(|_| IgnorePatternError::InvalidPattern)?;
        Ok(IgnorePattern(regex))
    }
}

#[derive(Default)]
pub struct TransliterateToKanaOptions {
    pub whitespace: Whitespace,
    pub ignore_pattern: Option<IgnorePattern>,
}

#[derive(Debug)]
pub enum TransliterateToKanaError {
    InvalidIgnore,
}

pub fn transliterate_to_kana(input: &str, options: &TransliterateToKanaOptions) -> String {
    let mut input: String = input.to_string();
    input = link(&input);

    let words_latn: Vec<&str> = input.split(' ').collect();
    let mut words_kana: Vec<String> = vec![];

    for word_latn in words_latn {
        if let Some(ignore) = &options.ignore_pattern
            && ignore.0.is_match(word_latn)
        {
            words_kana.push(word_latn.to_string());
            continue;
        }

        let word_kana = transliterate_word_to_kana(word_latn);
        words_kana.push(word_kana);
    }

    let output = words_kana.join(&options.whitespace.to_string()).to_string();

    output
}

fn strip_accents(input: &str) -> String {
    input.nfkd().filter(|c| !is_combining_mark(*c)).collect()
}

fn transliterate_word_to_kana(input: &str) -> String {
    let mut input: String = input.to_string();

    input = input.to_lowercase();
    input = strip_accents(&input);
    input = map_punc(&input);

    let chars: Vec<char> = input.chars().collect();

    let mut output = String::new();
    let mut index = 0;

    while index < chars.len() {
        let prev = chars.get(index.wrapping_sub(1));
        let current = chars.get(index).unwrap();
        let next = chars.get(index + 1);

        if current.is_vowel() {
            output.push(map_v(current));
            index += 1;
        } else if current.is_consonant() {
            if next.is_some_and(|next| next == current) && !matches!(&current, 'n' | 'y' | 'w') {
                output.push_str(&map_c(&'t', None));
                index += 1;
            } else if let Some(next) = next
                && next.is_vowel()
            {
                output.push_str(&map_cv(current, next));
                index += 2;
            } else {
                output.push_str(&map_c(current, prev));
                index += 1;
            }
        } else {
            if &'\'' == current {
                index += 1;
            } else {
                output.push(*current);
                index += 1;
            }
        }
    }

    output
}
