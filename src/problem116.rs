pub fn solver() {
    let result = combinations(50);
    println!("Problem 116 - Red, Green or Blue Tiles: {}", result);
}

fn combinations(n: usize) -> u64 {
    let mut combinations_a: Vec<u64> = [1, 1, 2, 3].to_vec();
    let mut combinations_b: Vec<u64> = [1, 1, 1, 2].to_vec();
    let mut combinations_c: Vec<u64> = [1, 1, 1, 1].to_vec();
    
    for i in 4..=n {
        let new_a = combinations_a[i-2..=i-1].iter().fold(0, |acc, x| acc + x);
        let new_b = combinations_b[i-1] + combinations_b[i-3];
        let new_c = combinations_c[i-1] + combinations_c[i-4];
        combinations_a.push(new_a);
        combinations_b.push(new_b);
        combinations_c.push(new_c);
    }

    combinations_a[n] + combinations_b[n] + combinations_c[n] - 3
}