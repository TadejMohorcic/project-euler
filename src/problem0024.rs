pub fn solver() {
    let result = get_ith_permutation(999999, 9);
    println!("Problem 0024 - Lexicographic Permutations: {}", result);
}

fn factorial(n: usize) -> u64 {
    let mut result = 1;

    for i in 1..=n {
        result *= i as u64;
    }

    result
}

fn get_ith_permutation(mut i: u64, number_of_digits: usize) -> u64 {
    let mut result = 0;
    let mut digits: Vec<u64> = (0..=number_of_digits).map(|x| x as u64).collect();

    for j in (0..=number_of_digits).rev() {
        let fac_j = factorial(j);
        let position = i / fac_j;
        i -= position * fac_j;

        let next_digit = digits[position as usize];
        result = result * 10 + next_digit;

        if let Some(index) = digits.iter().position(|x| *x == next_digit) {
            digits.remove(index);
        }
    }
    
    result
}