use std::collections::HashMap;

pub fn solver() {
    let max_red = 24690;
    let max_black = 12345;
    let mut memo: HashMap<(u64, u64), f64> = HashMap::new();

    for i in 0..=max_red {
        memo.insert((i, 0), 0.0);
    }

    for i in 1..=max_black {
        memo.insert((0, i), 1.0);
    }

    for red in (2..=max_red).step_by(2) {
        for black in 1..=max_black {
            let total = (red + black) as f64;
            let factor_rb = 1.0 - (black * (black - 1)) as f64 / (total * (total - 1.0));
            let factor_r = (red * (red - 1)) as f64 / (total * (total - 1.0));
            let factor_b = (2 * red * black) as f64 / (total * (total - 1.0));

            let probability = (factor_r * memo[&(red - 2, black)]
                + factor_b * memo[&(red, black - 1)])
                / factor_rb;

            memo.insert((red, black), probability);
        }
    }

    let prob = memo[&(max_red, max_black)];
    println!("Problem 0938 - Exhausting a Colour: {}", prob);
}
