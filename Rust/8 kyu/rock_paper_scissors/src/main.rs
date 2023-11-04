fn rps(p1: &str, p2: &str) -> String {
    let mut result = String::new();
    match (p1, p2) {
        ("rock", "paper") => result = "Player 2 won!".to_string(),
        ("rock", "scissors") => result = "Player 1 won!".to_string(),
        ("paper", "scissors") => result = "Player 2 won!".to_string(),
        ("paper", "rock") => result = "Player 1 won!".to_string(),
        ("scissors", "rock") => result = "Player 2 won!".to_string(),
        ("scissors", "paper") => result = "Player 1 won!".to_string(),
        (&_, _) => result = "Draw!".to_string(),
    }
    result
}

// // Best Practice
// fn rps(p1: &str, p2: &str) -> &'static str {
//     if (p1 == p2) {
//         return "Draw!";
//     }
//     match (p1, p2) {
//         ("scissors", "paper") | ("paper", "rock") | ("rock", "scissors") => "Player 1 won!",
//         _ => "Player 2 won!",
//     }
// }

#[cfg(test)]
mod tests {
    use super::rps;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(p1: &str, p2: &str, expected: &str) {
        assert_eq!(
            rps(p1, p2),
            expected,
            "{ERR_MSG} with p1 = \"{p1}\", p2 = \"{p2}\""
        )
    }

    #[test]
    fn fixed_tests() {
        dotest("rock", "scissors", "Player 1 won!");
        dotest("scissors", "rock", "Player 2 won!");
        dotest("rock", "rock", "Draw!");
    }
}

fn main() {}
