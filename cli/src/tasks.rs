use chrono::{DateTime, Utc};
use serde::{Deserialize,Serialize};
use std::fs::{File, Openoptions};
use std::path::PathBuf;
use std::io::{Result, Seek, SeekFrom};

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

pub fn add_task(joural_path: PathBuf, task: Task) -> Result<()> {
    //open file
    let mut file = Openoptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;
    
    //consume the file's content as a vector of tasks
    let mut tasks: Vec<Task> = match serde_json::from_reader(&file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };

    //rewind the file after reading from it
    file.seek(SeekFrom::Start(0))?;

    //write the modified task list back into the file
    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}