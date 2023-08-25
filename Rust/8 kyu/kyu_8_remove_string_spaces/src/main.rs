// Link: https://www.codewars.com/kata/57eae20f5500ad98e50002c5/rust

fn no_space(x : String) -> String{
    x.split_whitespace().collect()
}

fn main() {
    println!("Hello World = {}", no_space("Hello World".to_string()));
}
