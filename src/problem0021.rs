use std::collections::HashMap;

pub fn solver() {
    let mut result = 0;
    let mut lookup_table: HashMap<u64, u64> = HashMap::new();

    for i in 1..=10000 {
        let div_sum = divisor_sum(i);

        if lookup_table.contains_key(&div_sum) {
            if lookup_table[&div_sum] == i {
                result += div_sum;
                result += i;
            }
        }

        lookup_table.insert(i, div_sum);
    }

    println!("Problem 0021 - Amicable Numbers: {}", result);
}

fn divisor_sum(n: u64) -> u64 {
    let mut sum = 0;
    let sqrt_n = (n as f64).sqrt() as u64;

    for i in 1..=sqrt_n {
        if n % i == 0 {
            sum += i;
            sum += n / i;
        }
    }

    sum -= n;

    if sqrt_n * sqrt_n == n {
        sum -= sqrt_n;
    }

    sum
}
