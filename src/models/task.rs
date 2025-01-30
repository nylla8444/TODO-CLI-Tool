use serde::{Serialize, Deserialize};

/// Represents a single task record in the application.
/// - `id`: Uniquely identifies each task
/// - `title`: Brief name or label of the task
/// - `description`: Detailed explanation of what needs to be done
///
/// It supports JSON serialization/deserialization via Serde, 
/// making it easy to read/write tasks in the TaskManager.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
}

/// Stores basic statistics about the current set of tasks.
/// - `total`: The total number of tasks currently stored
/// - `last_id`: The highest task ID in use, allowing new tasks to be 
///              assigned an incremented ID value
///
/// The TaskManager uses `TaskStats` to summarize the tasks loaded from JSON.
#[derive(Debug)]
pub struct TaskStats {
    pub total: usize,
    pub last_id: u32,
}