//https://docs.microsoft.com/zh-cn/learn/modules/rust-create-command-line-program/3-cli-module?source=learn
use structopt::StructOpt;
use anyhow::{anyhow, Ok};
use std::path::PathBuf;
mod cli;
mod tasks;

use cli::{Action::*, CommandlineArgs};
use tasks::Task;

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    //get cli arguments
    let CommandlineArgs { 
        action, 
        journal_file 
    } = CommandlineArgs::from_args();

    //unpack the journal file
    let journal_file = journal_file.expect("Failed to find journal file");

    //Perfom the action
    match action {
        Add { task: text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }?;

    Ok(())
}


 
