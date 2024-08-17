mod read;
fn main(){
    let args: Vec<String> = std::env::args().collect();

    let request = &args[1];
    let item = &args[2];
    let mut list: Vec<String> = Vec::new();

    if args.len() < 3{
        println!("Not enough arguments");
        std::process::exit(1);
    }

    //TODO: Add "list" command to list all todo items
    //TODO: Add "done" command to mark a todo item as done
    //TODO: Add "edit" command to edit a todo item
    if request == "add"{
        list.push(item.to_string());
        println!("Adding a new todo item");
        read::write_out("todo.txt", &item);
    } else if request == "list"{
        read::list_items("todo.txt");
    } else if request == "edit"{
        read::edit_item("todo.txt", &item);
    } else {
        println!("Invalid command");
    }

    //TODO: Add error handling for the case where the user does not provide enough arguments
    //TODO: Create CLI for TODO app
}