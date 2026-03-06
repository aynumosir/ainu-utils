pub trait Phoneme {
    fn is_vowel(&self) -> bool;
    fn is_consonant(&self) -> bool;
}

impl Phoneme for char {
    fn is_vowel(&self) -> bool {
        ['a', 'i', 'u', 'e', 'o', 'á', 'í', 'ú', 'é', 'ó'].contains(self)
    }

    fn is_consonant(&self) -> bool {
        [
            'k', 'g', 's', 'z', 't', 'd', 'c', 'j', 'n', 'h', 'p', 'b', 'f', 'm', 'y', 'r', 'w',
        ]
        .contains(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vowel() {
        assert!('a'.is_vowel());
    }

    #[test]
    fn test_consonant() {
        assert!('k'.is_consonant());
    }
}
