mod todo;
mod storage;

use clap::{Parser, Subcommand};
use crate::todo::{Priority, Todo};
use crate::storage::{load_todos, save_todos, next_id};

// ─── CLI Definition ────────────────────────────────────────

/// 🦀 Smart Todo CLI — An intelligent task manager for the terminal.
#[derive(Parser)]
#[command(name = "todo", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new todo
    Add {
        /// The task description
        title: String,

        /// Priority level (low, medium, high)
        #[arg(short, long, default_value = "medium")]
        priority: Priority,
    },

    /// List all todos
    List,

    /// Mark a todo as done
    Done {
        /// The ID of the todo to complete
        id: usize,
    },
}

// ─── Main ──────────────────────────────────────────────────

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { title, priority } => {
            let mut todos = load_todos();
            let id = next_id(&todos);
            let todo = Todo {
                id,
                title: title.clone(),
                priority,
                done: false,
            };
            todos.push(todo);
            save_todos(&todos);
            println!("✅ Added todo #{}: \"{}\" ({})", id, title, priority);
        }
        Commands::List => {
            let todos = load_todos();
            if todos.is_empty() {
                println!("📋 No todos yet. Add one with: todo add \"My task\"");
                return;
            }
            println!("📋 Your Todos:");
            println!("{}", "─".repeat(40));
            for todo in &todos {
                println!("  {}", todo);
            }
            println!("{}", "─".repeat(40));
            let done_count = todos.iter().filter(|t| t.done).count();
            println!("  {} total, {} done, {} pending",
                todos.len(), done_count, todos.len() - done_count);
        }
        Commands::Done { id } => {
            let mut todos = load_todos();
            let found = if let Some(t) = todos.iter_mut().find(|t| t.id == id) {
                t.done = true;
                println!("✅ Marked todo #{} as done: \"{}\"", id, t.title);
                true
            } else {
                false
            };

            if found {
                save_todos(&todos);
            } else {
                eprintln!("❌ No todo found with ID #{}", id);
                std::process::exit(1);
            }
        }
    }
}
