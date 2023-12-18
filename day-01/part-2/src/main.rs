use std::io::Error;
use std::io::Read;
use std::{fs::File, io::BufReader};

const VALID_DIGITS: [(&str, usize); 18] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

fn main() {
    println!("Hello, world!");

    // Read the file
    let file_result = File::open("./input.txt");
    match file_result {
        Ok(file) => {
            let result = process_file(file);
            match result {
                Ok(sum) => {
                    println!("Successfully processed file. Sum is {}", sum);
                }
                Err(err) => {
                    eprintln!("Failed to process file: {}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("Error opening file: {}", err);
        }
    }
}

fn process_file(file: File) -> Result<usize, Error> {
    // Read the text from the file into a string buffer
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    let read_result = reader.read_to_string(&mut buffer);
    if let Err(err) = read_result {
        eprintln!(
            "Error occurred when reading the file into a string buffer: {err}",
            err = err
        );
        return Err(err);
    }
    println!("Successfully read the file into a string buffer");

    // Split into lines
    let lines: Vec<&str> = buffer.lines().collect();

    // Map each line into tuple of first digit and last digit
    let digit_tuples: Vec<(usize, usize)> = lines.iter().map(line_to_digit_tuple).collect();

    // Sum the integers
    let sum: usize = digit_tuples
        .iter()
        .map(|(digit1, digit2)| (digit1 * 10) + digit2)
        .sum();

    Ok(sum)
}

fn line_to_digit_tuple<T: AsRef<str>>(line: T) -> (usize, usize) {
    let first_digit = get_first_digit(line.as_ref());
    let last_digit = get_last_digit(line.as_ref());
    (first_digit, last_digit)
}

fn get_first_digit(line: &str) -> usize {
    let mut min_index = usize::MAX;
    let mut first_digit = 0;
    for (str, digit) in VALID_DIGITS {
        let index = line.find(str).unwrap_or(usize::MAX);
        if index < min_index {
            min_index = index;
            first_digit = digit;
        }
    }

    first_digit
}

fn get_last_digit(line: &str) -> usize {
    let mut max_index = line.len();
    let mut last_digit = 0;
    for (str, digit) in VALID_DIGITS {
        let index = line.rfind(str).unwrap_or(line.len());
        if max_index == line.len() || (index > max_index && index != line.len()) {
            max_index = index;
            last_digit = digit;
        }
    }

    last_digit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_to_digit_tuple() {
        assert_eq!(line_to_digit_tuple("4threelfvzndfive"), (4, 5))
    }
}
