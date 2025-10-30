use todo_app::ToDo;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut todo = ToDo::new();
    todo.add_task("Task 1".to_string());
    todo.add_task("Task 2".to_string());
    todo.add_task("Task 3".to_string());
    todo.complete_task(2)?;
    todo.complete_task(10)?;
    todo.delete_task(3)?;
    todo.list_tasks();
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}
