//TODO: Create CLI for TODO app

use std::io::Write;
use std::fs::{read, OpenOptions};
use std::io::BufRead;

fn main(){
    let args: Vec<String> = std::env::args().collect();

    let request = &args[1];
    let item = if args.len() > 2 { Some(&args[2]) } else { None };
    let change = if args.len() > 3 { Some(&args[3]) } else { None };

    // **Will list all todo items ** 
    if request == "list"{
        let file = OpenOptions::new()
            .read(true)
            .open("todo.txt")
            .expect("Failed to open file");
        let reader = std::io::BufReader::new(file);
        
        for (index, line) in reader.lines().enumerate(){
            println!("{}: {}", index + 1, line.unwrap());
        }
        std::process::exit(0);
    }

    //* add a new todo item ** 
    if request == "add"{
        if let Some(item) = item {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open("todo.txt")
                .expect("Failed to open file");
            writeln!(file, "{}", item).expect("failed to write to file");
            println!("Adding a new todo item");
            std::process::exit(0);
        } else {
            println!("No item to add");
            std::process::exit(1);
        }
    }

    // ** Mark items as complete ** 
    else if request == "complete"{
        let file = OpenOptions::new()
            .read(true)
            .open("todo.txt")
            .expect("Failed to open file");
        let reader = std::io::BufReader::new(file);
        let lines: Vec<_> = reader.lines().collect::<Result<_, _>>().expect("Failed to read lines");
        
        if lines.is_empty() {
            println!("No items to complete");
            std::process::exit(1);
        } else {
            if let Some(item_name) = item{
                let mut file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open("todo.txt")
                    .expect("Failed to open file");
                
                let mut item_found = false;
                for line in lines{
                    if line == *item_name{
                        writeln!(file, "[x] {}", line).expect("Failed to write to file");
                        item_found = true;
                    } else {
                        writeln!(file, "{}", line).expect("Failed to write to file");
                    }
                }
            
                if item_found{
                    println!("Item {} marked as complete", item_name);
                    std::process::exit(0);
                } else{
                    println!("Item {} not found", item_name);
                    std::process::exit(1);
                }
            } else{
                println!("No item to complete");
                std::process::exit(1);
            }
        }
    }

    //* edit list
    else if request == "edit"{
        //TODO: Add edit functionality
        let file = OpenOptions::new()
            .read(true)
            .open("todo.txt")
            .expect("Failed to open file");
        let reader = std::io::BufReader::new(file);
    
        let lines: Vec<_> = reader.lines().collect::<Result<_, _>>().expect("Failed to read lines");
        
        if lines.is_empty(){
            println!("Item to edit not found");
            std::process::exit(1);
        }else{
            if let Some(item_name) = item{
                let mut file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open("todo.txt")
                    .expect("Failed to open file");

                    for line in lines{
                        if line == *item_name{
                            println!("New item: {}", item_name);
                        }
                    }
                    }
                }
            }
        
    
    else if request == "delete"{
        //TODO: Add delete functionality
    }
    else{
        println!("Invalid command");
        std::process::exit(1);
    }
}
