// Link: https://www.codewars.com/kata/557cd6882bfa3c8a9f0000c1/rust

fn get_age(age: &str) -> u32 {
    age.chars().next().expect("Oh no!").to_digit(10).expect("Uh oh!") as u32
}

fn main() {
    println!("{}", get_age("5 years old."));
}
