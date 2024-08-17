use once_cell::sync::Lazy;
use regex::Regex;

const PREFIXES: [&str; 20] = [
    "a=", "ae=", "aen=", "an=", "aun=", "ay=", "c=", "ci=", "e=", "eci=", "ecien=", "ecii=",
    "eciun=", "en=", "ey=", "i=", "k=", "ku=", "kuy=", "un=",
];

const SUFFIXES: [&str; 2] = ["=an", "=as"];

static PREFIX: Lazy<Regex> = Lazy::new(|| {
    let pattern = &format!(r"^(?<prefix>{})(?<stem>.+)", PREFIXES.join("|"));
    Regex::new(pattern).unwrap()
});

static SUFFIX: Lazy<Regex> = Lazy::new(|| {
    let pattern = &format!(r"(?<stem>.+)(?<suffix>{})$", SUFFIXES.join("|"));
    Regex::new(pattern).unwrap()
});

fn unfix(token: String) -> Vec<String> {
    if token == "an=an" {
        return vec!["an".to_string(), "=an".to_string()];
    }

    let prefix = PREFIX.captures(&token);
    if let Some(captures) = prefix {
        let mut words = vec![];
        words.push(captures["prefix"].to_string());
        words.extend(unfix(captures["stem"].to_string()));
        return words;
    }

    let suffix = SUFFIX.captures(&token);
    if let Some(captures) = suffix {
        let mut words = vec![];
        words.extend(unfix(captures["stem"].to_string()));
        words.push(captures["suffix"].to_string());
        return words;
    }

    vec![token]
}

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
