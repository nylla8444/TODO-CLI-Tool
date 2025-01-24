use clap::{command, Arg, Command};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::{from_str};
use std::fs;

/*
    Todo Business Requirements / User Stories:

 ⁃ I as a User must be able to create a task with the Web App / CLI
 ⁃ I as a User must be able to read a certain task with the Web App / CLI
 ⁃ I as a User must be able to update a task with the Web App / CLI
 ⁃ I as a User must be able to delete a task with the Web App / CLI
 ⁃ I as a User must be able to view all my tasks with the Web App / CLI

A Task has:
 ⁃ A Title
 ⁃ A Description

*/

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u32,
    title: String,
    description: String,
}



// Load tasks from JSON file
// fn load_tasks() -> HashMap<u32, Task> {
//     let data = fs::read_to_string("todos.json").unwrap();
//     let tasks: HashMap<u32, Task> = from_str(&data).unwrap();
//     tasks
// }




fn main() {

     // Use proper path relative to src directory
    let data = match fs::read_to_string("src/todos.json") {
        Ok(content) => content,
        Err(e) => {
            println!("Error reading file: {}", e);
            return;
        }
    };

    let tasks = match from_str::<Vec<Task>>(&data) {
        Ok(t) => t,
        Err(e) => {
            println!("Error parsing JSON: {}", e);
            return;
        }
    };

    println!("{:?}", tasks);


    // Parse JSON into Task struct
    // let task: Task = match from_str(JSON) {
    //     Ok(task) => task,
    //     Err(e) => {
    //         println!("Error parsing JSON: {}", e);
    //         return;
    //     }
    // };

    // Print parsed task
    // println!("Parsed Task:");
    // println!("ID: {}", task.id);
    // println!("Title: {}", task.title);
    // println!("Description: {}", task.description);

    // Create a HashMap to hold tasks with an ID as the key
    let mut tasks: HashMap<u32, Task> = HashMap::new();

    let matches = command!()
        .about("A simple CLI ToDo application that demonstrates CRUD operations using Rust with Clap")
        .version("0.1.0")

         .arg_required_else_help(true)
        .subcommand_required(true)


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


         .subcommand(
            Command::new("read")
                .about("Reads task's details")
        )

         .subcommand(
            Command::new("update")
                .about("Update a task details")
        )

         .subcommand(
            Command::new("delete")
                .about("Delete a task")
        )


        .subcommand(
            Command::new("list")
                .about("List all tasks")
        )

        // It’s used to process the arguments and make them accessible for further handling in your program.
        .get_matches();
    

        
        match matches.subcommand() {
        Some(("create", sub_matches)) => {
            
            // Get passed in values
            let title = sub_matches.get_one::<String>("title").unwrap();
            let description = sub_matches.get_one::<String>("description").unwrap();


            // Print out the values
            println!("Created task: {}", title);
            println!("Description: {}", description);


            // Generate a simple ID (this could be more complex)
            let id = tasks.len() as u32 + 1;

            // Create a new Task and insert it into the tasks HashMap
            let task = Task {
                id,
                title: title.clone(),
                description: description.clone(),
            };

            tasks.insert(task.id, task);
        }

        Some(("delete", sub_matches)) => {
            let id = sub_matches.get_one::<String>("id").unwrap();
            println!("Deleting task with ID: {}", id);
        }


        _ => {
        } // This branch is technically unreachable due to subcommand_required
    }



    // println!("\nAll tasks:");
    // for (id, task) in &tasks {
    //     println!("ID: {}, Title: {}, Description: {}", id, task.title, task.description);
    // }


}
