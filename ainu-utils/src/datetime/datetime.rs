pub fn format(year: i32, month: i32, day: i32, hour: i32, minute: i32, second: i32) -> String {
    let mut output = String::new();

    output += &format!("{}-pa", year);
    output += " ";

    output += match month {
        1 => "toetanne",
        2 => "kuekay",
        3 => "kiwtacup",
        4 => "mocup",
        5 => "sincicup",
        6 => "mawtacup",
        7 => "mawcicup",
        8 => "haprap",
        9 => "nihorak",
        10 => "urepok",
        11 => "ruwekaricup",
        12 => "curup",
        _ => panic!("month must be in between 1 to 12"),
    };
    output += " ";

    output += &format!("{}-to", day);
    output += " ";

    output += &format!("{}-ci", hour);
    output += " ";

    output += &format!("{}-pun", minute);
    output += " ";

    output += &format!("{}-piyo", second);
    output += " ";

    return output;
}
