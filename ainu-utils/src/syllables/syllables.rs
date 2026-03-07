use crate::phoneme::Phoneme;

pub fn syllabicate(input: &str) -> Vec<String> {
    let chars: Vec<char> = input.chars().collect();

    let mut syllables = vec![];
    let mut index = 0;

    while index < chars.len() {
        match chars.get(index) {
            Some(current) if current.is_vowel() => {
                match chars.get(index + 1) {
                    Some(next) if next.is_consonant() => {
                        match chars.get(index + 2) {
                            Some(next_next) if next_next.is_vowel() => {
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
            Some(current) if current.is_consonant() => {
                match chars.get(index + 1) {
                    Some(next) if next.is_vowel() => {
                        match chars.get(index + 2) {
                            Some(next_next) if next_next.is_consonant() => {
                                match chars.get(index + 3) {
                                    Some(next_next_next) if next_next_next.is_vowel() => {
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
