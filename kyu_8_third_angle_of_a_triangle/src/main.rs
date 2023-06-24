// Link: https://www.codewars.com/kata/57cfdf34902f6ba3d300001e

fn other_angle(a: u32, b: u32) -> u32 {
    180 - a - b
}

fn main() {
    println!("{}", other_angle(60, 30));
}
