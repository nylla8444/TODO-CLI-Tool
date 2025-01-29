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