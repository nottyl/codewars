fn bool_to_word(value: bool) -> &'static str {
    match value {
        true => "Yes",
        _ => "No"
    }
}

fn main() {
    let value = true;
    let ans = bool_to_word(value);
    println!("{}", ans);
}
