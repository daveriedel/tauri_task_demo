use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    id: i32,
    name: String,
    description: String,
    category: String,
    status: String,
    created_at: String,
    updated_at: String,
    completed: bool,
    archived: bool,
    user: u32,
    deadline: String,
    tel: String,
}

pub struct TaskDTO {
    id: i32,
    name: String,
    description: String,
    category: String,
    status: String,
    created_at: String,
    updated_at: String,
    completed: u32,
    archived: u32,
    user: u32,
    deadline: String,
    tel: String,
}

impl From<Task> for TaskDTO {
    fn from(task: Task) -> Self {
        Self {
            id: task.id,
            name: task.name,
            description: task.description,
            category: task.category,
            status: task.status,
            created_at: task.created_at,
            updated_at: task.updated_at,
            completed: if task.completed { 1 } else { 0 },
            archived: if task.archived { 1 } else { 0 },
            user: task.user,
            deadline: task.deadline,
            tel: task.tel,
        }
    }
}

impl TaskDTO {
    pub fn id(&self) -> &i32 {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn status(&self) -> &str {
        &self.status
    }
    pub fn created_at(&self) -> &str {
        &self.created_at
    }
    pub fn updated_at(&self) -> &str {
        &self.updated_at
    }
    pub fn completed(&self) -> &u32 {
        &self.completed
    }
    pub fn archived(&self) -> &u32 {
        &self.archived
    }
    pub fn user(&self) -> &u32 {
        &self.user
    }
    pub fn deadline(&self) -> &str {
        &self.deadline
    }
    pub fn tel(&self) -> &str {
        &self.tel
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Task {{ {}, {}, {}, {}, {}, {}, {}, {}, {}, {} }}",
            self.id(),
            self.name(),
            self.description(),
            self.created_at(),
            self.updated_at(),
            self.completed(),
            self.archived(),
            self.user(),
            self.deadline(),
            self.tel()
        )
    }
}

impl fmt::Display for TaskDTO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Task {{ {}, {}, {}, {}, {}, {}, {}, {}, {} }}",
            self.id(),
            self.name(),
            self.description(),
            self.created_at(),
            self.updated_at(),
            self.completed(),
            self.archived(),
            self.user(),
            self.deadline()
        )
    }
}

impl Task {
    pub fn new(
        id: i32,
        name: String,
        description: String,
        category: String,
        status: String,
        created_at: Option<String>,
        updated_at: Option<String>,
        completed: Option<bool>,
        archived: Option<bool>,
        user: u32,
        deadline: String,
        tel: String,
    ) -> Self {
        let current_date = chrono::offset::Local::now();
        Self {
            id,
            name,
            description,
            category,
            status,
            created_at: created_at.unwrap_or(current_date.to_string()),
            updated_at: updated_at.unwrap_or(current_date.to_string()),
            completed: completed.unwrap_or(false),
            archived: archived.unwrap_or(false),
            user,
            deadline,
            tel,
        }
    }
    pub fn id(&self) -> &i32 {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn status(&self) -> &str {
        &self.status
    }
    pub fn created_at(&self) -> &str {
        &self.created_at
    }
    pub fn updated_at(&self) -> &str {
        &self.updated_at
    }
    pub fn completed(&self) -> &bool {
        &self.completed
    }
    pub fn archived(&self) -> &bool {
        &self.archived
    }
    pub fn user(&self) -> &u32 {
        &self.user
    }
    pub fn category(&self) -> &str {
        &self.category
    }
    pub fn deadline(&self) -> &str {
        &self.deadline
    }
    pub fn tel(&self) -> &str {
        &self.tel
    }
}
