pub fn solver() {
    let mut result = 0;

    for i in 3..=1000 {
        result += calculate_remainder(i);
    }

    println!("Problem 0120 - Square Remainders: {}", result)
}

fn calculate_remainder(num: u64) -> u64 {
    let mut max_res = 0;
    let num_sq = num * num;
    let max_range = 2 * num;

    for i in (1..=max_range).step_by(2) {
        let res = (2 * i * num) % num_sq;

        if res > max_res {
            max_res = res;
        }
    }

    max_res
}
