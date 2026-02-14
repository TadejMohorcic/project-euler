pub fn solver() {
    let m = 2000;
    let mut result = 0;

    for i in 1..=m {
        for j in 2..=2*i {
            if shortest_int(i, j) {
                if j > i + 1 {
                    let combinations = ((2 * i - j + 2) as f64 / 2.0).floor();
                    result += combinations as u64;
                }
                else {
                    let combinations = (j as f64 / 2.0).floor();
                    result += combinations as u64;
                }
            }
        }
        if result as f64 > 1e6 {
            println!("Problem 86 - Cuboid Route: {}", i);
            break
        }
    }
}

fn shortest_int(x: u64, yz: u64) -> bool {
    let distance = x.pow(2) + yz.pow(2);
    let distance_f = distance as f64;
    let distance_sqrt = distance_f.sqrt();

    distance_sqrt.fract() == 0.0
}