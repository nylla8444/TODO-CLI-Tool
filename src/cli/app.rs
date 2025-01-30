///////////////////////////////////////////////////////////////////////////////
// This file is responsible for building and defining the command line interface
// (CLI) of the ToDo application using the 'clap' crate.
//
// 1) The 'build_cli' function returns a Command object that includes all
//    subcommands (create, read, update, delete, list) and their respective
//    arguments/flags.
// 2) Each subcommand corresponds to a user action; the code in main.rs reads the
//    user's selected subcommand from the CLI and dispatches to the proper handler
//    in the 'commands' module.
// 3) This design allows main.rs to remain focused on matching the user's chosen
//    command and delegating the logic, while this file centralizes the structure
//    of the CLI.
//
// Interaction With Other Files:
// - 'main.rs' calls 'build_cli()' from this file to create the CLI structure.
// - 'commands' module: Each subcommand defined here (e.g. "create", "read",
//   "update", etc.) matches to the corresponding functions in commands/.
// - 'models' module: The commands, once invoked, use the TaskManager and data
//   types from 'models' to actually carry out the requested task.
//
// Below is the function that constructs the command line interface,
// detailing each subcommand and the arguments it accepts.
///////////////////////////////////////////////////////////////////////////////

use clap::{command, Arg, Command};

pub fn build_cli() -> Command {
    command!()
        .about("A simple CLI ToDo application that demonstrates CRUD using Rust + Clap")
        .version("0.1.0")
        // .subcommand_required(true) // optional; forces the user to select a subcommand
        .subcommand(
            Command::new("create")
                .about("Creates a new task.\nMore info: create --help")
                .arg(
                    Arg::new("title")
                        .short('t') 
                        .long("title")//
                        .required(true)
                        .help("Task's Title")
                )
                .arg(
                    Arg::new("description")
                        .short('d')
                        .long("description")
                        .required(true)
                        .help("Task's Description")
                )
        )
        .subcommand(
            Command::new("read")
                .about("Reads task's details")
                .arg(
                    Arg::new("id")
                        .help("The ID of the task to read")
                        .required(true)
                        .index(1)
                )
        )
        .subcommand(
            Command::new("update")
                .about("Update a task details\nMore info: update --help")
                .arg(
                    Arg::new("id")
                        .help("The ID of the task to read")
                        .required(true)
                        .index(1)
                )
                .arg(
                    Arg::new("title")
                        .short('t')
                        .long("title")
                        .required(false)
                        .help("Task's Title to be Updated")
                )
                .arg(
                    Arg::new("description")
                        .short('d')
                        .long("description")
                        .required(false)
                        .help("Task's Description to be Updated")
                )
        )
        .subcommand(
            Command::new("delete")
                .about("Delete a task")
                .arg(
                    Arg::new("id")
                        .help("The ID of the task to delete")
                        .required(true)
                        .index(1)
                )
        )
        .subcommand(
            Command::new("list")
                .about("List all tasks")
        )
}