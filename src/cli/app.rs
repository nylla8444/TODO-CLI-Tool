use clap::{command, Arg, Command};

pub fn build_cli() -> Command {
    command!()
        .about("A simple CLI ToDo application that demonstrates CRUD using Rust + Clap")
        .version("0.1.0")
        // .subcommand_required(true) // optional
        .subcommand(
            Command::new("create")
                .about("Creates a new task.\nMore info: create --help")
                .arg(
                    Arg::new("title")
                        .short('t')
                        .long("title")
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