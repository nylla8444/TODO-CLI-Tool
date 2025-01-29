use std::fs;
use serde_json::{from_str, to_string_pretty};
use crate::models::{Task, TaskStats};

pub struct TaskManager {
    pub tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new(json_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let data = fs::read_to_string(json_path)?;
        let tasks: Vec<Task> = from_str(&data)?;
        Ok(TaskManager { tasks })
    }

    pub fn get_stats(&self) -> TaskStats {
        TaskStats {
            total: self.tasks.len(),
            last_id: self.tasks.iter().map(|t| t.id).max().unwrap_or(0),
        }
    }

    pub fn create_task(&mut self, title: String, description: String) -> Result<Task, Box<dyn std::error::Error>> {
        let new_id = self.get_stats().last_id + 1;
        let task = Task { id: new_id, title, description };
        self.tasks.push(task.clone());
        let json = to_string_pretty(&self.tasks)?;
        fs::write("data/todos.json", json)?;
        Ok(task)
    }

    pub fn read_task(&self, id: u32) -> Result<Task, Box<dyn std::error::Error>> {
        let task = self.tasks
            .iter()
            .find(|t| t.id == id)
            .ok_or("Task not found")?;
        Ok(task.clone())
    }

    pub fn update_task(&mut self, id: u32, title: String, description: String) -> Result<Task, Box<dyn std::error::Error>> {
        let task_index = self.tasks
            .iter()
            .position(|t| t.id == id)
            .ok_or_else(|| format!("Task with id {} not found", id))?;

        let task = &mut self.tasks[task_index];
        task.title = title;
        task.description = description;

        let updated_task = task.clone();
        let json = to_string_pretty(&self.tasks)?;
        fs::write("data/todos.json", json)?;
        Ok(updated_task)
    }

    pub fn delete_task(&mut self, id: u32) -> Result<(), Box<dyn std::error::Error>> {
        self.tasks.iter().find(|t| t.id == id).ok_or("Task not found")?;
        self.tasks.retain(|task| task.id != id);
        let json = to_string_pretty(&self.tasks)?;
        fs::write("data/todos.json", json)?;
        Ok(())
    }

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