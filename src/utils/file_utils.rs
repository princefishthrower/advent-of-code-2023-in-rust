use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

pub fn read_file_lines(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    // Open the file in read-only mode
    let file = File::open(file_path)?;

    // Create a vector to store lines
    let mut lines: Vec<String> = Vec::new();

    // Create a buffered reader to read lines efficiently
    let reader = io::BufReader::new(file);

    // Iterate over lines and store them in the vector
    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}

