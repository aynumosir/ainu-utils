use crate::phonology::{is_consonant, is_vowel};

pub fn parse(input: &str) -> Vec<String> {
    let chars: Vec<char> = input.chars().collect();

    let mut syllables = vec![];
    let mut index = 0;

    while index < chars.len() {
        match chars.get(index) {
            Some(current) if is_vowel(current) => {
                match chars.get(index + 1) {
                    Some(next) if is_consonant(next) => {
                        match chars.get(index + 2) {
                            Some(next_next) if is_vowel(next_next) => {
                                syllables.push(current.to_string());
                                index += 1;
                            }
                            _ => {
                                syllables.push(format!("{}{}", current, next));
                                index += 2;
                            }
                        };
                    }
                    _ => {
                        syllables.push(current.to_string());
                        index += 1;
                    }
                };
            }
            Some(current) if is_consonant(current) => {
                match chars.get(index + 1) {
                    Some(next) if is_vowel(next) => {
                        match chars.get(index + 2) {
                            Some(next_next) if is_consonant(next_next) => {
                                match chars.get(index + 3) {
                                    Some(next_next_next) if is_vowel(next_next_next) => {
                                        syllables.push(format!("{}{}", current, next));
                                        index += 2;
                                    }
                                    _ => {
                                        syllables.push(format!("{}{}{}", current, next, next_next));
                                        index += 3;
                                    }
                                };
                            }
                            _ => {
                                syllables.push(format!("{}{}", current, next));
                                index += 2;
                            }
                        };
                    }
                    _ => {
                        syllables.push(current.to_string());
                        index += 1;
                    }
                };
            }
            Some(current) => {
                syllables.push(current.to_string());
                index += 1;
            }
            None => break,
        };
    }

    return syllables;
}
