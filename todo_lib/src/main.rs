use todo_lib::FileStorage;
use todo_lib::ToDo;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let fss = FileStorage::new("tasks.json")?;
    let mut todo = ToDo::new(fss)?;
    // todo.add_task("Task 1".to_string());
    // todo.add_task("Task 2".to_string());
    // todo.add_task("Task 3".to_string());
    // todo.complete_task(2)?;
    // todo.complete_task(1)?;
    // todo.delete_task(3)?;

    todo.list_tasks();
    todo.add_task("I should sleep and get some rest".to_string());
    todo.save()?;

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}
