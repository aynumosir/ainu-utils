use super::tokenizer::tokenize;

#[test]
fn test_tokenize() {
    let text = "irankarapte! eyami yak a=ye aeywankep ku=kar wa k=an.";
    let tokens = tokenize(text, false);

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
    let tokens = tokenize(text, false);

    assert_eq!(
        tokens,
        vec!["soyenpa", "=an", "wa", "sinot", "=an", "ro", "!"]
    );
}

#[test]
fn test_sentence_does_not_end_with_period() {
    let text = "a=nukar hike i=yaykohaytare i=yaypokaste wa iki pe";
    let tokens = tokenize(text, false);

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
    let tokens = tokenize(text, false);

    assert_eq!(
        tokens,
        vec![
            "neno", "a=", "ye", "itak", "pirka", "a=", "ye", "itak", "i=", "koynu", "wa", ".", ".",
            ".", "i=", "konu", "wa", "i=", "kore"
        ]
    );
}

#[test]
fn test_parse_numbers() {
    let text = "1000 yen ku=kor";
    let tokens = tokenize(text, false);

    assert_eq!(tokens, vec!["1000", "yen", "ku=", "kor"]);
}

#[test]
fn test_handles_hyphen_within_word() {
    let text = "cep-koyki wa e";
    let tokens = tokenize(text, false);
    assert_eq!(tokens, vec!["cep-koyki", "wa", "e"]);
}

#[test]
fn test_handles_double_prefixes() {
    let text = "niwen seta ne kusu a=e=kupa na.";
    let tokens = tokenize(text, false);
    assert_eq!(
        tokens,
        vec!["niwen", "seta", "ne", "kusu", "a=", "e=", "kupa", "na", "."]
    );
}

#[test]
fn test_handles_glottal_stop() {
    let text = "ku=kor irwak'utari";
    let tokens = tokenize(text, false);
    assert_eq!(tokens, vec!["ku=", "kor", "irwak'utari"]);

    let text = "'ku=kor rusuy!' sekor hawean";
    let tokens = tokenize(text, false);
    assert_eq!(
        tokens,
        vec!["'", "ku=", "kor", "rusuy", "!", "'", "sekor", "hawean"]
    );
}

#[test]
fn test_keep_whitespace() {
    let text = "irankarapte. tanto sirpirka ne.";
    let tokens = tokenize(text, true);
    assert_eq!(
        tokens,
        vec![
            "irankarapte",
            ".",
            " ",
            "tanto",
            " ",
            "sirpirka",
            " ",
            "ne",
            "."
        ]
    );
}
