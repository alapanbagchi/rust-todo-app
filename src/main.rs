mod command;
mod parser;

use std::env;

use crate::parser::parse_args;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match parse_args(args) {
        Ok(command) => {
            println!("Parsed command {:?}", command);
        }
        Err(err) => {
            println!("ERROR: {err}");
        }
    }
}
