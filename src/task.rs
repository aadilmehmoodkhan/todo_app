use std::fmt::Display;

#[derive(Debug)]
pub enum ToDoError {
    TaskNotFound(u32),
}

impl Display for ToDoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToDoError::TaskNotFound(id) => write!(f, "ToDo item with id {id} not found"),
        }
    }
}

impl std::error::Error for ToDoError {}

#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ID: {id}, Desc: {desc}, Completed: {comp}",
            id = self.id,
            desc = self.description,
            comp = self.completed
        )
    }
}

impl Task {
    pub fn new(id: u32, description: String) -> Self {
        Self {
            id,
            description,
            completed: false,
        }
    }
}
