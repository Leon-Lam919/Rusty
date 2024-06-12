
use std::fs;
use std::io::Write;

pub fn read_in(text: &str){
    println!("Reading from file {}", &text);
    let contents = fs::read_to_string(&text)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

pub fn write_out(text: &str, contents: &str){
    println!("Writing to file {}", &text);
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(&text)
        .expect("Could not open file");

    let contents = format!("{}\n", contents);
    file.write_all(contents.as_bytes())
        .expect("Could not write to file");
}

pub fn list_items(file: &str){
    println!("Listing all todo items");
    read_in(file);
}

pub fn edit_item(file: &str, item: &str){
    println!("Editing todo item {}", &item);
    let contents = fs::read_to_string(&file)
        .expect("Something went wrong reading the file");

    let new_contents = contents.replace(item, "DONE");
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&file)
        .expect("Could not open file");

    file.write_all(new_contents.as_bytes())
        .expect("Could not write to file");
}
