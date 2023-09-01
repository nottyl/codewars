fn find_average(slice: &[f64]) -> f64 {
    if slice.is_empty() {
        0.0
    }
    else {
        let sum: f64 = slice.iter().sum();
        let avg: f64 = sum / slice.len() as f64;
        avg
    }
}

fn main() {
    println!("{}", find_average(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]));
}
