pub fn solver() {
    let target: f64 = 2e6;

    let mut shortest_distance: f64 = f64::MAX;
    let mut area: u64 = 0;

    for i in 2..=2000 {
        for j in 2..=i {
            let rectangles = count_rectangles(i, j);
            let distance_abs: f64 = (rectangles as f64 - target).abs();

            if distance_abs < shortest_distance {
                shortest_distance = distance_abs;
                area = i * j;
            }
        }
    }

    println!("Problem 0085 - Counting Rectangles: {}", area);
}

fn count_rectangles(x: u64, y: u64) -> u64 {
    let rectangles: u64 = ((x + 1) * x * (y + 1) * y) / 4;

    rectangles
}
