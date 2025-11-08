use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{storage::Storage, task::Task};

#[derive(Debug, Serialize, Deserialize)]
pub enum ToDoError {
    TaskNotFound(u32),
    StorageError(String),
    JsonError(String),
    IoError(String),
}

impl std::error::Error for ToDoError {}

impl From<std::io::Error> for ToDoError {
    fn from(value: std::io::Error) -> Self {
        ToDoError::IoError(value.to_string())
    }
}

impl From<serde_json::error::Error> for ToDoError {
    fn from(value: serde_json::error::Error) -> Self {
        ToDoError::JsonError(value.to_string())
    }
}

impl Display for ToDoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToDoError::TaskNotFound(id) => write!(f, "ToDo item with id {id} not found"),
            ToDoError::StorageError(error) => write!(f, "Storage error: {error}"),
            ToDoError::JsonError(error) => write!(f, "JSON error: {error}"),
            ToDoError::IoError(error) => write!(f, "IO error: {error}"),
        }
    }
}

#[derive(Debug)]
pub struct ToDo<S: Storage> {
    tasks: Vec<Task>,
    storage: S,
}

impl<S: Storage> ToDo<S> {
    pub fn new(storage: S) -> Result<Self, ToDoError> {
        Ok(ToDo {
            tasks: storage.load()?,
            storage,
        })
    }

    pub fn save(&self) -> Result<(), ToDoError> {
        self.storage.save(&self.tasks)?;
        Ok(())
    }

    pub fn get_max_task_id(&self) -> u32 {
        match self.tasks.iter().max_by_key(|t| t.id) {
            Some(t) => t.id + 1,
            _ => 1,
        }
    }

    fn get_task_by_id(&mut self, task_id: u32) -> Result<&mut Task, ToDoError> {
        match self.tasks.iter_mut().find(|t| t.id == task_id) {
            Some(t) => Ok(t),
            None => Err(ToDoError::TaskNotFound(task_id)),
        }
    }

    pub fn add_task(&mut self, description: String) {
        self.tasks
            .push(Task::new(self.get_max_task_id(), description));
    }

    pub fn complete_task(&mut self, task_id: u32) -> Result<(), ToDoError> {
        let task = self.get_task_by_id(task_id)?;
        task.completed = true;
        Ok(())
    }

    pub fn delete_task(&mut self, id: u32) -> Result<Task, ToDoError> {
        match self.tasks.iter().position(|t| t.id == id) {
            Some(t) => Ok(self.tasks.remove(t)),
            None => Err(ToDoError::TaskNotFound(id)),
        }
    }

    pub fn list_tasks(&self) {
        self.tasks.iter().for_each(|t| println!("{}", t));
    }
}
