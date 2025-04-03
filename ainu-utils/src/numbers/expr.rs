pub enum Expr {
    Int(i32),
    Add { lhs: Box<Expr>, rhs: Box<Expr> },
    Sub { lhs: Box<Expr>, rhs: Box<Expr> },
    Mul { lhs: Box<Expr>, rhs: Box<Expr> },
}

impl ToString for Expr {
    fn to_string(&self) -> String {
        match self {
            Expr::Int(i) => match i {
                1 => "sine".to_string(),
                2 => "tu".to_string(),
                3 => "re".to_string(),
                4 => "ine".to_string(),
                5 => "asikne".to_string(),
                6 => "iwan".to_string(),
                7 => "arwan".to_string(),
                8 => "tupesan".to_string(),
                9 => "sinepesan".to_string(),
                10 => "wan".to_string(),
                20 => "hotne".to_string(),
                _ => panic!("Invalid integer for Ainu number: {}", i),
            },
            Expr::Add { lhs, rhs } => format!("{} ikasma {}", lhs.to_string(), rhs.to_string()),
            Expr::Sub { lhs, rhs } => format!("{} e{}", rhs.to_string(), lhs.to_string()),
            Expr::Mul { lhs, rhs } => format!("{}{}", lhs.to_string(), rhs.to_string()),
        }
    }
}

pub fn parse(input: i32) -> Result<Expr, String> {
    if input < 0 || 100 < input {
        return Err("Input must be between 0 and 100".to_string());
    }

    if input < 10 {
        return Ok(Expr::Int(input));
    }

    if input == 10 || input == 20 {
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
