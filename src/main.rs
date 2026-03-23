mod command;
mod parser;

use std::env;

use crate::command::Command;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
}
