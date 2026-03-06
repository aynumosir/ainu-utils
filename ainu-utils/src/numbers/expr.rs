use thiserror::Error;

#[derive(Error, Debug)]
pub enum NumberToWordsError {
    #[error("number out of range")]
    OutOfRange,
}

pub enum NumeralForm {
    HumanCount,
    ThingCount,
    Serial,
    AdnominalBase,
    Adnominal(String),
}

trait I32Ext {
    fn format(&self, kind: &NumeralForm) -> String;
}

impl I32Ext for i32 {
    fn format(&self, kind: &NumeralForm) -> String {
        let v = match kind {
            NumeralForm::HumanCount => match self {
                1 => "sinen",
                2 => "tun",
                3 => "ren",
                4 => "inen",
                5 => "asiknen",
                6 => "iwaniw",
                7 => "arwaniw",
                8 => "tupesaniw",
                9 => "sinepesaniw",
                10 => "waniw",
                20 => "hotnen",
                _ => unreachable!(),
            },
            NumeralForm::ThingCount => match self {
                1 => "sinep",
                2 => "tup",
                3 => "rep",
                4 => "inep",
                5 => "asiknep",
                6 => "iwanpe",
                7 => "arwanpe",
                8 => "tupesanpe",
                9 => "sinepesanpe",
                10 => "wanpe",
                20 => "hotnep",
                _ => unreachable!(),
            },
            NumeralForm::Serial => match self {
                1 => "sinep",
                2 => "tup",
                3 => "rep",
                4 => "inep",
                5 => "asik",
                6 => "iwan",
                7 => "arwan",
                8 => "tupes",
                9 => "sinepes",
                10 => "wan",
                20 => "hot",
                _ => unreachable!(),
            },
            NumeralForm::AdnominalBase => match self {
                1 => "sine",
                2 => "tu",
                3 => "re",
                4 => "ine",
                5 => "asikne",
                6 => "iwan",
                7 => "arwan",
                8 => "tupesan",
                9 => "sinepesan",
                10 => "wan",
                20 => "hotne",
                _ => unreachable!(),
            },
            NumeralForm::Adnominal(noun) => {
                &format!("{} {}", self.format(&NumeralForm::AdnominalBase), noun)
            }
        };

        v.to_string()
    }
}

pub enum Expr {
    Int(i32),
    Add { lhs: Box<Expr>, rhs: Box<Expr> },
    Sub { lhs: Box<Expr>, rhs: Box<Expr> },
    Mul { lhs: Box<Expr>, rhs: Box<Expr> },
}

impl Expr {
    pub fn format(&self, kind: &NumeralForm) -> String {
        match self {
            Expr::Int(i) => i.format(kind),
            Expr::Add { lhs, rhs } => format!("{} ikasma {}", lhs.format(kind), rhs.format(kind)),
            Expr::Sub { lhs, rhs } => format!("{} e{}", rhs.format(kind), lhs.format(kind)),
            Expr::Mul { lhs, rhs } => {
                format!(
                    "{}{}",
                    lhs.format(&NumeralForm::AdnominalBase),
                    rhs.format(kind)
                )
            }
        }
    }
}

pub fn parse(input: i32) -> Result<Expr, NumberToWordsError> {
    if input < 0 || 100 < input {
        return Err(NumberToWordsError::OutOfRange);
    }

    if input <= 10 || input == 20 {
        return Ok(Expr::Int(input));
    }

    if input % 20 == 0 {
        return Ok(Expr::Mul {
            lhs: Box::new(Expr::Int(input / 20)),
            rhs: Box::new(Expr::Int(20)),
        });
    }

    if input % 20 == 10 {
        return Ok(Expr::Sub {
            lhs: Box::new(parse(input + 10)?),
            rhs: Box::new(Expr::Int(10)),
        });
    }

    let ones = input % 10;
    let tens = input - ones;

    return Ok(Expr::Add {
        lhs: Box::new(parse(ones)?),
        rhs: Box::new(parse(tens)?),
    });
}

pub fn number_to_words(number: i32, form: &NumeralForm) -> Result<String, NumberToWordsError> {
    let expr = parse(number)?;
    Ok(expr.format(form))
}
