mod command;
mod manager;
mod parser;
mod tasks;
use std::{env, fs, task};

use crate::{manager::TaskManager, parser::parse_args, tasks::Task};

fn load_tasks() -> Vec<Task> {
    let file_name = "./src/tasks.json".to_string();
    let contents = fs::read_to_string(file_name).expect("File not found");
    let parsed_contents: Vec<Task> = serde_json::from_str(&contents).expect("asd");
    parsed_contents
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let tasks: Vec<Task> = load_tasks();
    let mut task_manager = TaskManager::new(tasks);
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
            command::Command::Delete { id } => {
                task_manager.task_delete(&id);
                println!("Task deleted!");
            }
            _ => eprintln!("ERROR: Command not implemented yet"),
        },
        Err(err) => {
            println!("ERROR: {err}");
        }
    }
}
