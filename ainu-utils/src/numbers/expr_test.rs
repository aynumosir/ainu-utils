use super::expr::parse;

#[test]
fn test_simple() {
    let expr = parse(1).unwrap();
    assert_eq!(expr.to_string(), "sine");
}

#[test]
fn test_ten_and_twenty() {
    let expr = parse(10).unwrap();
    assert_eq!(expr.to_string(), "wan");

    let expr = parse(20).unwrap();
    assert_eq!(expr.to_string(), "hotne");
}

#[test]
fn test_addition() {
    let expr = parse(11).unwrap();
    assert_eq!(expr.to_string(), "sine ikasma wan");
}

#[test]
fn test_subtraction() {
    let expr = parse(90).unwrap();
    assert_eq!(expr.to_string(), "wan easiknehotne");
}
