// #![allow(unused_imports)]

use clap::{command, Arg, Command};
use serde::{Serialize, Deserialize};
use serde_json::{from_str, to_string_pretty};
use std::fs;


/*
    Todo Business Requirements / User Stories:

 DONE ⁃ I as a User must be able to CREATE a task with the Web App / CLI
 DONE ⁃ I as a User must be able to READ a certain task with the Web App / CLI
 ⁃ I as a User must be able to UPDATE a task with the Web App / CLI
 DONE ⁃ I as a User must be able to DELETE a task with the Web App / CLI
 DONE ⁃ I as a User must be able to VIEW ALL (list) my tasks with the Web App / CLI

A Task has:
 ⁃ A Title
 ⁃ A Description

*/

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Task {
    id: u32,
    title: String,
    description: String,
}

struct TaskManager {
    tasks: Vec<Task>,
}


struct TaskStats {
    total: usize,
    last_id: u32,
}

impl TaskManager {
    fn new(json_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let data = fs::read_to_string(json_path)?;
        let tasks: Vec<Task> = from_str(&data)?;
        Ok(TaskManager { tasks })
    }

    // --------- GET STATS
    fn get_stats(&self) -> TaskStats {
        TaskStats {
            total: self.tasks.len(),
            last_id: self.tasks.iter().map(|t| t.id).max().unwrap_or(0),
        }
    }


    // --------- CREATE NEW TASK 
    fn create_task(&mut self, title: String, description: String) -> Result<Task, Box<dyn std::error::Error>> {
        let new_id = self.get_stats().last_id + 1;
        
        let task = Task {
            id: new_id,
            title,
            description,
        };
    
        self.tasks.push(task.clone()); // need derive Clone for Task
    
        // Save to JSON file
        let json = serde_json::to_string_pretty(&self.tasks)?;
        fs::write("src/todos.json", json)?;
        
        Ok(task)
    }


        // --------- READ TASK
    fn read_task(&self, id: u32) -> Result<Task, Box<dyn std::error::Error>>{
        let task = self.tasks.iter()
                .find(|t| t.id == id)
                .ok_or("Task not found")?;

        Ok(task.clone())
    }

    // --------- DELETE TASK
    fn delete_task(&mut self, id: u32) -> Result<(), Box<dyn std::error::Error>> {
        // Remove task with matching id
        self.tasks.iter()
                .find(|t| t.id == id)
                .ok_or("Task not found")?;


        self.tasks.retain(|task| task.id != id);
        // only retain values that is not equal to the passed id value

        // Save updated tasks to JSON
        let json = to_string_pretty(&self.tasks)?;
        fs::write("src/todos.json", json)?;
        
        Ok(())
    }


    // --------- LIST ALL TASKS
    fn list_tasks(&self) {
        println!("\n=== Tasks List ===");
        println!("{:-<50}", "");
        for task in &self.tasks {
            println!("{} - {}",task.id, task.title);

            // todo: Description will be reserved for read subcommand
            // println!("Description: {}", task.description);
            println!("{:-<50}", "");
        }
        
        let stats = self.get_stats();
        println!("\nTotal Tasks: {}", stats.total);
        // println!("Last ID: {}", stats.last_id);
    }




}




fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut task_manager = TaskManager::new("src/todos.json")?;


    let matches = command!()
        .about("A simple CLI ToDo application that demonstrates CRUD operations using Rust with Clap")
        .version("0.1.0")

        // .arg_required_else_help(true)
        // .subcommand_required(true)
        // returns: error: process didn't exit successfully: `target\debug\todo-rust-cli.exe` (exit code: 2)


        // Create
        .subcommand(
            Command::new("create")
                .about("Creates a new task.\nMore info: create --help")
                .arg(
                    Arg::new("title")
                    .short('t')
                    .long("title")
                    .required(true)
                    .help("Task's Title")
                )
                .arg(
                    Arg::new("description")
                    .short('d')
                    .long("description")
                    .required(true)
                    .help("Task's Description")
                )
        )


        // Read
        .subcommand(
            Command::new("read")
                .about("Reads task's details")
                .arg(
                    // Positional argument - no short/long flags needed
                    Arg::new("id")
                        .help("The ID of the task to read")
                        .required(true)
                        .index(1)  // First position
                )
        )


        // Update
        .subcommand(
            Command::new("update")
                .about("Update a task details")
        )


        // Delete
        .subcommand(
            Command::new("delete")
                .about("Delete a task")
                .arg(
                    // Positional argument - no short/long flags needed
                    Arg::new("id")
                        .help("The ID of the task to read")
                        .required(true)
                        .index(1)  // First position
                )
                
        )

        // List
        .subcommand(
            Command::new("list")
                .about("List all tasks")
        )

        // It’s used to process the arguments and make them accessible for further handling in your program.
        .get_matches();
    

    // Commands Functions
        match matches.subcommand() {

        Some(("create", args)) => {
            let title = args.get_one::<String>("title")
                .expect("Required")
                .to_string();
            let description = args.get_one::<String>("description")
                .expect("Required")
                .to_string();

            match task_manager.create_task(title, description) {
                Ok(task) => {
                    println!("\nTask created successfully!");
                    println!("ID: {}", task.id);
                    println!("Title: {}", task.title);
                    println!("Description: {}", task.description);
                }
                Err(e) => println!("Failed to create task: {}", e),
            }
        }


        Some(("read", arg_id)) => {
            let id = arg_id.get_one::<String>("id")
                .unwrap()
                .parse::<u32>() // turn string into u32 (integer)
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
        }


        Some(("delete", arg_id)) => {
            let id = arg_id.get_one::<String>("id")
                .unwrap()
                .parse::<u32>() // turn string into u32 (integer)
                .unwrap_or_else(|_| {
                    println!("Error: ID must be a positive number");
                    std::process::exit(1);
                });
            println!("Deleting task {}...", id);

            match task_manager.delete_task(id){
                Ok(_) => {
                    println!("Task {} deleted successfully",id );
                },
                Err(e) => println!("Error: {}", e),
            }
        
        }


        Some(("list", _)) => {
            task_manager.list_tasks();
        }


      
        _ => {
            println!("No subcommand was used. Use help for usage information.");
        } // This branch will be technically unreachable if subcommand_required is enabled
    }



    // Return status
    Ok(())
}



