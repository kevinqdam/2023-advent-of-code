use std::io::Read;
use std::{fs::File, io::BufReader};

fn main() {
    println!("Hello, world!");

    // Read the file
    let file_result = File::open("./input.txt");
    match file_result {
        Ok(file) => {
            process_file(file);
        }
        Err(err) => {
            eprintln!("Error opening file: {}", err);
        }
    }
}

fn process_file(file: File) {
    // Read the text from the file into a string buffer
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    let read_result = reader.read_to_string(&mut buffer);
    if let Err(err) = read_result {
        eprintln!(
            "Error occurred when reading the file into a string buffer: {err}",
            err = err
        );
        return;
    }
    println!("Successfully read the file into a string buffer");

    // Split into lines
    let lines: Vec<&str> = buffer.lines().collect();

    // Map each line into tuple of first digit and last digit
    let digit_tuples: Vec<(usize, usize)> = lines
        .iter()
        .map(|line| {
            let first_digit = get_first_digit(line);
            let last_digit = get_last_digit(line);
            (first_digit, last_digit)
        })
        .collect();

    // Sum the integers
    let sum: usize = digit_tuples
        .iter()
        .map(|(digit1, digit2)| (digit1 * 10) + digit2)
        .sum();

    // Print it out
    println!("Successfully processed file. Sum is {}", sum);
}

fn get_first_digit(line: &str) -> usize {
    let needles: Vec<(&str, usize)> = vec![
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
    let mut min_index = usize::MAX;
    let mut first_digit = 0;
    for (needle, digit) in needles {
        let index = line.find(needle).unwrap_or(usize::MAX);
        if index < min_index {
            min_index = index;
            first_digit = digit;
        }
    }

    first_digit
}

fn get_last_digit(line: &str) -> usize {
    let needles: Vec<(&str, usize)> = vec![
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
    let mut max_index = line.len();
    let mut last_digit = 0;
    for (needle, digit) in needles {
        let index = line.rfind(needle).unwrap_or(line.len());
        if max_index == line.len() || (index > max_index && index != line.len()) {
            max_index = index;
            last_digit = digit;
        }
    }

    last_digit
}
