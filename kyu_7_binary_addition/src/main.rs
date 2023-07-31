// Link: https://www.codewars.com/kata/551f37452ff852b7bd000139/rust

fn add_binary(a: u64, b: u64) -> String {
    let sum = a + b;
    let bin = String::from(format!("{sum:b}"));
    bin
}

fn main() {
    println!("Hello, world!");
}
