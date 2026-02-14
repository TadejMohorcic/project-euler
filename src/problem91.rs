pub fn solver() {
    let size = 50;

    let mut solutions = 3 * size * size;

    for i in 1..=size {
        for j in 1..=size {
            let sol = integer_solutions(i, j, size);
            solutions += sol;
        }
    }

    println!("Problem 91 solution: {}", solutions);
}

fn integer_solutions(a: i64, b: i64, max_size: i64) -> i64 {
    let mut num_of_solutions = 0;
    let g = gcd(a, b);
    let normal_vec = (-b / g, a / g);
    
    let mut a_neg = a;
    let mut b_neg = b;
    let mut a_pos = a;
    let mut b_pos = b;

    while b_neg >= 0 {
        a_neg -= normal_vec.0;
        b_neg -= normal_vec.1;

        if 0 <= a_neg && a_neg <= max_size && 0 <= b_neg && b_neg <= max_size {
            num_of_solutions += 1;
        }
    }

    while a_pos >= 0 {
        a_pos += normal_vec.0;
        b_pos += normal_vec.1;

        if 0 <= a_pos && a_pos <= max_size && 0 <= b_pos && b_pos <= max_size {
            num_of_solutions += 1;
        }
    }

    num_of_solutions
}

fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 {
        return b
    }
    else {
        return gcd(b % a, a)
    }
}