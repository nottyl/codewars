// Link: https://www.codewars.com/kata/568dcc3c7f12767a62000038

fn set_alarm(employed: bool, vacation: bool) -> bool {
    match (employed, vacation) {
        (true, true) => false,
        (true, false) => true,
        (false, true) => false,
        (false, false) => false,
    }
}

fn main() {
    let if_true = set_alarm(true, false);
    println!("{if_true}");
}
