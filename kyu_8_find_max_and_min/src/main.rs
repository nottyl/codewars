// Link: https://www.codewars.com/kata/577a98a6ae28071780000989

fn minimum(arr: &[i32]) -> i32 {
    let v = arr.to_vec();
    *v.iter().min().unwrap()
}

fn maximum(arr: &[i32]) -> i32 {
    let v = arr.to_vec();
    *v.iter().max().unwrap()
}

fn main() {
    let arr = [1, 3, 5, 4, 2, 6, 7];
    println!("{}", maximum(&arr));
    println!("{}", minimum(&arr));
}
