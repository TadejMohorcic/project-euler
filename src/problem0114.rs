use std::cmp::{min, max};

pub fn solver() {
    let a = count_block_combos(50);
    println!("Problem 0114 - Counting Block Combinations I: {}", a);
}

pub fn count_block_combos(n: usize) -> u64 {
let mut combinations: Vec<u64> = [1, 1, 1, 2].to_vec();

    for i in 4..=n {
        let mut new_combinations = combinations[i-1];
        let upper_bound = min(3, i-2);

        for j in 0..upper_bound {
            new_combinations += combinations[j];
        }

        for j in 2..i-3 {
            new_combinations += combinations[j];
        }

        combinations.push(new_combinations);
    }

    combinations[n]
}