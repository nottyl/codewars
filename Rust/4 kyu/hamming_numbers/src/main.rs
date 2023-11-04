fn hamming(n: usize) -> u64 {
    
}

#[cfg(test)]
mod tests {
    use super::hamming;

    #[test]
    fn sample_tests() {
        assert_eq!(hamming(1), 1);
        assert_eq!(hamming(2), 2);
        assert_eq!(hamming(3), 3);
        assert_eq!(hamming(4), 4);
        assert_eq!(hamming(5), 5);
        assert_eq!(hamming(6), 6);
        assert_eq!(hamming(7), 8);
        assert_eq!(hamming(8), 9);
        assert_eq!(hamming(9), 10);
        assert_eq!(hamming(10), 12);
        assert_eq!(hamming(11), 15);
        assert_eq!(hamming(12), 16);
        assert_eq!(hamming(13), 18);
        assert_eq!(hamming(14), 20);
        assert_eq!(hamming(15), 24);
        assert_eq!(hamming(16), 25);
        assert_eq!(hamming(17), 27);
        assert_eq!(hamming(18), 30);
        assert_eq!(hamming(19), 32);
    }
}

fn main() {}
