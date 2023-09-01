fn descending_order(x: u64) -> u64 {
    let mut digits: Vec<u64> = x
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();

    digits.sort_by(|a, b| b.cmp(a));

    let mut result = 0;
    for digit in digits {
        result = result * 10 + digit;
    }

    result
}

fn main() {
    println!("Hello, world!");
}
