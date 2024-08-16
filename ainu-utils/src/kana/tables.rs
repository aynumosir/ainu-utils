use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static TABLE_1: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let items = [
        ("a", "ア"),
        ("i", "イ"),
        ("u", "ウ"),
        ("e", "エ"),
        ("o", "オ"),
        ("k", "ㇰ"),
        ("s", "ㇱ"),
        ("t", "ッ"),
        ("n", "ン"),
        ("p", "ㇷ゚"),
        ("m", "ㇺ"),
        ("y", "イ"),
        ("w", "ウ"),
        ("ka", "カ"),
        ("ki", "キ"),
        ("ku", "ク"),
        ("ke", "ケ"),
        ("ko", "コ"),
        ("sa", "サ"),
        ("si", "シ"),
        ("su", "ス"),
        ("se", "セ"),
        ("so", "ソ"),
        ("ta", "タ"),
        ("tu", "トゥ"),
        ("te", "テ"),
        ("to", "ト"),
        ("ca", "チャ"),
        ("ci", "チ"),
        ("cu", "チュ"),
        ("ce", "チェ"),
        ("co", "チョ"),
        ("na", "ナ"),
        ("ni", "ニ"),
        ("nu", "ヌ"),
        ("ne", "ネ"),
        ("no", "ノ"),
        ("ha", "ハ"),
        ("hi", "ヒ"),
        ("hu", "フ"),
        ("he", "ヘ"),
        ("ho", "ホ"),
        ("pa", "パ"),
        ("pi", "ピ"),
        ("pu", "プ"),
        ("pe", "ペ"),
        ("po", "ポ"),
        ("ma", "マ"),
        ("mi", "ミ"),
        ("mu", "ム"),
        ("me", "メ"),
        ("mo", "モ"),
        ("ya", "ヤ"),
        ("yu", "ユ"),
        ("ye", "イェ"),
        ("yo", "ヨ"),
        ("ra", "ラ"),
        ("ri", "リ"),
        ("ru", "ル"),
        ("re", "レ"),
        ("ro", "ロ"),
        ("wa", "ワ"),
        ("wi", "ウィ"),
        ("we", "ウェ"),
        ("wo", "ウォ"),
    ];

    items.iter().cloned().collect()
});

pub static TABLE_2: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let items = [
        ("ar", "ㇻ"),
        ("ir", "ㇼ"),
        ("ur", "ㇽ"),
        ("er", "ㇾ"),
        ("or", "ㇿ"),
        ("ah", "ㇵ"),
        ("ih", "ㇶ"),
        ("uh", "ㇷ"),
        ("eh", "ㇸ"),
        ("oh", "ㇹ"),
    ];

    items.iter().cloned().collect()
});

pub static TABLE_3: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let items = [(",", "、"), (".", "。")];
    items.iter().cloned().collect()
});
