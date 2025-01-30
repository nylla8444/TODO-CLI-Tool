////////////////////////////////////////////////////////////////////////////////
// This file handles the "read" subcommand for our ToDo application.
//
// Explanation:
// 1) The `handle_read` function retrieves the 'id' argument from ArgMatches, 
//    converting it into a `u32`.
// 2) We then use the `task_manager.read_task(id)` method (from the `models` module) 
//    to look for a matching task.
// 3) On success, it prints out the task details (ID, title, description). 
//    On failure, it displays an error.
// 4) Any error during conversion or lookup causes a message to be printed 
//    and the application to exit.
//
// Communication with Other Files:
// - main.rs: Invokes `handle_read` when the user chooses the "read" subcommand.
// - models::TaskManager: The actual logic for looking up a task is in `read_task`.
// - cli/app.rs: Sets up the "read" subcommand and `id` parameter in the CLI structure.
//    That allows main.rs to forward the command (and any arguments) to this function.
//
// Generally:
// - The flow is: user calls "read <id>" → main.rs → handle_read → TaskManager → prints task info.
////////////////////////////////////////////////////////////////////////////////

use clap::ArgMatches;
use crate::models::TaskManager;

pub fn handle_read(args: &ArgMatches, task_manager: &TaskManager) -> Result<(), Box<dyn std::error::Error>> {
    let id = args.get_one::<String>("id")
        .expect("ID is required")
        .parse::<u32>()
        .unwrap_or_else(|_| {
            println!("Error: ID must be a positive number");
            std::process::exit(1);
        });

    match task_manager.read_task(id) {
        Ok(task) => {
            println!("\n|| ===== Task details ===== ||");
            println!("ID: {}", task.id);
            println!("Title: {}", task.title);
            println!("Description:\n {}\n", task.description);
        },
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}