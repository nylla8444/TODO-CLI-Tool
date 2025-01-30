////////////////////////////////////////////////////////////////////////////////
// This file handles the "delete" subcommand for our ToDo application.
//
// 1) The 'handle_delete' function uses 'clap' (ArgMatches) to read the user's 
//    input, extracting the 'id' parameter which identifies the task to be deleted.
//
// 2) The function then attempts to parse the provided 'id' as a positive integer. 
//    If the parse fails, an error message is printed, and the program exits.
//
// 3) We then call 'delete_task' on the given 'task_manager' (from models::TaskManager).
//    If successful, this removes the task from the in-memory list and updates
//    the JSON file. Otherwise, it returns an error.
//
// 4) Finally, the function prints a success or error message, then returns a 
//    Result indicating success or failure.
//
// Communication with Other Files:
// - main.rs: Invokes 'handle_delete' when the user enters the "delete" subcommand.
// - models/TaskManager: The 'delete_task' method is defined here, managing the list
//   of tasks and performing the file I/O to save changes.
// - cli/app.rs: The 'delete' subcommand is defined, specifying required arguments.
//
////////////////////////////////////////////////////////////////////////////////

use clap::ArgMatches;
use crate::models::TaskManager;

pub fn handle_delete(args: &ArgMatches, task_manager: &mut TaskManager) -> Result<(), Box<dyn std::error::Error>> {

    let id = args.get_one::<String>("id")
        .expect("ID is required")
        .parse::<u32>()
        .unwrap_or_else(|_| {
            println!("Error: ID must be a positive number");
            std::process::exit(1);
        });
        
    println!("Deleting task {}...", id);

    match task_manager.delete_task(id) {
        Ok(_) => println!("Task {} deleted successfully", id),
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}