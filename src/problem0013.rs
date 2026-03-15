use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn solver() -> io::Result<()> {
    let path = "input/0013.txt";

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut result = 0.0;

    for line in reader.lines() {
        let line = line?;
        let number: f64 = line.trim().parse().unwrap();
        result += number;
    }

    println!("Problem 0013 - Large Sum: {}", result);

    Ok(())
}
