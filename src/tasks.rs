// Crate to get current datetime
use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;

// Crate to work with files
use std::fs::{File, OpenOptions};

// Rust's standard library tool to represent paths in a platform-independent way
use std::fmt;
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};
use std::path::PathBuf;

// derive(Debug) is an attribute that allows me to print the struct
// for debugging purpose
// like this: println!("{:?}", task_struct_instance);

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

// We create an 'implementation', which is a specific method for the task struct.
// Functions defined within impl Task block will only be able to operate
// on Task struct
impl Task {
    pub fn new(text: String) -> Task {
        let created_at: DateTime<Utc> = Utc::now();
        // We create a task instance with the date snapshot and the text parameter
        Task { text, created_at }
    }
}

fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?; // Rewind the file before.
    let tasks = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0))?; // Rewind the file after.
    Ok(tasks)
}

// LINE 55: The question mark symbol (?) after that statement is used to propagate errors without writing too much boilerplate code
// It's syntax sugar for early returning an error if that error matches with the return type of the function it's in.
// So these snippets are equivalent:

// fn function_1() -> Result(Success, Failure) {
//     match operation_that_might_fail() {
//         Ok(success) => success,
//         Err(failure) => return Err(failure),
//     }
// }

// fn function_2() -> Result(Success, Failure) {
//     operation_that_might_fail()?
// }

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    // Open the file.
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    let mut tasks = collect_tasks(&file)?;

    // Write the modified task list back into the file.
    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

//////////////////////////////////////////////////////////////////////////////////

pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
    // Open the file.
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    let mut tasks = collect_tasks(&file)?;

    // Remove the task.
    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }
    tasks.remove(task_position - 1);

    // We're truncating the file before writing to it because we're
    // performing a removal operation. So the file will be smaller than the original.
    // If we ignored this step, the rewound cursor would stop behind the previously written
    // bytes of the file, resulting in a malformed JSON file. When we truncate the file by using
    // the file.set_len(0) operation, we ensure that we're writing the bytes in a blank page.
    file.set_len(0)?;

    // Write the modified task list back into the file.
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}

//////////////////////////////////////////////////////////////////////////////////

pub fn list_tasks(journal_path: PathBuf) -> Result<()> {
    // Open the file.
    let file = OpenOptions::new().read(true).open(journal_path)?;
    // Parse the file and collect the tasks.
    let tasks = collect_tasks(&file)?;

    // Enumerate and display tasks, if any.
    if tasks.is_empty() {
        println!("Task list is empty!");
    } else {
        let mut order: u32 = 1;
        for task in tasks {
            println!("{}: {}", order, task);
            order += 1;
        }
    }

    Ok(())
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]", self.text, created_at)
    }
}
