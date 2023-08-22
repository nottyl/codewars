fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if input == [] {
        [].to_vec()
    }
    else{
        let mut positive_count = 0;
        let mut negative_sum = 0;
        for i in input.iter() {
            if i > &0 {
                positive_count += 1;
            }
            else if i < &0 {
                negative_sum += i;
            }
        }
        [positive_count, negative_sum].to_vec()
    }
}

fn main() {
    println!("Hahahhahaha");
}
