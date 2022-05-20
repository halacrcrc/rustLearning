use chrono::{serde::ts_seconds, DateTime, Utc, Local};
use serde::{Deserialize,Serialize};
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::io::{Error, ErrorKind,Result, Seek, SeekFrom,};
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub create_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let create_at: DateTime<Utc> = Utc::now();
        Task { text, create_at}
    }
}

//add task
pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    //open file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;
    
    //consume the file's content as a vector of tasks
    let mut tasks = collect_tasks(&file)?;
    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}

//complete task
pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
    //open file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    //consume the file's contents as a vector of tasks
    let mut tasks = collect_tasks(&file)?;

    //remove the task
    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }
    tasks.remove(task_position - 1);

    //write the modified task list back into the file
    file.set_len(0)?;
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}

//print task
pub fn list_tasks(journal_path: PathBuf) -> Result<()> {
    //open the file
    let file = OpenOptions::new().read(true).open(journal_path)?;
    //parse the file and collect the tasks
    let tasks =collect_tasks(&file)?;

    //enumerate and display tasks, if any
    if tasks.is_empty() {
        println!("Task list is empty!");
    } else {
        let mut order = 1u32;
        for task in tasks {
            println!("{}: {}", order, task);
            order += 1;
        }
    }

    Ok(())
}

fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?; //rewind the file before
    let tasks = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0))?; //rewind the file after
    Ok(tasks)
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let create_at = self.create_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]", self.text, create_at)
    }
}
