pub fn solver() {
    let mut result = 0.0;

    for i in 1..=1000 {
        let r = exponent_modulo(i as f64, i, 10000000000.0);
        result = (result + r) % 10000000000.0;
    }

    println!("Problem 0048 - Self Powers: {}", result)
}

fn exponent_modulo(n: f64, pow: i64, modulo: f64) -> f64 {
    let mut res = 1.0;
    let mut p = 0;

    while p < pow {
        res = (res * n) % modulo;
        p += 1
    }

    res
}