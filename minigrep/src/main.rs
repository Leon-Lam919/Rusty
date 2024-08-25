//TODO: Create CLI for TODO app

use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Not enough arguments");
        std::process::exit(1);
    }

    let request = &args[1];
    let item = if args.len() > 2 { Some(&args[2]) } else { None };
    let change = if args.len() > 3 { Some(&args[3]) } else { None };

    // Read the file contents into memory
    let file = OpenOptions::new()
        .read(true)
        .open("todo.txt")
        .expect("Failed to open file");
    let reader = std::io::BufReader::new(file);
    let mut lines: Vec<String> = reader.lines().collect::<Result<_, _>>().expect("Failed to read lines");

    // Handle the "list" request
    if request == "list" {
        for (index, line) in lines.iter().enumerate() {
            println!("{}: {}", index + 1, line);
        }
        std::process::exit(0);
    }

    // Handle the "add" request
    if request == "add" {
        if let Some(item) = item {
            lines.push(item.to_string());
            println!("Adding a new todo item");
        } else {
            println!("No item to add");
            std::process::exit(1);
        }
    }

    // Handle the "complete" request
    else if request == "complete" {
        if lines.is_empty() {
            println!("No items to complete");
            std::process::exit(1);
        } else {
            if let Some(item_name) = item {
                let mut item_found = false;
                for line in lines.iter_mut() {
                    if line == item_name {
                        *line = format!("[x] {}", line);
                        item_found = true;
                        break;
                    }
                }

                if item_found {
                    println!("Item {} marked as complete", item_name);
                } else {
                    println!("Item {} not found", item_name);
                    std::process::exit(1);
                }
            } else {
                println!("No item to complete");
                std::process::exit(1);
            }
        }
    }

    // Handle the "edit" request
    else if request == "edit" {
        if lines.is_empty() {
            println!("Item to edit not found");
            std::process::exit(1);
        } else {
            if let Some(item_name) = item {
                if let Some(new_content) = change {
                    let mut item_found = false;
                    for line in lines.iter_mut() {
                        if line == item_name {
                            *line = new_content.to_string();
                            item_found = true;
                            break;
                        }
                    }

                    if item_found {
                        println!("Item {} edited successfully", item_name);
                    } else {
                        println!("Item {} not found", item_name);
                        std::process::exit(1);
                    }
                } else {
                    println!("No new content provided for the item");
                    std::process::exit(1);
                }
            } else {
                println!("No item name provided");
                std::process::exit(1);
            }
        }
    }

    // Handle the "delete" request
    else if request == "delete" {
        if lines.is_empty() {
            println!("No items to delete");
            std::process::exit(1);
        } else {
            if let Some(item_name) = item {
                let initial_len = lines.len();
                lines.retain(|line| line != item_name);

                if lines.len() < initial_len {
                    println!("Item {} deleted successfully", item_name);
                } else {
                    println!("Item {} not found", item_name);
                    std::process::exit(1);
                }
            } else {
                println!("No item name provided");
                std::process::exit(1);
            }
        }
    }

    // Handle invalid requests
    else {
        println!("Invalid command");
        std::process::exit(1);
    }

    // Write the modified data back to the file
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("todo.txt")
        .expect("Failed to open file");

    for line in lines {
        writeln!(file, "{}", line).expect("Failed to write to file");
    }
}