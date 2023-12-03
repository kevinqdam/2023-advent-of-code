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

    // Map each line into a character vector, keeping only digits
    let character_vectors: Vec<Vec<char>> = lines
        .into_iter()
        .map(|line| line.chars().filter(|ch| ch.is_ascii_digit()).collect())
        .collect();

    // Join the character vectors into strings
    let num_str_vec: Vec<String> = character_vectors
        .into_iter()
        .map(|vec| vec.into_iter().collect())
        .collect();

    // Map each string to delete the middle characters (keep only the first and last character)
    let processed_num_str: Vec<String> = num_str_vec
        .into_iter()
        .map(|num_str| {
            let first_digit: String = num_str.chars().take(1).collect();
            let last_digit: String = num_str.chars().rev().take(1).collect();
            format!("{first_digit}{last_digit}")
        })
        .collect();

    // Parse each string into an integer
    let nums: Vec<usize> = processed_num_str
        .into_iter()
        .map(|num_str| {
            let parse_res = num_str.parse::<usize>();
            match parse_res {
                Ok(num) => num,
                Err(err) => {
                    eprintln!(
                        "Error encountered when parsing num_str {num_str}: {err}",
                        num_str = num_str,
                        err = err
                    );
                    0
                }
            }
        })
        .collect();

    // Sum the integers
    let sum: usize = nums.into_iter().sum();

    // Print it out
    println!("Successfully processed file. Sum is {}", sum);
}
