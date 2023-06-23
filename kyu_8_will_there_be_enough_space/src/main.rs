fn enough(cap: i32, on: i32, wait: i32) -> i32 {
    let result = wait - (cap - on);
    if result >= 0 {
        result
    }
    else {
        0
    }
}

fn main() {
    println!("{}", {enough(100, 60, 50)});
    println!("{}", {enough(10, 5, 5)});
}
