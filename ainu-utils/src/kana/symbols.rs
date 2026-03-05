static PUNC: [(&str, &str); 14] = [
    ("-", ""),
    ("=", ""),
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

    for (from, to) in PUNC.iter() {
        output = output.replace(from, to);
    }

    output
}
