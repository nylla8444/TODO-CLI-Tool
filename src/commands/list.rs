////////////////////////////////////////////////////////////////////////////////
// This file handles the "list" subcommand for the ToDo application.
//
// 1) The 'handle_list' function is invoked when the user runs the command
//    line tool with the "list" subcommand. 
//    Example: `cargo run list`
//
// 2) It does not require any additional arguments or flags.
//
// 3) The function simply calls 'task_manager.list_tasks()', which prints out
//    all existing tasks and a summary. 
//    
// Communication with Other Files:
// - main.rs: Matches the "list" subcommand and delegates to this function.
// - models/TaskManager: Defines the 'list_tasks()' method that prints 
//   information about tasks in memory.
// - cli/app.rs: Declares the "list" subcommand structure, linking user input
//   to this function in a clean, modular way.
//
// Behind the Scenes:
// - The 'list_tasks()' method in TaskManager accesses its internal Vec<Task>
//   to display each task, as well as total task statistics.
// - This subcommand does not modify any data; it only reads and displays.
//
// Usage Example:
//   cargo run list   // Displays all tasks dynamically from data/todos.json
////////////////////////////////////////////////////////////////////////////////


use crate::models::TaskManager;

pub fn handle_list(task_manager: &TaskManager) -> Result<(), Box<dyn std::error::Error>> {
    task_manager.list_tasks();
    Ok(())
}