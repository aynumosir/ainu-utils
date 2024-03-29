use super::dict::{get_entries, NumberForm};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Int(i64),
    Add { lhs: Box<Expr>, rhs: Box<Expr> },
    Sub { lhs: Box<Expr>, rhs: Box<Expr> },
    Mul { lhs: Box<Expr>, rhs: Box<Expr> },
}

impl Expr {
    pub fn new(input: i64) -> Result<Expr, String> {
        if input < 0 || 199 < input {
            return Err(format!("{} is out of range", input));
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
                lhs: Box::new(Expr::new(input + 10)?),
                rhs: Box::new(Expr::Int(10)),
            });
        }

        let ones = input % 10;
        let tens = input - ones;

        Ok(Expr::Add {
            lhs: Box::new(Expr::new(tens)?),
            rhs: Box::new(Expr::new(ones)?),
        })
    }

    pub fn eval(&self) -> i64 {
        match self {
            Expr::Int(n) => *n,
            Expr::Add { lhs, rhs } => lhs.eval() + rhs.eval(),
            Expr::Sub { lhs, rhs } => lhs.eval() - rhs.eval(),
            Expr::Mul { lhs, rhs } => lhs.eval() * rhs.eval(),
        }
    }

    pub fn show(&self, form: &NumberForm) -> String {
        let dict = get_entries();

        match self {
            Expr::Int(n) => dict.get(n).unwrap().to_string(form.clone()),
            Expr::Add { lhs, rhs } => format!("{} ikasma {}", rhs.show(form), lhs.show(form)),
            Expr::Sub { lhs, rhs } => format!("{} e{}", rhs.show(form), lhs.show(form)),
            Expr::Mul { lhs, rhs } => {
                // 掛け算のときは無条件に普通形式
                format!("{}{}", lhs.show(&NumberForm::Regular), rhs.show(form))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_expr() {
        let expr = Expr::new(99).unwrap();

        let l1 = expr.show(&NumberForm::Regular);
        assert_eq!(l1, "sinepesan ikasma wan easiknehotne");

        let l2 = expr.show(&NumberForm::Thing);
        assert_eq!(l2, "sinepesanpe ikasma wanpe easiknehotnep");

        let l2 = expr.show(&NumberForm::Person);
        assert_eq!(l2, "sinepesaniw ikasma waniw easiknehotnen");

        let l3 = expr.show(&NumberForm::Custom("seta".to_string()));
        assert_eq!(l3, "sinepesan seta ikasma wan seta easiknehotne seta");
    }

    #[test]
    fn it_handles_ten_and_twenty() {
        let expr = Expr::new(10).unwrap();
        assert_eq!(expr, Expr::Int(10));

        let expr = Expr::new(20).unwrap();
        assert_eq!(expr, Expr::Int(20));
    }

    #[test]
    fn it_cannot_handle_unsupported_range_of_value() {
        let expr = Expr::new(200);
        assert_eq!(expr, Err("200 is out of range".to_string()));
    }
}
