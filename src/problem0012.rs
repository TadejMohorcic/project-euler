pub fn solver() {
    let mut triangular_number = 0;
    let mut i = 1;
    let target = 500;

    loop {
        triangular_number += i;
        let divisors = get_divisors(triangular_number);
        if divisors > target {
            println!(
                "Problem 0012 - Highly Divisible Triangular Number: {}",
                triangular_number
            );
            break;
        }
        i += 1;
    }
}

fn _triangular_number(n: u64, previous: u64) -> u64 {
    previous + n
}

fn get_divisors(n: u64) -> usize {
    let mut divisors = 0;
    let sqrt_n: u64 = (n as f64).sqrt() as u64;

    for i in 1..=sqrt_n {
        if n % i == 0 {
            divisors += 2;
        }
    }

    if sqrt_n * sqrt_n == n {
        divisors -= 1;
    }

    divisors
}
