use ainu_utils::datetime::datetime;

fn main() {
    let formatted_date = datetime::format(2000, 10, 31, 12, 30, 0);

    println!("{}", formatted_date);
}
