use std::error::Error;

use clap::{Parser, Subcommand};
use todo_lib::{FileStorage, ToDo};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    operation: Operation,
}

#[derive(Subcommand)]
enum Operation {
    List,
    Add { name: String },
    Delete { id: u32 },
    Complete { id: u32 },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let storage = FileStorage::new("todo.json")?;
    let mut todo = ToDo::new(storage)?;

    match cli.operation {
        Operation::List => {
            println!("{:#?}", todo.list_tasks());
            return Ok(());
        }
        Operation::Add { name } => todo.add_task(name),
        Operation::Complete { id } => todo.complete_task(id)?,
        Operation::Delete { id } => {
            todo.delete_task(id)?;
        }
    }
    todo.save()?;

    Ok(())
}
