fn invert(values: &[i32]) -> Vec<i32> {
    let mut inverted = Vec::new();
    for number in values.iter() {
        inverted.push(-*number);
    }
    inverted
}

fn main() {
    println!("Hello, world!");
}
