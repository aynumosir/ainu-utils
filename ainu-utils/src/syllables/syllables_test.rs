use super::syllables::parse;

#[test]
fn it_parses() {
    assert_eq!(parse("pirka"), ["pir", "ka"]);
    assert_eq!(parse("cikappo"), ["ci", "kap", "po"]);
    assert_eq!(parse("aep"), ["a", "ep"]);
    assert_eq!(
        parse("eyaykosiramsuypa"),
        ["e", "yay", "ko", "si", "ram", "suy", "pa"]
    );
    assert_eq!(
        parse("eci=koyayrayke p ne na!"),
        ["e", "ci", "=", "ko", "yay", "ray", "ke", " ", "p", " ", "ne", " ", "na", "!"]
    )
}


#[test]
fn it_handles_accent_symbols_as_well() {
    assert_eq!(parse("káni"), ["ká", "ni"])
}