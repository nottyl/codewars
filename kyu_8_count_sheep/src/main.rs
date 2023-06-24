// Link: https://www.codewars.com/kata/5b077ebdaf15be5c7f000077

fn count_sheep(n: u32) -> String {
    let mut str = String::new();
    for i in 1..n+1 {
        str += &i.to_string();
        str += " sheep...";
    }
    str
}

fn main() {
    let n = 4;
    println!("{}", count_sheep(n));
}
