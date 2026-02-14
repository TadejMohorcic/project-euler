pub fn solver() {
    let result = dice_probability(9, 6);
    
    println!("Problem 0205 - Dice Game: {}", result)
}

fn dice_probability(four_sided_throws: usize, six_sided_throws: usize) -> f64 {
    let mut probability = 0.0;

    let mut count_four: Vec<u64> = vec![0; 3];
    let mut count_six: Vec<u64> = vec![0; 5];

    count_four.extend(vec![1; 4]);
    count_four.extend(vec![0; 3]);

    count_six.extend(vec![1; 6]);
    count_six.extend(vec![0; 5]);

    for _ in 1..four_sided_throws {
        let n = count_four.len() - 3;
        let mut new_count_four: Vec<u64> = vec![0; 3];

        for i in 0..n {
            let sliding_window = &count_four[i..i+4];
            let sum_window: u64 = sliding_window.iter().sum();

            new_count_four.push(sum_window);
        }

        new_count_four.extend(vec![0; 3]);
        count_four = new_count_four;
    }

    for _ in 1..six_sided_throws {
        let n = count_six.len() - 5;
        let mut new_count_six: Vec<u64> = vec![0; 5];

        for i in 0..n {
            let sliding_window = &count_six[i..i+6];
            let sum_window: u64 = sliding_window.iter().sum();

            new_count_six.push(sum_window);
        }

        new_count_six.extend(vec![0; 5]);
        count_six = new_count_six
    }

    let unpadded_four = &count_four[3..count_four.len()-3];
    let four_total: u64 = count_four.iter().sum();
    let four_probability: Vec<f64> = unpadded_four.iter().map(|x| *x as f64 / four_total as f64).collect();

    let unpadded_six = &count_six[5..count_six.len()-5];
    let six_total: u64 = count_six.iter().sum();
    let six_probability: Vec<f64> = unpadded_six.iter().map(|x| *x as f64 / six_total as f64).collect();

    let difference = four_sided_throws - six_sided_throws;

    let m = six_probability.len();

    for i in 0..four_probability.len() {
        let mut probability_acc = 0.0;

        let upper_range;

        if i+difference > m {
            upper_range = m;
        }
        else {
            upper_range = i+difference;
        }

        for j in 0..upper_range {
            probability_acc += six_probability[j];
        }

        probability += four_probability[i] * probability_acc;
    }

    probability
}