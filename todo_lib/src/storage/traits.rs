use crate::{task::Task, todo::ToDoError};

pub trait Storage {
    fn save(&self, tasks: &[Task]) -> Result<(), ToDoError>;
    fn load(&self) -> Result<Vec<Task>, ToDoError>;
}
