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

pub fn segment(text: &str) -> Vec<String> {
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
        }
    }

    if !word.is_empty() {
        words.extend(unfix(word));
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

    #[test]
    fn test_handles_hyphen_within_word() {
        let text = "cep-koyki wa e";
        let tokens = segment(text);
        assert_eq!(tokens, vec!["cep-koyki", "wa", "e"]);
    }

    #[test]
    fn test_handles_double_prefixes() {
        let text = "niwen seta ne kusu a=e=kupa na.";
        let tokens = segment(text);
        assert_eq!(
            tokens,
            vec!["niwen", "seta", "ne", "kusu", "a=", "e=", "kupa", "na", "."]
        );
    }

    #[test]
    fn test_handles_glottal_stop() {
        let text = "ku=kor irwak'utari";
        let tokens = segment(text);
        assert_eq!(tokens, vec!["ku=", "kor", "irwak'utari"]);

        let text = "'ku=kor rusuy!' sekor hawean";
        let tokens = segment(text);
        assert_eq!(
            tokens,
            vec!["'", "ku=", "kor", "rusuy", "!", "'", "sekor", "hawean"]
        );
    }
}
