////////////////////////////////////////////////////////////////////////////////
// This file handles the "create" subcommand for our ToDo application.
//
// 1) The 'handle_create' function uses 'clap' (ArgMatches) to read the user's
//    input: the 'title' and 'description' of the new task.
//
// 2) It then calls 'create_task' on the provided 'task_manager' (from models/TaskManager),
//    which updates the in-memory task list and saves the new data to JSON.
//
// 3) Finally, it prints information about the newly created task or logs any error.
//
// Communication with Other Files:
// - main.rs: Invokes 'handle_create' when the user enters the "create" subcommand.
// - models/TaskManager: The 'create_task' method is defined here, handling the actual
//   creation logic and file I/O.
// - cli/app.rs: Defines the subcommand structure and required arguments (title, description).
////////////////////////////////////////////////////////////////////////////////

use clap::ArgMatches;
use crate::models::TaskManager;

pub fn handle_create(args: &ArgMatches, task_manager: &mut TaskManager) -> Result<(), Box<dyn std::error::Error>> {
    
    let title = args.get_one::<String>("title").expect("Required").to_string();
    let description = args.get_one::<String>("description").expect("Required").to_string();


    match task_manager.create_task(title, description) {
        // if create_task returns Ok(task), print the task details
        Ok(task) => {
            println!("\nTask created successfully!");
            println!("ID: {}", task.id);
            println!("Title: {}", task.title);
            println!("Description: {}", task.description);
        }
        Err(e) => println!("Failed to create task: {}", e),
    }
    Ok(())
}