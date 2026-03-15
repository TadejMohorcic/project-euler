pub fn solver() {
    let mut result = 0;

    for i in 1..100000000 {
        result += is_concat(i);
    }

    println!("Problem 0932 - 2025: {}", result);
}

fn calculate_discriminant(b: i64, n: u32) -> i64 {
    let discriminant = 4 * b * (1 - 10_i64.pow(n)) + 10_i64.pow(2 * n);
    discriminant
}

fn is_concat(b: i64) -> i64 {
    let n = b.checked_ilog10().unwrap_or(0) + 1;
    let p = 10_i64.pow(n);

    let d = calculate_discriminant(b, n as u32);

    if d < 0 {
        return 0;
    } else if d == 0 {
        let a = (p - 2 * b) / 2;
        let m = a * p + b;
        let mut result = 0;

        if (a + b) * (a + b) == m {
            result += m;
        }

        return result;
    } else {
        let a1 = (p - 2 * b + d.isqrt()) / 2;
        let a2 = (p - 2 * b - d.isqrt()) / 2;
        let m1 = a1 * p + b;
        let m2 = a2 * p + b;
        let mut result = 0;

        if (a1 + b) * (a1 + b) == m1 && a1 != 0 {
            result += m1;
        }

        if (a2 + b) * (a2 + b) == m2 && a2 != 0 {
            result += m2;
        }

        return result;
    }
}
