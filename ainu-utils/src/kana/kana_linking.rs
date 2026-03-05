// https://ainugo.nam.go.jp/pages/guide.html
static LINKING: [(&str, &str); 26] = [
    ("_h", ""),
    ("_y", ""),
    (" __h", ""),
    (" __y", ""),
    (" _a", "a"),
    (" _i", "i"),
    (" _u", "u"),
    (" _e", "e"),
    (" _o", "o"),
    (" __a", ""),
    (" __i", ""),
    (" __u", ""),
    (" __e", ""),
    (" __o", ""),
    ("r_ n", "n n"),
    ("r_ r", "n r"),
    ("r_ t", "t t"),
    ("r_ c", "t c"),
    ("n_ s", "y s"),
    ("n_ y", "y y"),
    ("n_ w", "u w"),
    ("n _w", "n m"),
    // ("n _w", "n m"),
    ("m _w", "n m"),
    ("p _w", "t p"),
    ("mp", "np"),
    ("mm", "nm"),
];

pub fn link(input: &str) -> String {
    let mut output: String = input.to_string();

    for (from, to) in LINKING.iter() {
        output = output.replace(from, to);
    }

    output
}
