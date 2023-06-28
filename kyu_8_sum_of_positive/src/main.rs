// Link: https://www.codewars.com/kata/5715eaedb436cf5606000381/rust

fn positive_sum(slice: &[i32]) -> i32 {
    let mut result = 0;
    for &num in slice {
        if num > 0 {
            result += num;
        }
    }
    result
}

fn main() {
    println!("{}", positive_sum(&[-1, 1, 2, 3]));
}
