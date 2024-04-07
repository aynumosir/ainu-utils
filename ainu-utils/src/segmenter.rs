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
    let mut words = Vec::new();

    if let Some(caps) = PREFIX_REGEX.captures(&token) {
        words.push(caps["prefix"].to_string());
        words.push(caps["word"].to_string());
    } else if let Some(caps) = SUFFIX_REGEX.captures(&token) {
        words.push(caps["word"].to_string());
        words.push(caps["suffix"].to_string());
    } else {
        words.push(token);
    }

   words 
}

pub fn segment(text: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut word = String::new();

    for c in text.chars() {
        if c.is_alphabetic() || c.is_numeric() || c == '='  {
            word.push(c);
        } else {
            if !word.is_empty() {
                words.extend(parse_affix(word));
                word = String::new();
            }

            if !c.is_whitespace() {
                words.push(c.to_string());
            }
        }
    }

    if !word.is_empty() {
        words.extend(parse_affix(word));
    }

   words 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment() {
        let text = "irankarapte! eyami yak a=ye aeywankep ku=kar wa k=an.";
        let tokens = segment(text);

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
    fn test_segment_suffix() {
        let text = "soyenpa=an wa sinot=an ro!";
        let tokens = segment(text);

        assert_eq!(
            tokens,
            vec!["soyenpa", "=an", "wa", "sinot", "=an", "ro", "!"]
        );
    }

    #[test]
    fn test_sentence_does_not_end_with_period() {
        let text = "a=nukar hike i=yaykohaytare i=yaypokaste wa iki pe";
        let tokens = segment(text);

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
        let tokens = segment(text);

        assert_eq!(
            tokens,
            vec![
                "neno", "a=", "ye", "itak", "pirka", "a=", "ye", "itak", "i=", "koynu", "wa", ".",
                ".", ".", "i=", "konu", "wa", "i=", "kore"
            ]
        );
    }

    #[test]
    fn test_parse_numbers() {
        let text = "1000 yen ku=kor";
        let tokens = segment(text);

        assert_eq!(tokens, vec!["1000", "yen", "ku=", "kor"]);
    }
}
