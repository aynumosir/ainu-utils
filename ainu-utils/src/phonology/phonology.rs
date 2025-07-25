use once_cell::sync::Lazy;
use std::collections::HashSet;

pub static VOWELS: Lazy<HashSet<char>> = Lazy::new(|| {
    let items = ['a', 'i', 'u', 'e', 'o'];
    items.iter().cloned().collect()
});

pub static CONSONANTS: Lazy<HashSet<char>> = Lazy::new(|| {
    let items = [
        'k', 'g', 's', 'z', 't', 'd', 'c', 'j', 'n', 'h', 'p', 'b', 'f', 'm', 'y', 'r', 'w',
    ];
    items.iter().cloned().collect()
});

pub fn is_vowel(c: &char) -> bool {
    return VOWELS.contains(c);
}

pub fn is_consonant(c: &char) -> bool {
    return CONSONANTS.contains(c);
}
