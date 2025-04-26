use once_cell::sync::Lazy;
use regex::Regex;

static PREFIX: Lazy<Regex> = Lazy::new(|| {
    let prefixes = [
        "a=", "ae=", "aen=", "an=", "aun=", "ay=", "c=", "ci=", "e=", "eci=", "ecien=", "ecii=",
        "eciun=", "en=", "ey=", "i=", "k=", "ku=", "kuy=", "un=",
    ];
    let pattern = &format!(r"^(?<prefix>{})(?<stem>.+)", prefixes.join("|"));
    Regex::new(pattern).unwrap()
});

static SUFFIX: Lazy<Regex> = Lazy::new(|| {
    let suffixes = ["=an", "=as"];
    let pattern = &format!(r"(?<stem>.+)(?<suffix>{})$", suffixes.join("|"));
    Regex::new(pattern).unwrap()
});

pub fn unfix(token: String) -> Vec<String> {
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
