use std::path::PathBuf;

use crate::todo::ToDoError;
use crate::{storage::Storage, task::Task};

pub struct FileStorage {
    file_path: PathBuf,
}

impl FileStorage {
    pub fn new(file_name: &str) -> Result<Self, ToDoError> {
        let file_path = std::env::current_dir()
            .map_err(|e| ToDoError::IoError(format!("Unable to get current directory: {e}")))?
            .join(file_name);
        Ok(FileStorage { file_path })
    }
}

impl Storage for FileStorage {
    fn save(&self, tasks: &[Task]) -> Result<(), ToDoError> {
        let json_tasks = serde_json::to_string_pretty(tasks)?;
        std::fs::write(&self.file_path, json_tasks)?;
        Ok(())
    }

    fn load(&self) -> Result<Vec<crate::task::Task>, ToDoError> {
        if !self.file_path.exists() {
            return Ok(Vec::new());
        }

        let file_contents = std::fs::read_to_string(&self.file_path)?;

        let tasks = serde_json::from_str(&file_contents)?;

        Ok(tasks)
    }
}
