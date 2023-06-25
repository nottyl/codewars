// Link: https://www.codewars.com/kata/58649884a1659ed6cb000072/

fn update_light(current: &str) -> String {
    match current {
        "red" => "green".to_string(),
        "yellow" => "red".to_string(),
        "green" => "yellow".to_string(),
        &_ => unimplemented!(),
    }
}

fn main() {
    println!("{}", update_light("red"));
}
