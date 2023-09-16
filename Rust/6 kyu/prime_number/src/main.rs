fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    for a in 2..=(n as f64).sqrt() as i32 {
        if n % a == 0 {
            return false;
        }
    }
    true
}

#[test]
fn not_prime_tests() {
    assert!(!is_prime(4), "4 is not prime");
    assert!(!is_prime(6), "6 is not prime");
    assert!(!is_prime(8), "8 is not prime");
    assert!(!is_prime(9), "9 is not prime");
    assert!(!is_prime(45), "45 is not prime");
    assert!(!is_prime(-5), "-5 is not prime");
    assert!(!is_prime(-8), "-8 is not prime");
    assert!(!is_prime(-41), "-41 is not prime");
}

fn main(){}
