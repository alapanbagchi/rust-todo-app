use crate::command::Command;

/// Parses CLI arguments into a `Command`.
///
/// This function expects arguments excluding the binary name.
/// Example input:
///     ["add", "buy", "milk"]
///
/// Returns:
/// - `Ok(Command)` if parsing succeeds
/// - `Err(String)` with a user-friendly message if parsing fails

pub fn parse_args(args: Vec<String>) -> Result<Command, String> {
    if args.is_empty() {
        return Err(String::from("Please provide a valid commmand"));
    }
    let command = args[0].as_str().trim();

    match command {
        "add" => {
            if args.len() < 2 {
                return Err(String::from("The title of the task is missing"));
            }

            let title = args[1..].join(" ");

            Ok(Command::Add { title })
        }
        "list" => Ok(Command::List),
        "done" => {
            if args.len() < 2 {
                return Err(String::from("The id of the task is missing"));
            }

            let id = args[1]
                .parse::<u32>()
                .map_err(|_| String::from("Invalid task id"))?;

            Ok(Command::Done { id })
        }
        "delete" => {
            if args.len() < 2 {
                return Err(String::from("The id of the task is missing"));
            }

            let id = args[1]
                .parse::<u32>()
                .map_err(|_| String::from("Invalid task id"))?;

            Ok(Command::Delete { id })
        }
        _ => Err(String::from("Please provide a valid command")),
    }
}
