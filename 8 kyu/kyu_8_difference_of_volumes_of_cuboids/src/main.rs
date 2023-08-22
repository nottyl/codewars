// Link: https://www.codewars.com/kata/58cb43f4256836ed95000f97/

fn find_difference(a: &[i32; 3], b: &[i32; 3]) -> i32 {
    let a_value = a[0] * a[1] * a[2];
    let b_value = b[0] * b[1] * b[2];
    (a_value - b_value).abs()
}

fn main() {
    println!("{}", find_difference(&[5, 12, 13], &[3, 4, 5]));
    println!("{}", find_difference(&[3, 4, 5], &[5, 12, 13]));
}
