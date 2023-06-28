// Link: https://www.codewars.com/kata/544675c6f971f7399a000e79/rust

fn string_to_number(s: &str) -> i32 {
    s.to_string().parse::<i32>().unwrap()
}

fn main() {
    println!("{}", string_to_number(&"1234"));
}
