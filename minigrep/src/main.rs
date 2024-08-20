fn main(){
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2{
        println!("not enough arguments");
        std::process::exit(1);
    }

    let request = &args[1];
    let item = if args.len() > 2 { Some(&args[2]) } else { None };
    let mut list: Vec<String> = Vec::new();


    //TODO: Add "list" command to list all todo items
    //TODO: Add "done" command to mark a todo item as done
    //TODO: Add "edit" command to edit a todo item
    if request == "add"{
        if let Some(item) = item {
            list.push(item.to_string());
            println!("Adding a new todo item");
        } else {
            println!("No item to add");
        }
    } else if request == "list"{
    // TODO: Add loop to print out list items
        for (index, item) in list.iter().enumerate(){
            println!("{}: {}", index, item);
        }
    } else if request == "edit"{
        //TODO: Add edit functionality
    } else {
        println!("Invalid command");
    }

    //TODO: Create CLI for TODO app

}