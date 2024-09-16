use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let mut str = String::new();

    let file = File::open("test.txt").expect("Error opening file");

    let mut buffer_reader = BufReader::new(file);

    buffer_reader.read_to_string(&mut str);

    let mut balanace = 0;

    for c in str.chars() {
        if c == '(' {
            balanace += 1;
        } else {
            balanace += 1;
        }
    }
    println!("{}", balanace);
}
