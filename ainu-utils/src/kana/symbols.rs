static SYMBOLS: [(&str, &str); 15] = [
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
    ("`", ""),
];

pub fn map_symbols(input: String) -> String {
    let mut input = input;

    for (from, to) in SYMBOLS.iter() {
        input = input.replace(from, to);
    }

    input
}
