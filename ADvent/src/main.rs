use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let mut str = String::new();

    let file = File::open("input.txt").expect("Error opening file");

    let mut buffer_reader = BufReader::new(file);

    buffer_reader.read_to_string(&mut str);

    let char_vec: Vec<char> = str.chars().collect();

    println!("{:?}", char_vec);
}
