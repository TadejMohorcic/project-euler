pub fn solver() {
    let result = combinations(50);
    println!("Problem 0117 - Red, Green, and Blue Tiles: {}", result);
}

fn combinations(n: usize) -> u64 {
    let mut combinations: Vec<u64> = [1, 1, 2, 4].to_vec();
    
    for i in 4..=n {
        let new_combination = combinations[i-4..=i-1].iter().fold(0, |acc, x| acc + x);
        combinations.push(new_combination);
    }

    combinations[n]
}