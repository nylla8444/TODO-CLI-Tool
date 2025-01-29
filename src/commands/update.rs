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