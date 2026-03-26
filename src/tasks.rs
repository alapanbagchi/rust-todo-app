use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Task {
    #[serde(rename = "ID")]
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
}
