use std::cmp::max;

use itertools::Itertools;

pub fn solver() {
    let n: u64 = 10;
    let mut result = 0;

    for permutation in (1..=n).permutations(n as usize) {
        if is_valid_ngon(&permutation) {
            let number = generate_numstring(&permutation);

            if length_16(number) {
                result = max(result, number);
            }
        }
    }

    println!("Problem 0068 - Magic 5-gon Ring: {}", result)
}

fn is_valid_ngon(sequence: &Vec<u64>) -> bool {
    let n = sequence.len() / 2;
    let mut previous_sum: Option<u64> = None;

    for i in 0..n {
        let sum;

        if i == n - 1 {
            sum = sequence[0] + sequence[i] + sequence[n + i];
        } else {
            sum = sequence[i..i + 2].iter().fold(0, |acc, x| acc + x) + sequence[n + i];
        }

        match previous_sum {
            Some(s) => {
                if sum != s {
                    return false;
                }
            }
            None => previous_sum = Some(sum),
        }
    }

    true
}

fn generate_numstring(sequence: &Vec<u64>) -> u64 {
    let n = sequence.len() / 2;
    let i = n + sequence[n..]
        .iter()
        .enumerate()
        .min_by_key(|(_, val)| **val)
        .map(|(idx, _)| idx)
        .unwrap();
    let mut result = 0;

    for j in 0..n {
        let first_id = n + (i - n + j) % n;
        let second_id = first_id - n;
        let third_id = (second_id + 1) % n;

        let to_add = [sequence[first_id], sequence[second_id], sequence[third_id]];

        for num in to_add {
            if num > 9 {
                result = result * 100 + num;
            } else {
                result = result * 10 + num;
            }
        }
    }

    result
}

fn length_16(n: u64) -> bool {
    let mut number = n;
    let mut number_len = 0;

    while number > 0 {
        let d = number % 10;
        number = (number - d) / 10;
        number_len += 1
    }

    number_len == 16
}
