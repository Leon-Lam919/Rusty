use std::env;
use std::fs;

pub fn read_in(text: &str){
    println!("Reading from file {}", &text);
    let contents = fs::read_to_string(&text)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
