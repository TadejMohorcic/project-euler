pub fn solver() {
    let (c, d) = dics_probability(15);
    let result = d / c;
    
    println!("Problem 121 - Disc Game Prize Fund: {}", result);
}

fn dics_probability(n: u64) -> (u64, u64) {
    let mut numerator: u64 = 1;

    for i in 1..=(n as usize - 1)/2 {
        numerator += get_combinations(n, i as u64);
    }

    let denominator: u64 = (1..=n+1).into_iter().fold(1, |acc, x| acc * x);

    (numerator, denominator)
}

fn get_combinations(n: u64, k: u64) -> u64 {
    let mut result = 0;
    let mut window: Vec<u64> = (1..=k).collect();

    loop {
        let combination_fold: u64 = window.iter().fold(1, |acc, x| acc * x);
        result += combination_fold;

        let mut i = k;

        while i > 0 && window[i as usize - 1] == n - k + i {
            i -= 1;
        }

        if i == 0 {
            break;
        }

        window[i as usize - 1] += 1;
        for j in i..k {
            window[j as usize] = window[j as usize - 1] + 1;
        }
    }

    result
}