use std::io::Write;
use std::fs::OpenOptions;

fn main(){
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2{
        println!("not enough arguments");
        std::process::exit(1);
    }

    let request = &args[1];
    let item = if args.len() > 2 { Some(&args[2]) } else { None };


    //TODO: Add "list" command to list all todo items
    //TODO: Add "done" command to mark a todo item as done
    //TODO: Add "edit" command to edit a todo item
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
    } else if request == "edit"{
        //TODO: Add edit functionality
    } else {
        println!("Invalid command");
    }

    //TODO: Create CLI for TODO app

}
