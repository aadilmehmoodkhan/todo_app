use crate::task::{Task, ToDoError};

#[derive(Debug)]
pub struct ToDo {
    tasks: Vec<Task>,
}

impl ToDo {
    pub const fn new() -> Self {
        ToDo { tasks: Vec::new() }
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
