use std::collections::HashSet;

pub fn solver() {
    let mut longest_cycle = 0;
    let mut number = 0;

    for i in 2..1000 {
        let l = fraction_length(i);
        if l > longest_cycle {
            longest_cycle = l;
            number = i;
        }
    }

    println!("Problem 26 - Reciprocal Cycles: {}", number);
}

fn fraction_length(n: u64) -> usize {
    let mut digits: Vec<u64> = vec![];
    let mut remainders: HashSet<u64> = HashSet::new();
    let mut remainder = 1 % n;

    while remainder != 0 {
        if remainders.contains(&remainder) {
            break;
        }
        else {
            remainders.insert(remainder);
        }

        remainder *= 10;

        let digit = remainder / n;
        digits.push(digit);

        remainder %= n;
    }

    digits.len()
}