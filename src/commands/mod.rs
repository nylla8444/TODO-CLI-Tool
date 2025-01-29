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