pub mod task;
pub mod task_manager;

// Re-export for convenient use
pub use task::Task;
pub use task_manager::TaskManager;
pub use task::TaskStats;

// So that we can use the TaskManager in the main.rs file as:
// >  use models::TaskManager;
// >  let task_manager = TaskManager::new();
