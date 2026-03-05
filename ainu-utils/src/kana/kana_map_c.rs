pub fn map_c(current: &char, prev: Option<&char>) -> String {
    let value = match &current {
        'k' => "ㇰ",
        's' => "ㇱ",
        't' => "ッ",
        'n' => "ン",
        'p' => "ㇷ゚",
        'm' => "ㇺ",
        'y' => "イ",
        'w' => "ウ",
        'r' => match prev {
            Some(prev) => match prev {
                'a' => "ㇻ",
                'i' => "ㇼ",
                'u' => "ㇽ",
                'e' => "ㇾ",
                'o' => "ㇿ",
                _ => &*current.to_string(),
            },
            None => &current.to_string(),
        },
        'h' => match prev {
            Some(prev) => match prev {
                'a' => "ㇵ",
                'i' => "ㇶ",
                'u' => "ㇷ",
                'e' => "ㇸ",
                'o' => "ㇹ",
                _ => &*current.to_string(),
            },
            None => &current.to_string(),
        },
        _ => &current.to_string(),
    };

    value.to_string()
}
