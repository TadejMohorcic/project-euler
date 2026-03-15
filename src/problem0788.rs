pub fn solver() {
    let mut total = 0;
    let modulo = 1000000007;

    for i in 1..=2022 {
        for k in 0..=(i - 1) / 2 {
            let current_total =
                modular_exponent(9, k + 1, modulo) * modular_binomial_coefficient(i, k, modulo);
            total = (total + current_total) % modulo;
        }
    }

    println!("Problem 0788 - Dominating Numbers: {}", total);
}

fn modular_exponent(base: u64, exponent: u64, modulo: u64) -> u64 {
    let mut result = 1;
    let mut p = 0;

    while p < exponent {
        result = (result * base) % modulo;
        p += 1;
    }

    result
}

fn modular_binomial_coefficient(n: u64, k: u64, modulo: u64) -> u64 {
    if k == 0 {
        return 1;
    } else if k > n / 2 {
        return modular_binomial_coefficient(n, n - k, modulo);
    } else {
        return (n * modular_binomial_coefficient(n - 1, k - 1, modulo) / k) % modulo;
    }
}
