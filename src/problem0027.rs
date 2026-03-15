pub fn solver() {
    let mut longest_chain = 0;
    let mut product = 0;

    for i in 2..=1000 {
        for j in -1000..=1000 {
            if is_prime(i) && is_prime(i + j + 1) {
                let mut n = 2;
                let mut current = i + j + 1;

                while is_prime(current) {
                    current = n * n + n * j + i;
                    n += 1
                }

                if n > longest_chain {
                    longest_chain = n;
                    product = i * j;
                }
            }
        }
    }

    println!("Problem 0027 - Quadratic Primes: {}", product);
}

fn is_prime(n: i64) -> bool {
    if n <= 0 {
        return false;
    } else {
        let root: i64 = (n as f64).sqrt() as i64;
        for i in 2..=root {
            if n % i == 0 {
                return false;
            }
        }
        return true;
    }
}
