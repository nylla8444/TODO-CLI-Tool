////////////////////////////////////////////////////////////////////////////////
// This file is the entry point for the 'commands' module.
//
// Explanation:
// 1) We declare submodules for each subcommand: create, read, update, delete, list.
//    These submodules hold the logic for the corresponding CLI operations.
//
// 2) We then re-export specific functions (e.g., handle_create, handle_read) so
//    other parts of the application (like main.rs) can invoke them directly via
//    commands::handle_create or commands::handle_read, etc.
//
// 3) This centralizes and organizes all command-oriented functions in one place,
//    ensuring each subcommand has a dedicated file for clarity, while this 
//    mod.rs file provides a single point of access.
//
// Communication with Other Files:
// - main.rs: When the user inputs a subcommand (e.g., "create", "read", etc.),
//   main.rs matches this subcommand and then calls the appropriate function
//   re-exported here (e.g., handle_create).
// - create.rs, read.rs, update.rs, delete.rs, list.rs: Each file implements the
//   logic necessary to process its respective subcommand, such as reading project
//   arguments, updating data, and printing results.
// - models::TaskManager: Each handler calls methods on TaskManager (create_task,
//   read_task, etc.) to manipulate the underlying data (in-memory and JSON storage).
// - cli/app.rs: Defines the structure of subcommands ("create", "read", etc.)
//   and passes the relevant arguments, which eventually get handled by these
//   functions.
//
// Generally:
// - This design helps maintain a clean separation of concerns. The commands
//   module focuses on the "what" to do for each CLI command, while the models
//   module addresses the data structures and logic, and cli sets up the
//   command-line interface. main.rs glues everything together in one
//   orchestrating file.
////////////////////////////////////////////////////////////////////////////////

pub mod create;
pub mod read;
pub mod update;
pub mod delete;
pub mod list;

pub use create::handle_create;
pub use read::handle_read;
pub use update::handle_update;
pub use delete::handle_delete;
pub use list::handle_list;