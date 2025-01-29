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