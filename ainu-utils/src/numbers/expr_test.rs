use crate::numbers::NumeralForm;

use super::expr::parse;

#[test]
fn test_simple() {
    let expr = parse(1).unwrap();
    assert_eq!(expr.format(&NumeralForm::ThingCount), "sinep");
}

#[test]
fn test_ten_and_twenty() {
    let expr = parse(10).unwrap();
    assert_eq!(expr.format(&NumeralForm::ThingCount), "wanpe");

    let expr = parse(20).unwrap();
    assert_eq!(expr.format(&NumeralForm::ThingCount), "hotnep");
}

#[test]
fn test_addition() {
    let expr = parse(11).unwrap();
    assert_eq!(expr.format(&NumeralForm::ThingCount), "sinep ikasma wanpe");
}

#[test]
fn test_subtraction() {
    let expr = parse(90).unwrap();
    assert_eq!(expr.format(&NumeralForm::ThingCount), "wanpe easiknehotnep");
}

#[test]
fn test_serial() {
    let expr = parse(31).unwrap();
    assert_eq!(expr.format(&NumeralForm::Serial), "sinep ikasma wan etuhot");
}

#[test]
fn test_human() {
    let expr = parse(31).unwrap();
    assert_eq!(
        expr.format(&NumeralForm::HumanCount),
        "sinen ikasma waniw etuhotnen"
    );
}

#[test]
fn test_noun() {
    let expr = parse(31).unwrap();
    assert_eq!(
        expr.format(&NumeralForm::Adnominal("cape".to_string())),
        "sine cape ikasma wan cape etuhotne cape"
    );
}
