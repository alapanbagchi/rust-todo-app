/// Represents all supported CLI commands.
///
/// Each variant corresponds to a user-invoked command and may carry
/// additional data required to execute that command.
///
/// This enum acts as the bridge between raw CLI input (strings)
/// and structured application logic.
#[derive(Debug)]
pub enum Command {
    /// Create a new task with the given title.
    /// The title is free-form text and may contain multiple words.
    Add { title: String },

    /// List all existing tasks.
    /// Additional filtering/sorting may be applied later.
    List,

    /// Mark a task as completed using its unique ID.
    Done { id: String },

    /// Delete a task by its unique ID.
    Delete { id: String },
}
