use super::sets::{CONSONANTS, SPECIAL_CONSONANTS, SYMBOLS, VOWELS};
use super::tables::{TABLE_1, TABLE_2, TABLE_3};
use diacritics::remove_diacritics;

fn normalize(input: &str) -> String {
    let mut input = input.to_string();

    input = input.to_lowercase();
    input = input.replace("=", "");
    input = remove_diacritics(&input).to_string();

    input
}

pub fn to_kana(input: &str) -> String {
    let input = normalize(input);
    let chars: Vec<char> = input.chars().collect();

    let mut kana = String::new();
    let mut index = 0;

    while index < chars.len() {
        let prev = chars.get(index.wrapping_sub(1));
        let current = chars.get(index);
        let next = chars.get(index + 1);

        if let Some(&current) = current {
            if VOWELS.contains(&current) {
                if let Some(&value) = TABLE_1.get(&current.to_string().as_ref()) {
                    kana.push_str(value);
                    index += 1;
                    continue;
                }
            }

            if CONSONANTS.contains(&current) {
                if let Some(&next) = next {
                    if current == next && current != 'n' {
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

            if SPECIAL_CONSONANTS.contains(&current) {
                if let Some(&prev) = prev {
                    let k0 = format!("{}{}", prev, current);
                    if let Some(&value) = TABLE_2.get(&k0.as_ref()) {
                        kana.push_str(value);
                        index += 1;
                        continue;
                    }
                }
            }

            if SYMBOLS.contains(&current) {
                if let Some(&value) = TABLE_3.get(&current.to_string().as_ref()) {
                    kana.push_str(value);
                    index += 1;
                    continue;
                }
            }

            kana.push(current);
            index += 1;
        }
    }

    kana
}
