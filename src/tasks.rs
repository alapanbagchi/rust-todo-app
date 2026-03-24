use uuid::Uuid;

pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
}
