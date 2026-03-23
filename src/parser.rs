use crate::command::Command;

pub fn parse_args(args: Vec<String>) -> Result<Command, String> {
    if args.is_empty() {
        return Err("No command provided".into());
    }

    let command = args[0].as_str();

    match command {
        "add" => {
            if args.len() < 2 {
                return Err("Missing task title".into());
            }

            Ok(Command::Add {
                title: args[1].clone(),
            })
        }
        "list" => Ok(Command::List),

        "done" => {
            if args.len() < 2 {
                return Err("Missing task ID".into());
            }

            let id = args[1].parse::<u32>().map_err(|_| "Invalid ID")?;
            Ok(Command::Done { id })
        }
        "delete" => {
            if args.len() < 2 {
                return Err("Missing task ID".into());
            }

            let id = args[1].parse::<u32>().map_err(|_| "Invalid ID")?;

            Ok(Command::Delete { id })
        }

        _ => Err("Unknown Command".into()),
    }
}
