#[allow(dead_code)]
fn dna_strand(dna: &str) -> String {
    let mut strand = String::new();

    for i in dna.chars() {
        match i {
            'A' => strand.push_str("T"),
            'T' => strand.push_str("A"),
            'G' => strand.push_str("C"),
            'C' => strand.push_str("G"),
            _ => strand.push_str(" "),
        }
    }
    strand
}

fn main() {
    println!("Hello, world!");
}
