use std::collections::HashMap;

static DICT: &[(i64, &str)] = &[
    (1, "sine"),
    (2, "tu"),
    (3, "re"),
    (4, "ine"),
    (5, "asikne"),
    (6, "iwan"),
    (7, "arwan"),
    (8, "tupesan"),
    (9, "sinepesan"),
    (10, "wan"),
    (20, "hotne"),
];

pub struct Number {
    word: String,
    pub value: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumberForm {
    Regular,
    Person,
    Thing,
    Custom(String),
}

impl Number {
    pub fn to_string(&self, form: NumberForm) -> String {
        match form {
            NumberForm::Regular => self.to_regular(),
            NumberForm::Person => self.to_person(),
            NumberForm::Thing => self.to_thing(),
            NumberForm::Custom(s) => self.to_custom(s),
        }
    }

    fn to_regular(&self) -> String {
        self.word.to_string()
    }

    fn to_person(&self) -> String {
        if self.word.ends_with("n") {
            self.word.to_string() + "iw"
        } else {
            self.word.to_string() + "n"
        }
    }

    fn to_thing(&self) -> String {
        if self.word.ends_with("n") {
            self.word.to_string() + "pe"
        } else {
            self.word.to_string() + "p"
        }
    }

    fn to_custom(&self, s: String) -> String {
        self.word.to_string() + " " + &s
    }
}

pub fn get_entries() -> HashMap<i64, Number> {
    let mut entries = HashMap::new();
    for (value, word) in DICT {
        entries.insert(
            *value,
            Number {
                value: *value,
                word: word.to_string(),
            },
        );
    }
    entries
}
