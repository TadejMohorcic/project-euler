pub fn solver() {
    let mut numerator = 1;
    let mut denominator = 1;

    for i in 10..=100 {
        for j in i+1..=100 {
            let digits_i = get_digits(i);
            let digits_j = get_digits(j);

            match (digits_i, digits_j) {
                ([x0, x1], [y0, y1]) if x0 == y0 => {
                    if (x1 as f64 / y1 as f64) == (i as f64) / (j as f64) {
                        if x0 != 0 {
                            numerator *= x1;
                            denominator *= y1;
                        }
                    }
                },
                ([x0, x1], [y0, y1]) if x0 == y1 => {
                    if (x1 as f64 / y0 as f64) == (i as f64) / (j as f64) {
                        if x0 != 0 {
                            numerator *= x1;
                            denominator *= y0;
                        }
                    }
                },
                ([x0, x1], [y0, y1]) if x1 == y0 => {
                    if (x0 as f64 / y1 as f64) == (i as f64) / (j as f64) {
                        if x1 != 0 {
                            numerator *= x0;
                            denominator *= y1;
                        }
                    }
                },
                ([x0, x1], [y0, y1]) if x1 == y1 => {
                    if (x0 as f64 / y0 as f64) == (i as f64) / (j as f64) {
                        if x1 != 0 {
                            numerator *= x0;
                            denominator *= y0;
                        }
                    }
                },
                _ => continue
            }
        }
    }

    let q = gcd(numerator, denominator);

    println!("Problem 33 - Digit Cancelling Fractions: {}", denominator / q);
}

fn get_digits(n: u64) -> [u64; 2] {
    let mut digits: [u64; 2] = [0; 2];

    digits[0] = n / 10;
    digits[1] = n % 10;

    digits
}

fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 {
        return b
    }
    else {
        return gcd(b % a, a)
    }
}