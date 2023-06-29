// Link: https://www.codewars.com/kata/56747fd5cb988479af000028/rust

fn get_middle(s:&str) -> &str {
    let len = s.len();
    if len % 2 != 0 {
        &s[len/2..=len/2]
    }
    else {
        &s[len/2-1..=len/2]
    }
}

fn main() {
    println!("{}", get_middle("Love"));
}
