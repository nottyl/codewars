// Link: https://www.codewars.com/kata/55f2b110f61eb01779000053/rust

fn get_sum(a: i64, b: i64) -> i64 {
    match a < b {
        true => (a..=b).sum(),
        false => (b..=a).sum(),
    }
}

fn main() {
    println!("{}", get_sum(5, -1));
}
