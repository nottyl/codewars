fn likes(names: &[&str]) -> String {
    let length = names.len();
    let mut result = String::new();
    match length {
        0 => result = "no one likes this".to_string(),
        1 => result = format!("{} likes this", names[0]),
        2 => result = format!("{} and {} like this", names[0], names[1]),
        3 => result = format!("{}, {}, and {} like this", names[0], names[1], names[2]),
        _ => {
            result = format!(
                "{}, {}, and {} others like this",
                names[0],
                names[1],
                length - 2
            )
        }
    }
    result
}

// // Best Practice:
// fn likes(names: &[&str]) -> String {
//     match names {
//         [] => format!("no one likes this"),
//         [a] => format!("{} likes this", a),
//         [a, b] => format!("{} and {} like this", a, b),
//         [a, b, c] => format!("{}, {} and {} like this", a, b, c),
//         [a, b, rest @ ..] => format!("{}, {} and {} others like this", a, b, rest.len()),
//     }
// }

#[cfg(test)]
mod tests {
    use super::likes;

    #[test]
    fn example_tests() {
        assert_eq!(likes(&[]), "no one likes this");
        assert_eq!(likes(&["Peter"]), "Peter likes this");
        assert_eq!(likes(&["Jacob", "Alex"]), "Jacob and Alex like this");
        assert_eq!(
            likes(&["Max", "John", "Mark"]),
            "Max, John and Mark like this"
        );
        assert_eq!(
            likes(&["Alex", "Jacob", "Mark", "Max"]),
            "Alex, Jacob and 2 others like this"
        );
    }
}

fn main() {}
