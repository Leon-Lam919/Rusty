fn main(){
    let args: Vec<String> = std::env::args().collect();

    let request = &args[1];
    let item = &args[2];
    let mut list: Vec<String> = Vec::new();

    if args[2] == ""{
        if args[1] == "list"{
            return;
        }
        else{
            println!("not enough arguments");
            std::process::exit(1);
        }
    } 


    //TODO: Add "list" command to list all todo items
    //TODO: Add "done" command to mark a todo item as done
    //TODO: Add "edit" command to edit a todo item
    if request == "add"{
        list.push(item.to_string());
        println!("Adding a new todo item");
        //TODO: Add item to list
    } else if request == "list"{
    // TODO: Add loop to print out list items
    } else if request == "edit"{
        //TODO: Add edit functionality
    } else {
        println!("Invalid command");
    }

    //TODO: Add error handling for the case where the user does not provide enough arguments
    //TODO: Create CLI for TODO app
}