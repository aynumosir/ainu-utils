use ainu_utils::number::Expr;
use ainu_utils::number::NumberForm;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let text = &args[1];

    let int = text.parse::<i64>().expect("Failed to parse integer");

    let expr = Expr::new(int)?;
    let str = expr.show(&NumberForm::Thing);

    println!("{:?}", str);

    Ok(())
}
