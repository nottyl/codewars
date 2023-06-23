fn two_sort(arr: &[&str]) -> String {
    let mut v = arr.to_vec();
    let mut result = String::new();
    v.sort();
    if let Some(first_element) = v.get(0) {
        let len = first_element.len();
        for (i, c) in first_element.chars().enumerate() {
            result += &c.to_string();
            if i != len - 1 {
                result += "***";
            }
        }
    }
    result
}

fn main() {
    let arr = ["apple", "mela", "zucchero"];
    let result = two_sort(&arr);
    println!("{result}");
}
