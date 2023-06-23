fn summation(n: i32) -> i32 {
    let mut total = 0;
    for num in 0..n+1 {
        total += num;
    }
    total
}

fn main(){
    let result = summation(8);
    println!("{result}");
}

