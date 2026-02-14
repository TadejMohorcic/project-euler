pub fn solver() {
    let max_size = 1_000_000;
    let result = combinations(50, max_size);
    println!("Problem 0115 - Counting Block Combinations II: {}", result);
}

fn combinations(m: usize, max_size: u64) -> usize {
    let mut combinations: Vec<u64> = vec![1; m];
    let mut j = 0;
    let mut i = m;

    loop {
        if j < m {
            let new_combination = combinations[i-1] + (j as u64) + 1;
            combinations.push(new_combination);
            j += 1;
        }
        else {
            let index_to = i - m;
            let new_combination = combinations[i-1] + combinations[m-1..index_to].iter().fold(0, |acc, x| acc + x) + (m as u64);
            combinations.push(new_combination);
        }

        if let Some(x) = combinations.last() {
            if *x > max_size {
                return i;
            }
        }

        i += 1;
    }
}