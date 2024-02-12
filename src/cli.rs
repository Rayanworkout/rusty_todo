// This is the cli module, it will be used to get the cli arguments

// Rust's standard library tool to represent paths in a platform-independent way
use std::path::PathBuf;

// We import structopt crate
// it is used to easily handle cli input
// and convert it to a struct
use structopt::StructOpt;

// We initialize an enum that contains the actions that will be available in our program
// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
#[derive(Debug, StructOpt)]
pub enum Action {
    /// Write tasks to the journal file.
    Add {
        /// The task description text.
        #[structopt()]
        task: String,
    },

    /// Remove an entry from the journal file by position.
    Done {
        #[structopt()]
        position: usize,
    },
    /// List all tasks in the journal file.
    List,
}

// Next we define a struct https://doc.rust-lang.org/book/ch05-00-structs.html
// It holds the Action enum as a wrapper
#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rusty ToDo",
    about = "A command line to-do app written in Rust"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
    pub journal_file: Option<PathBuf>, // If a user wants to point to a journal file that isn't the default one.
}
