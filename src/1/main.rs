use rayon::prelude::*;

#[path = "../utils/file_utils.rs"] mod file_utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "./src/1/input/input.txt";

    // Call the function from the utils module
    let lines = file_utils::read_file_lines(file_path)?;

    // 1.1
    // Define a sum using Rayon's parallel iterator
    let sum: u32 = lines.par_iter().enumerate().map(|(index, line)| {
        // Get the first and last digit of the line
        if let Some((first_digit, last_digit)) = get_first_and_last_numeric_digit(line) {
            // return the two digits as a two digit number
            first_digit as u32 * 10 + last_digit as u32
        } else {
            // Print an error message
            eprintln!("Line {} does not contain a single-digit number", index + 1);
            // Return 0 if there's an error
            0
        }
    }).sum();

    // Print the sum
    println!("Sum: {}", sum);

    // TODO: 1.2

    Ok(())
}

// gets first and last numerical digit from a string
fn get_first_and_last_numeric_digit(input: &str) -> Option<(u8, u8)> {
    let mut first_digit: Option<u8> = None;
    let mut last_digit: Option<u8> = None;

    for c in input.chars() {
        if let Some(digit) = c.to_digit(10) {
            if digit < 10 {
                // Found a single-digit number
                if first_digit.is_none() {
                    first_digit = Some(digit as u8);
                }
                last_digit = Some(digit as u8);
            }
        }
    }

    match (first_digit, last_digit) {
        (Some(first), Some(last)) => Some((first, last)),
        _ => None,
    }
}

// gets either the first digit or word (one, two, three, etc) from a string
// and also the last digit or word (one, two, three, etc) from a string
fn get_first_and_last_digit_or_word(input: &str) -> Option<(u8, u8)> {
    let mut first_digit: Option<u8> = None;
    let mut last_digit: Option<u8> = None;

    // TODO: regex: if the words "one", "two", etc. up to 'nine' are found before any digit, that's the first digit
    // TODO: regex: if the words "one", "two", etc. up to 'nine' are found after any digit, that's the last digit
    for c in input.chars() {
        if let Some(digit) = c.to_digit(10) {
            if digit < 10 {
                // Found a single-digit number
                if first_digit.is_none() {
                    first_digit = Some(digit as u8);
                }
                last_digit = Some(digit as u8);
            }
        }
    }

    match (first_digit, last_digit) {
        (Some(first), Some(last)) => Some((first, last)),
        _ => None,
    }
}