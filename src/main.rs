use todo_app::FileStorage;
use todo_app::Storage;
use todo_app::ToDo;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let fss = FileStorage::new("tasks.json")?;
    let mut todo = ToDo::new();
    // todo.add_task("Task 1".to_string());
    // todo.add_task("Task 2".to_string());
    // todo.add_task("Task 3".to_string());
    // todo.complete_task(2)?;
    // todo.complete_task(1)?;
    // todo.delete_task(3)?;

    todo.tasks = fss.load()?;
    todo.list_tasks();

    fss.save(&todo.tasks)?;

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}
