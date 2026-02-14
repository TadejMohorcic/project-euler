use std::fs::File;
use std::io::{self, BufRead, BufReader};

use std::cmp::max;

fn calculate_product(grid: &Vec<Vec<u64>>) -> u64 {
    let mut product: u64 = 0;
    let m = grid.len();
    let n = grid[0].len();

    for i in 0..m {
        for j in 0..n {
            let current = grid[i][j];

            let mut prod_r: u64 = 0;
            let mut prod_rd: u64 = 0;
            let mut prod_d: u64 = 0;
            let mut prod_ld: u64 = 0;

            if j < n-4 {
                prod_r = current * grid[i][j+1] * grid[i][j+2] * grid[i][j+3];
            }

            if i < m-3 {
                prod_d = current * grid[i+1][j] * grid[i+2][j] * grid[i+3][j];

                if j < n-4 {
                    prod_rd = current * grid[i+1][j+1] * grid[i+2][j+2] * grid[i+3][j+3];
                }

                if j > 2 {
                    prod_ld = current * grid[i+1][j-1] * grid[i+2][j-2] * grid[i+3][j-3];
                }
            }

            product = max(product, max(prod_r, max(prod_d, max(prod_rd, prod_ld))));
        }
    }

    product
}

pub fn solver() -> io::Result<()> {
    let path = "input/11.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<u64>> = vec![];

    for line in reader.lines() {
        let ok_line = line?;
        let line_vec: Vec<u64> = ok_line
            .split_whitespace()
            .map(|s| s.parse::<u64>())
            .collect::<Result<_, _>>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        grid.push(line_vec);
    }

    let result = calculate_product(&grid);
    println!("Problem 11 - Largest Product in a Grid: {}", result);

    Ok(())
}