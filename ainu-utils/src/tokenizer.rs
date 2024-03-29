use once_cell::sync::Lazy;
use regex::Regex;

const PREFIXES: [&str; 20] = [
    "a=", "ae=", "aen=", "an=", "aun=", "ay=", "c=", "ci=", "e=", "eci=", "ecien=", "ecii=",
    "eciun=", "en=", "ey=", "i=", "k=", "ku=", "kuy=", "un=",
];

const SUFFIXES: [&str; 2] = ["=an", "=as"];

static PREFIX_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(&format!(r"^(?<prefix>{})(?<word>\w+)", PREFIXES.join("|"))).unwrap());

static SUFFIX_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(&format!(r"(?<word>\w+)(?<suffix>{})$", SUFFIXES.join("|"))).unwrap());

fn parse_affix(token: String) -> Vec<String> {
    let mut tokens = Vec::new();

    if let Some(caps) = PREFIX_REGEX.captures(&token) {
        tokens.push(caps["prefix"].to_string());
        tokens.push(caps["word"].to_string());
    } else if let Some(caps) = SUFFIX_REGEX.captures(&token) {
        tokens.push(caps["word"].to_string());
        tokens.push(caps["suffix"].to_string());
    } else {
        tokens.push(token);
    }

    tokens
}

pub fn tokenize(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut token = String::new();

    for c in text.chars() {
        if c.is_alphabetic() || c == '=' {
            token.push(c);
        } else {
            if !token.is_empty() {
                tokens.extend(parse_affix(token));
                token = String::new();
            }

            if !c.is_whitespace() {
                tokens.push(c.to_string());
            }
        }
    }

    if !token.is_empty() {
        tokens.extend(parse_affix(token));
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let text = "irankarapte! eyami yak a=ye aeywankep ku=kar wa k=an.";
        let tokens = tokenize(text);

        assert_eq!(
            tokens,
            vec![
                "irankarapte",
                "!",
                "eyami",
                "yak",
                "a=",
                "ye",
                "aeywankep",
                "ku=",
                "kar",
                "wa",
                "k=",
                "an",
                "."
            ]
        );
    }

    #[test]
    fn test_tokenize_suffix() {
        let text = "soyenpa=an wa sinot=an ro!";
        let tokens = tokenize(text);

        assert_eq!(
            tokens,
            vec!["soyenpa", "=an", "wa", "sinot", "=an", "ro", "!"]
        );
    }

    #[test]
    fn test_sentence_does_not_end_with_period() {
        let text = "a=nukar hike i=yaykohaytare i=yaypokaste wa iki pe";
        let tokens = tokenize(text);

        assert_eq!(
            tokens,
            vec![
                "a=",
                "nukar",
                "hike",
                "i=",
                "yaykohaytare",
                "i=",
                "yaypokaste",
                "wa",
                "iki",
                "pe"
            ]
        );
    }

    #[test]
    fn test_sentence_ending_with_a_fixed_word() {
        let text = "neno a=ye itak pirka a=ye itak i=koynu wa ... i=konu wa i=kore";
        let tokens = tokenize(text);

        assert_eq!(
            tokens,
            vec![
                "neno", "a=", "ye", "itak", "pirka", "a=", "ye", "itak", "i=", "koynu", "wa", ".",
                ".", ".", "i=", "konu", "wa", "i=", "kore"
            ]
        );
    }
}
