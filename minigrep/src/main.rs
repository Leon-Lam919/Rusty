//TODO: Create CLI for TODO app

use std::io::Write;
use std::fs::OpenOptions;
use std::io::BufRead;

fn main(){
    let args: Vec<String> = std::env::args().collect();

    let request = &args[1];
    let item = if args.len() > 2 { Some(&args[2]) } else { None };

    // *Will list all todo items
    if request == "list"{
        let file = OpenOptions::new()
            .read(true)
            .open("todo.txt")
            .expect("Failed to open file");
        let reader = std::io::BufReader::new(file);
        
        for (index, line) in reader.lines().enumerate(){
            println!("{}: {}", index + 1, line.unwrap());
        }
        println!("not enough arguments");
        std::process::exit(0);
    }

    if args.len() < 2 {
        println!("not enough arguments");
        std::process::exit(0);
    }

    //* add a new todo item
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
        } else {
            println!("No item to add");
        }
    }
    //TODO: Mark items as complete
    else if request == ""{
    }
    //* edit list
    else if request == "edit"{

    //TODO: Add edit functionality
    } else {
        println!("Invalid command");
    }


}
