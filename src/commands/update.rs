////////////////////////////////////////////////////////////////////////////////
// This file handles the "update" subcommand of the ToDo application.
//
// 1) We parse the 'id' from ArgMatches, ensuring it's a valid positive number.
// 2) We fetch the existing task via 'task_manager.read_task(id)', if it exists.
// 3) We then check 'title' and 'description' from ArgMatches:
//    - If the user didn't supply them, we keep the old values from existing_task.
// 4) We call 'task_manager.update_task(id, title, description)' to apply changes 
//    and save the updated data to JSON.
// 5) Finally, we print the success/error message accordingly.
//
// Communication with Other Files:
// - main.rs: Invokes 'handle_update' when "update" subcommand is chosen.
// - models::TaskManager: The 'read_task' and 'update_task' methods are defined here, 
//   performing lookups, updates, and JSON file writes.
// - cli/app.rs: Defines the "update" subcommand and optional title/description flags.
//   This ensures that if they are not passed, the existing fields remain unchanged.
//
// Example Flow: 
//   cargo run update 1 -t "New Title" -d "New Desc"

//   main.rs → handle_update → read_task → update_task → prints result.
////////////////////////////////////////////////////////////////////////////////

use clap::ArgMatches;
use crate::models::TaskManager;

pub fn handle_update(args: &ArgMatches, task_manager: &mut TaskManager) -> Result<(), Box<dyn std::error::Error>> {
    let id = args.get_one::<String>("id")
        .expect("ID is required")
        .parse::<u32>()
        .unwrap_or_else(|_| {
            println!("Error: ID must be a positive number");
            std::process::exit(1);
        });

    let existing_task = task_manager.read_task(id)?;

    let title = args.get_one::<String>("title")
        .map(|s| s.to_string())
        .unwrap_or(existing_task.title);

    let description = args.get_one::<String>("description")
        .map(|s| s.to_string())
        .unwrap_or(existing_task.description);

    match task_manager.update_task(id, title, description) {
        Ok(task) => println!("Task updated successfully: {:?}", task),
        Err(e) => println!("Error updating task: {}", e),
    }
    Ok(())
}