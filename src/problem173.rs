pub fn solver() {
    let result = odd_even_pairs(1000000.0);
    println!("Problem 173 solution: {}", result);
}

fn odd_even_pairs(upper_limit: f64) -> u64 {
    let mut number_of_pairs: u64 = 0;

    let mut start = 1.0;
    let mut difference: f64 = 10.0;

    while difference > 1.0 {
        let end = (start * start + upper_limit).sqrt();

        difference = end - start;

        let mut start_int = start as u64;
        let mut end_int = end.floor() as u64;

        if start_int % 2 == 1 {
            start_int += 1;
            
            if end_int % 2 == 1 {
                end_int += 1;
            }
        }
        else {
            if end_int % 2 == 1 {
                end_int -= 1;
            }
        }

        number_of_pairs += (end_int - start_int) / 2;
        start += 1.0;
    }

    number_of_pairs
}