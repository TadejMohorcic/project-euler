pub fn solver() {
    let result = number_of_valid_sequences(30);
    println!("Problem 0191 - Prize Strings: {}", result);
}

fn number_of_valid_sequences(n: usize) -> u32 {
    let mut result = 0;
    let sequences = ab_sequence(n);

    result += sequences[n];

    for i in 0..(n+1) / 2 {
        if i == n-1-i {
            result += sequences[i] * sequences[n-1-i];
        }
        else {
            result += 2 * sequences[i] * sequences[n-1-i];
        }
    }

    result
}

fn ab_sequence(n: usize) -> Vec<u32> {
    let mut ab_sequences = Vec::new();

    ab_sequences.push(1);
    ab_sequences.push(2);
    ab_sequences.push(4);

    for i in 3..=n {
        ab_sequences.push(ab_sequences[i-1] + ab_sequences[i-2] + ab_sequences[i-3]);
    }

    ab_sequences
}