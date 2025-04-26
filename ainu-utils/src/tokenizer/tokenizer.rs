use crate::tokenizer::unfix::unfix;

pub fn tokenize(text: &str, keep_whitespace: bool) -> Vec<String> {
    let mut words = Vec::new();
    let mut word = String::new();

    for c in text.chars() {
        if c.is_alphabetic() || c.is_numeric() || c == '=' {
            word.push(c);
        } else if c == '\'' && !word.is_empty() {
            word.push(c);
        } else if c == '-' && !word.is_empty() {
            word.push(c);
        } else {
            if !word.is_empty() {
                words.extend(unfix(word));
                word = String::new();
            }

            if !c.is_whitespace() {
                words.push(c.to_string());
            }

            if c.is_whitespace() && keep_whitespace {
                words.push(c.to_string());
            }
        }
    }

    if !word.is_empty() {
        words.extend(unfix(word));
    }

    words
}
