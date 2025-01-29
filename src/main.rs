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
        Some(("list", args)) => handle_list(args, &task_manager)?,
        _ => println!("No subcommand was used. Use --help for usage information."),
    }

    Ok(())
}