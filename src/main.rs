//  ToDo Business Requirements / User Stories:

// DONE ⁃ I as a User must be able to create a task with the Web App / CLI
// DONE ⁃ I as a User must be able to read a certain task with the Web App / CLI
// DONE ⁃ I as a User must be able to update a task with the Web App / CLI
// DONE ⁃ I as a User must be able to delete a task with the Web App / CLI
// DONE ⁃ I as a User must be able to view all my tasks with the Web App / CLI

// A Task has:
// added ID field
//  ⁃ A Title
//  ⁃ A Description


///////////////////////////////////////////////////////////////////////////////
// MAIN ENTRY POINT EXPLANATION:
//
// 1) The `models` module contains the data definitions and the `TaskManager` 
//    struct, which manages the internal list of tasks and handles file I/O.
//
// 2) The `commands` module contains the subcommand handlers (create, read, 
//    update, delete, list). Each handler calls the appropriate methods on 
//    the `TaskManager` to perform the requested action.
//
// 3) The `cli` module sets up the command-line interface structure using 
//    Clap (see `cli/app.rs`). It defines available subcommands, arguments, 
//    and options.
//
// 4) In `main`, we create a `TaskManager` instance pointing to `data/todos.json`,
//    then build the CLI commands via `cli::app::build_cli()`. We parse and match 
//    the user’s input. Then we dispatch each subcommand to its matching function 
//    in the `commands` module.
//
// 5) This design cleanly separates concerns: 
//    - `models` for data and logic
//    - `commands` for specific actions 
//    - `cli` for argument parsing 
//    - `main` for coordinating everything.
///////////////////////////////////////////////////////////////////////////////

mod models;
mod commands;
mod cli;

use models::TaskManager;
use commands::{handle_create, handle_read, handle_update, handle_delete, handle_list};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut task_manager = TaskManager::new("data/todos.json")?;
    
    let cli = cli::app::build_cli();
    let matches = cli.get_matches();

    match matches.subcommand() {
        Some(("create", args)) => handle_create(args, &mut task_manager)?,
        Some(("read", args)) => handle_read(args, &task_manager)?,
        Some(("update", args)) => handle_update(args, &mut task_manager)?,
        Some(("delete", args)) => handle_delete(args, &mut task_manager)?,
        Some(("list", _)) => handle_list( &task_manager)?,
        _ => println!("No subcommand was used. Use --help for usage information."),
    }

    Ok(())
}