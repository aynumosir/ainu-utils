pub fn map_v(v: &char) -> char {
    match v {
        'a' => 'ア',
        'i' => 'イ',
        'u' => 'ウ',
        'e' => 'エ',
        'o' => 'オ',
        _ => unreachable!(),
    }
}
