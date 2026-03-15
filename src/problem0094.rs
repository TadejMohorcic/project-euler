pub fn solver() {
    let mut result = 0;

    for i in 2..=500_000_000 {
        let min_perimeter = 3 * i - 1;

        if min_perimeter > 1_000_000_000 {
            break;
        }

        result += integer_area(i);
    }

    println!("Problem 0094 - Almost Equilateral Triangles: {}", result);
}

fn integer_area(side_length: u128) -> u128 {
    let mut result = 0;

    let perimeter_p = 3 * side_length + 1;
    let perimeter_m = 3 * side_length - 1;

    if let Some(r) = is_square(perimeter_p * (side_length - 1)) {
        let area = (side_length + 1) * r;
        if area % 4 == 0 {
            result += perimeter_p
        }
    }

    if let Some(r) = is_square(perimeter_m * (side_length + 1)) {
        let area = (side_length - 1) * r;
        if area % 4 == 0 {
            result += perimeter_m;
        }
    }

    result
}

fn is_square(n: u128) -> Option<u128> {
    let r = (n as f64).sqrt() as u128;

    if r * r == n {
        Some(r)
    } else if (r + 1) * (r + 1) == n {
        Some(r + 1)
    } else {
        None
    }
}
