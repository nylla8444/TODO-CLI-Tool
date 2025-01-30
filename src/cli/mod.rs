///////////////////////////////////////////////////////////////////////////////
// This 'mod.rs' file serves as the entry point for the 'cli' module.
// It re-exports the 'app' submodule, which contains the function 'build_cli'.
//
// 1) The 'cli' directory contains everything related to command-line interface
//    setup, such as the structure of subcommands, arguments, and options.
//
// 2) 'mod.rs' makes the 'app' module accessible to the rest of the program by
//    declaring `pub mod app;`, meaning code in the root of 'cli' can be accessed
//    as 'cli::app::...'.
///////////////////////////////////////////////////////////////////////////////

pub mod app;