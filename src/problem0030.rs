pub fn solver() {
    let mut result = 0;
    let exp = 5;

    for i in 2..=200000 {
        let digits = get_digits(i);
        let digits_pow_sum: u64 = digits.iter().fold(0, |acc, x| acc + x.pow(exp));
        if i == digits_pow_sum {
            result += i;
        }
    }

    println!("Problem 0030 - Digit Fifth Powers: {}", result);
}

fn get_digits(number: u64) -> Vec<u64> {
    let mut num_copy = number;
    let mut digits: Vec<u64> = vec![];

    while num_copy != 0 {
        let d = num_copy % 10;
        num_copy = num_copy / 10;
        digits.push(d);
    }

    digits.reverse();
    digits
}
