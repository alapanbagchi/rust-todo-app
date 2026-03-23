#[derive(Debug)]
pub enum Command {
    Add { title: String },
    List,
    Done { id: u32 },
    Delete { id: u32 },
}
