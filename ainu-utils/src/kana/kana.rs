use crate::normalizer::normalize;
use crate::phoneme::Phoneme;

use super::kana_linking::link;
use super::kana_map_c::map_c;
use super::kana_map_cv::map_cv;
use super::kana_map_punc::map_punc;
use super::kana_map_v::map_v;

pub fn to_kana(input: &str) -> String {
    let mut input: String = input.to_string();
    input = link(&input);

    let words: Vec<&str> = input.split(' ').collect();
    let mut output = String::new();

    for word in words {
        let kana = word_to_kana(word);

        if kana.chars().any(|c| c.is_ascii_alphabetic()) {
            output += word;
        } else {
            output += &kana;
        }

        output += "　";
    }

    output = output.trim_end().to_string();

    output
}

fn word_to_kana(input: &str) -> String {
    let mut input: String = input.to_string();

    input = normalize(&input);
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
