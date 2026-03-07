use super::syllables::syllabicate;

#[test]
fn it_syllabicates() {
    assert_eq!(syllabicate("pirka"), ["pir", "ka"]);
    assert_eq!(syllabicate("cikappo"), ["ci", "kap", "po"]);
    assert_eq!(syllabicate("aep"), ["a", "ep"]);
    assert_eq!(
        syllabicate("eyaykosiramsuypa"),
        ["e", "yay", "ko", "si", "ram", "suy", "pa"]
    );
    assert_eq!(
        syllabicate("eci=koyayrayke p ne na!"),
        [
            "e", "ci", "=", "ko", "yay", "ray", "ke", " ", "p", " ", "ne", " ", "na", "!"
        ]
    )
}

#[test]
fn it_handles_accent_symbols_as_well() {
    assert_eq!(syllabicate("káni"), ["ká", "ni"])
}
