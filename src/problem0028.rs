pub fn solver() {
    let result = sum_of_diagonals(1001);
    println!("Problem 0028 - Number Spiral Diagonals: {}", result)
}

fn sum_of_diagonals(mut size: u64) -> u64 {
    let mut result = 1;

    while size > 1 {
        result += 2 * ( 2 * size * size - 3 * size + 3);
        size -= 2;
    }

    result
}