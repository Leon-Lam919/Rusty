use std::fs;
use std::env;
mod read;

fn main(){
    let args: Vec<String> = std::env::args().collect();

    let request = &args[1];
    let item = &args[2];
    let mut list: Vec<String> = Vec::new();

    //TODO: Add "add" command to add a new todo item
    //TODO: Add "list" command to list all todo items
    //TODO: Add "done" command to mark a todo item as done
    //TODO: Add "edit" command to edit a todo item
    
    if request == "add"{
        list.push(item.to_string());
        println!("Adding a new todo item");
    } else if request == "list"{
        println!("Listing all todo items");
        println!("{:?}", list);
    } else if request == "done"{
        println!("Marking a todo item as done");
    } else if request == "edit"{
        println!("Editing a todo item");
    } else {
        println!("Invalid command");
    }

    //TODO: Add error handling for the case where the user does not provide enough arguments
    //TODO: Create CLI for TODO app
}