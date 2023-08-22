// Link: https://www.codewars.com/kata/53af2b8861023f1d88000832/rust

fn are_you_playing_banjo(name: &str) -> String {
    let initial = name.chars().next(); // Retrieve the first character of the name
    match initial {
        Some('R') | Some('r') => format!("{} plays banjo", name),
        _ => format!("{} does not play banjo", name),
    }
}

fn main() {
    println!("{}", are_you_playing_banjo("Teewhy"));
}
