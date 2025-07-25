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

pub fn map_symbols(input: &str) -> String {
    let mut output = input.to_string();

    for (from, to) in SYMBOLS.iter() {
        output = output.replace(from, to);
    }

    output
}
