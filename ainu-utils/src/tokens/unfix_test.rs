use super::unfix::unfix;

#[test]
fn test_prefix() {
    let result = unfix("a=ye".to_string());
    assert_eq!(result, vec!["a=".to_string(), "ye".to_string()]);
}

#[test]
fn test_suffix() {
    let result = unfix("soyenpa=an".to_string());
    assert_eq!(result, vec!["soyenpa".to_string(), "=an".to_string()]);
}
