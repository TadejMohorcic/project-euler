use std::collections::HashMap;

pub fn solver() {
    let result = caluculate_rounts(20);
    println!("Problem 15 - Lattice Paths: {}", result);
}

fn caluculate_rounts(n: usize) -> u64 {
    let mut grid: HashMap<(usize, usize), u64> = HashMap::new();

    for i in 0..=n {
        grid.insert((0, i), 1);
    }

    for i in 1..=n {
        for j in i..=n {
            let new_value;

            if i == j {
                let grid_above = *grid.get(&(i-1, j)).expect("Missing key!");
                new_value = 2 * grid_above;
            }
            else {
                let grid_above = *grid.get(&(i-1, j)).expect("Missing key!");
                let grid_left = *grid.get(&(i, j-1)).expect("Missing key!");
                new_value = grid_above + grid_left;
            }

            grid.insert((i, j), new_value);
        }   
    }

    let number_of_paths = *grid.get(&(n, n)).expect("Missing key!");
    number_of_paths
}