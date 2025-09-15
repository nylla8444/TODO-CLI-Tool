use std::fs; // Filesystem operations
use serde_json::{from_str, to_string_pretty}; // JSON serialization
use crate::models::{Task, TaskStats}; // Internal Task and TaskStats structs

/// Manages a list of tasks, provides creation, reading, updating, deleting, and listing functionalities.
pub struct TaskManager {
    /// In-memory collection of tasks loaded from JSON and updated at runtime.
    pub tasks: Vec<Task>,
    /// Path to the JSON file where tasks are stored.
    json_path: String,
}

impl TaskManager {
    /// Loads tasks from the specified JSON file path into the internal `tasks` vector.
    pub fn new(json_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Read the entire file as a string, parse it into a Vec<Task>, and store it in the TaskManager.
        let data = fs::read_to_string(json_path)?;
        let tasks: Vec<Task> = from_str(&data)?;
        Ok(TaskManager { 
            tasks,
            json_path: json_path.to_string(),
        })
    }

    /// Provides basic stats about the current task list (e.g. total number, last used ID).
    pub fn get_stats(&self) -> TaskStats {
        TaskStats {
            total: self.tasks.len(),
            last_id: self.tasks.iter().map(|t| t.id).max().unwrap_or(0),
        }
    }

    /// Creates a new task with a unique ID, adds it to the list, and writes all tasks to JSON.
    pub fn create_task(&mut self, title: String, description: String) -> Result<Task, Box<dyn std::error::Error>> {
        // Generate a new task ID by incrementing the highest existing ID.
        let new_id = self.get_stats().last_id + 1;
        let task = Task { id: new_id, title, description };
        self.tasks.push(task.clone());

        // Serialize the updated tasks vector to JSON, then write it to disk.
        let json = to_string_pretty(&self.tasks)?;
        fs::write(&self.json_path, json)?;

        Ok(task)
    }

    /// Obtains a specific task by ID, returning an error if not found.
    pub fn read_task(&self, id: u32) -> Result<Task, Box<dyn std::error::Error>> {
        // Find the task matching the provided ID in the tasks vector.
        let task = self.tasks
            .iter()
            .find(|t| t.id == id)
            .ok_or("Task not found")?;
        Ok(task.clone())
    }

    /// Updates an existing task and saves changes to disk.
    pub fn update_task(&mut self, id: u32, title: String, description: String) -> Result<Task, Box<dyn std::error::Error>> {
        // Find the index of the task matching 'id' for mutable access.
        let task_index = self.tasks
            .iter()
            .position(|t| t.id == id)
            .ok_or_else(|| format!("Task with id {} not found", id))?;

        // Modify the existing task in place.
        let task = &mut self.tasks[task_index];
        task.title = title;
        task.description = description;

        // Clone the updated task before writing to disk.
        let updated_task = task.clone();
        let json = to_string_pretty(&self.tasks)?;
        fs::write(&self.json_path, json)?;

        Ok(updated_task)
    }

    /// Removes a task from the list by ID, then updates the JSON file.
    pub fn delete_task(&mut self, id: u32) -> Result<(), Box<dyn std::error::Error>> {
        // Check if the task exists first, returning an error if it doesn't.
        self.tasks.iter().find(|t| t.id == id).ok_or("Task not found")?;
        // Retain only tasks that do not match the provided ID.
        self.tasks.retain(|task| task.id != id);

        // Save the updated tasks to disk.
        let json = to_string_pretty(&self.tasks)?;
        fs::write(&self.json_path, json)?;

        Ok(())
    }

    /// Prints a formatted list of all tasks, along with a summary of total tasks.
    pub fn list_tasks(&self) {
        println!("\n=== Tasks List ===");
        println!("{:-<50}", "");
        for task in &self.tasks {
            println!("{} - {}", task.id, task.title);
            println!("{:-<50}", "");
        }
        let stats = self.get_stats();
        println!("\nTotal Tasks: {}", stats.total);
    }
}