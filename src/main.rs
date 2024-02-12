use anyhow::anyhow;
use std::path::PathBuf;
use structopt::StructOpt;
mod cli;
mod tasks;

use cli::{Action::*, CommandLineArgs};
use tasks::Task;

fn find_default_journal_file() -> Option<PathBuf> {
    // Get the home directory
    let default_path = home::home_dir()?.join(".rusty-todo.json");

    // Check if the file exists
    if !default_path.exists() {
        // If the file doesn't exist, create it
        match std::fs::File::create(&default_path) {
            Ok(_) => println!("Created default journal file: {:?}", default_path),
            Err(err) => {
                eprintln!("Error creating default journal file: {}", err);
                return None;
            }
        }
    }

    Some(default_path)
}

fn main() -> anyhow::Result<()> {
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("Failed to find journal file."))?;

    match action {
        Add { task } => tasks::add_task(journal_file, Task::new(task)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }?;
    Ok(())
}
