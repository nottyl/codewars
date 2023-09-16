fn get_count(string: &str) -> usize {
    let mut vowels_count: usize = 0;
    for i in string.chars() {
        match i {
            'a' | 'e' | 'i' | 'o' | 'u' => vowels_count += 1,
            _ => {}
        }
    }
    vowels_count
}

fn main() {
    println!("Hello, world!");
}
