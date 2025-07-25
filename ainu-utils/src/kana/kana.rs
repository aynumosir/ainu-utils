use crate::phonology::{is_consonant, is_vowel};

use super::linking::link;
use super::maps::{TABLE_1, TABLE_2};
use super::symbols::map_symbols;
use unicode_normalization::char::is_combining_mark;
use unicode_normalization::UnicodeNormalization;

fn strip_accents(input: &str) -> String {
    input.nfkd().filter(|c| !is_combining_mark(*c)).collect()
}

fn normalize(mut input: String) -> String {
    input = input.to_lowercase();
    input = strip_accents(input.as_str()).to_string();
    input
}

pub fn to_kana(input: &str) -> String {
    let mut input = input.to_string();

    input = normalize(input);
    input = link(input);
    input = map_symbols(input);

    let chars: Vec<char> = input.chars().collect();

    let mut kana = String::new();
    let mut index = 0;

    while index < chars.len() {
        let prev = chars.get(index.wrapping_sub(1));
        let current = chars.get(index);
        let next = chars.get(index + 1);

        if let Some(&current) = current {
            if is_vowel(&current) {
                if let Some(&value) = TABLE_1.get(&current.to_string().as_ref()) {
                    kana.push_str(value);
                    index += 1;
                    continue;
                }
            }

            if is_consonant(&current) {
                if let Some(&next) = next {
                    if current == next && !['n', 'y', 'w'].contains(&current) {
                        kana.push_str(TABLE_1.get("t").unwrap());
                        index += 1;
                        continue;
                    }

                    let k1 = format!("{}{}", current, next);
                    if let Some(&value) = TABLE_1.get(&k1.as_ref()) {
                        kana.push_str(value);
                        index += 2;
                        continue;
                    }
                }

                if let Some(&value) = TABLE_1.get(&current.to_string().as_ref()) {
                    kana.push_str(value);
                    index += 1;
                    continue;
                }
            }

            if current == 'r' || current == 'h' {
                if let Some(&prev) = prev {
                    let k0 = format!("{}{}", prev, current);
                    if let Some(&value) = TABLE_2.get(&k0.as_ref()) {
                        kana.push_str(value);
                        index += 1;
                        continue;
                    }
                }
            }

            if '\'' == current {
                index += 1;
                continue;
            }

            kana.push(current);
            index += 1;
        }
    }

    kana
}
