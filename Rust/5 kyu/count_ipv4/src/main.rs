fn ips_between(start: &str, end: &str) -> u32 {
    let start_ip: Vec<u8> = start.split('.').map(|s| s.parse().unwrap()).collect();
    let end_ip: Vec<>
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(ips_between("10.0.0.0", "10.0.0.50"), 50);
        assert_eq!(ips_between("20.0.0.10", "20.0.1.0"), 246);
    }
}

fn main() {}
