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
        for task in self.tasks.iter() {
            let status = if task.completed {
                "Completed"
            } else {
                "Pending"
            };
            println!("[{}] {} - {}", task.id, task.title, status);
        }
    }
    pub fn task_done(&mut self, id: &str) {
        let task_id = match Uuid::parse_str(id) {
            Ok(id) => id,
            Err(_) => {
                eprintln!("Wrong task id");
                return;
            }
        };

        match self.tasks.iter_mut().find(|t| t.id == task_id) {
            Some(task) => {
                task.completed = true;
            }
            None => {
                eprintln!("Wrong task id");
                return;
            }
        }
    }
    pub fn task_delete(&mut self, id: &str) {
        let task_id = match Uuid::parse_str(id) {
            Ok(id) => id,
            Err(_) => {
                eprintln!("Wrong task id");
                return;
            }
        };

        if let Some(index) = self.tasks.iter().position(|task| task.id == task_id) {
            self.tasks.remove(index);
        } else {
            eprintln!("Could not remove tasks");
            return;
        }
    }
}
