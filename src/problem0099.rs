use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn solver() -> io::Result<()> {
    let path = "input/99.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut highest_value: f64 = 0.0;
    let mut row_highest: u64 = 0;

    let mut current_row: u64 = 1;

    for line in reader.lines() {
        let ok_line = line?;
        let mut line_vec = ok_line.split(',');
        let base: f64 = line_vec
            .next()
            .expect("missing base")
            .trim()
            .parse()
            .expect("invalid base");
        let exponent: f64 = line_vec
            .next()
            .expect("missing exponent")
            .trim()
            .parse()
            .expect("invalid exponent");

        let value = exponent * base.ln();

        if value > highest_value {
            highest_value = value;
            row_highest = current_row;
        }

        current_row += 1;
    }

    println!("Problem 0099 solution: {}", row_highest);

    Ok(())
}
