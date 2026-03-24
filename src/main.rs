mod command;
mod manager;
mod parser;
mod tasks;
use std::{env, task};

use crate::{manager::TaskManager, parser::parse_args};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut task_manager = TaskManager::new();
    match parse_args(args) {
        Ok(command) => match command {
            command::Command::List => {
                task_manager.list_tasks();
            }
            command::Command::Add { title } => {
                task_manager.add_task(&title);
                println!("Task successfully added!")
            }
            command::Command::Done { id } => {
                task_manager.task_done(&id);
                println!("Task marked as done!");
            }
            _ => eprintln!("ERROR: Command not implemented yet"),
        },
        Err(err) => {
            println!("ERROR: {err}");
        }
    }
}
