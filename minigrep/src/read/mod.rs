use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, Write};

pub fn write_out(file_path: &str, item: &str) {
    let mut file = OpenOptions::new().append(true).open(file_path).expect("Cannot open file");
    writeln!(file, "{}", item).expect("Cannot write to file");
}

pub fn list_items(file_path: &str) {
    let file = fs::File::open(file_path).expect("Cannot open file");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.expect("Cannot read line"));
    }
}

pub fn edit_item(file_path: &str, item: &str) {
    // Implement the logic to edit an item in the file
    println!("Editing item: {}", item);
}

pub fn mark_done(file_path: &str, item: &str) {
    // Implement the logic to mark an item as done
    println!("Marking item as done: {}", item);
}