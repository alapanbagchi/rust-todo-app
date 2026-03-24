use uuid::Uuid;

use crate::tasks::Task;

pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }
    pub fn add_task(&mut self, title: &str) {
        let task = Task {
            id: Uuid::new_v4(),
            title: title.to_string(),
            completed: false,
        };
        self.tasks.push(task);
    }
    pub fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks found");
            return;
        }
        for (idx, task) in self.tasks.iter().enumerate() {
            let status = if task.completed {
                "Completed"
            } else {
                "Pending"
            };
            println!("[{}] {} - {}", task.id, task.title, status);
        }
    }
}
