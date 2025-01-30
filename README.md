# Rust ToDo CLI

A simple command-line application for creating, reading, updating, deleting, and listing tasks.  
Built with Rust, using the following modules:
- **models**: Defines the Task data structure and manages file I/O via the TaskManager.
- **commands**: Contains the core subcommand handlers (create, read, update, delete, list).
- **cli**: Configures the command-line interface using Clap, mapping subcommands to handler functions.
- **main.rs**: Initializes the TaskManager and dispatches subcommands.

## How It Works
1. The application loads tasks from a JSON file (data/todos.json) into an in-memory Vec<Task>.
2. Users run a subcommand (e.g., create, read, update, delete, list) from the CLI.
3. Each subcommand calls a handler in the commands/ module, which in turn uses TaskManager to manipulate data.
4. The updated tasks are written back to JSON, preserving changes between runs.

## Prerequisites
1. Install Rust (version 1.68+ recommended).
2. Install Git (latest version).

## Setup

2. Clone or download this repository.
```bash
git clone https://github.com/KeebyResearch/TODO-CLI-Tool.git
```
3. Navigate to the project folder /src and run:
```bash
cargo build
```

## Usage
Run the CLI with:
Available subcommands:
- **create**: Add a new task.  
  Example: `cargo run create --title "Buy Groceries" --description "Milk, Eggs, Bread"`
- **read**: Display a specific task by ID.  
  Example: `cargo run read 1`
- **update**: Update an existing task.  
  Example: `cargo run update 1 --title "New Title"`
- **delete**: Remove a task by ID.  
  Example: `cargo run delete 1`
- **list**: Show all tasks.  
  Example: `cargo run list`
- **help**: Show all subcommands information.  
  Example: `cargo run help`

## Project Structure
- **src/models**  
  - `task.rs`: Task data and serialization.  
  - `task_manager.rs`: Methods to create, read, update, delete, and list tasks.
- **src/commands**  
  - One file per subcommand (create, read, update, delete, list), each with a handler function.
- **src/cli**  
  - `app.rs`: Builds the Clap-based CLI.
  - `mod.rs`: Re-exports app for easy access.
- **main.rs**  
  - Initializes TaskManager from data/todos.json.
  - Parses CLI input.
  - Dispatches subcommands to the corresponding handlers.

Feel free to extend this CLI or use it as a base for more advanced Rust projects!