mod command;
mod manager;
mod parser;
mod tasks;
use std::env;

use crate::{manager::TaskManager, parser::parse_args};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let task_manager = TaskManager::new();
    match parse_args(args) {
        Ok(command) => match command {
            command::Command::List => {
                task_manager.list_tasks();
            }
            _ => eprintln!("ERROR: Command not implemented yet"),
        },
        Err(err) => {
            println!("ERROR: {err}");
        }
    }
}
