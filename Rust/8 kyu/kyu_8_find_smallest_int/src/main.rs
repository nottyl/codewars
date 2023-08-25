// Link: https://www.codewars.com/kata/55a2d7ebe362935a210000b2/rust

fn find_smallest_int(arr: &[i32]) -> i32 {
    *arr.iter().min().unwrap()
}

fn main() {
    println!("{}", find_smallest_int(&[5, 6, 8, 4]));
}
