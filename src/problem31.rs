pub fn solver() {
    let coins: [usize; 8] = [1, 2, 5, 10, 20, 50, 100, 200];
    let solution = coin_sums(200, coins);

    println!("Problem 31 - Coin Sums: {}", solution)
}

fn coin_sums(total: usize, coins: [usize; 8]) -> u64 {
    let mut solutions: Vec<u64> = vec![0; total+1];
    solutions[0] = 1;

    for coin in coins {
        for i in coin..total+1 {
            solutions[i] += solutions[i-coin];
        }
    }

    solutions[total]
}