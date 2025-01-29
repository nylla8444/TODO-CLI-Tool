use clap::ArgMatches;
use crate::models::TaskManager;

pub fn handle_create(args: &ArgMatches, task_manager: &mut TaskManager) -> Result<(), Box<dyn std::error::Error>> {
    let title = args.get_one::<String>("title").expect("Required").to_string();
    let description = args.get_one::<String>("description").expect("Required").to_string();

    match task_manager.create_task(title, description) {
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