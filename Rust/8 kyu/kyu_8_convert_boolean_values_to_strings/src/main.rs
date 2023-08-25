// Link: https://www.codewars.com/kata/53369039d7ab3ac506000467

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
