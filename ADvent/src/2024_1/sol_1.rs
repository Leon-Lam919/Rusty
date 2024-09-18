use std::fs;
use std::io::{self, BufRead};

fn main() {
    // Attempt to open the file and handle any potential errors
    let input = fs::File::open("./file_1.txt").expect("Failed to open file");
    let reader = io::BufReader::new(input);

    // Initialize a variable to store the final number
    let mut final_num: u32 = 0;

    // Iterate over each line in the file
    for (index, line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read line");
        let mut concat_string = String::new();

        // Iterate over each character in the line
        for c in line.chars() {
            if c.is_digit(10) {
                concat_string.push(c);
                if c.len() == 1{
                    concat_string.push(c);
                }
            }
        }

        // Parse the concatenated string of digits into a number
        let num: u32 = concat_string.parse().unwrap();
        final_num = num; // Update the final number
    }

    // Print the final number
    println!("{}", final_num);
}