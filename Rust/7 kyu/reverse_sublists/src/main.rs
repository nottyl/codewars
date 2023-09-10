fn rev_sub(xs: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let mut sub_start = 0;
    let mut sublist = false;
    let len = xs.len();

    for i in 0..len {
        if xs[i] % 2 == 0 {
            if !sublist {
                sub_start = i;
                sublist = true;
            }
        } else {
            if sublist {
                result.extend(xs[sub_start..i].iter().rev());
                sublist = false;
            }
            result.push(xs[i]);
        }
    }

    // If the last item is also an even number
    if sublist {
        result.extend(xs[sub_start..].iter().rev());
    }

    result
}

fn main() {
    println!("Hello, world!");
}
