static SYMBOLS: [(&str, &str); 16] = [
    ("-", ""),
    ("=", ""),
    (" ", "　"),
    ("“", "「"),
    ("”", "」"),
    ("‘", "『"),
    ("’", "』"),
    ("...", "…"),
    ("(", "（"),
    (")", "）"),
    (",", "、"),
    (".", "。"),
    ("!", "！"),
    ("?", "？"),
    ("'", ""),
    ("`", ""),
];

pub fn symbols(input: String) -> String {
    let mut input = input;

    for (from, to) in SYMBOLS.iter() {
        input = input.replace(from, to);
    }

    input
}
