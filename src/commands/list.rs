use clap::ArgMatches;
use crate::models::TaskManager;

pub fn handle_list(_args: &ArgMatches, task_manager: &TaskManager) -> Result<(), Box<dyn std::error::Error>> {
    task_manager.list_tasks();
    Ok(())
}