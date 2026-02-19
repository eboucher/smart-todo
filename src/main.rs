use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::PathBuf;

// ─── Data Model ────────────────────────────────────────────

/// Priority levels for a todo item.
#[derive(Debug, Clone, Copy, ValueEnum, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

use colored::*;

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "{}", "Low".green()),
            Priority::Medium => write!(f, "{}", "Medium".yellow()),
            Priority::High => write!(f, "{}", "High".red()),
        }
    }
}

/// A single todo item.
#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub priority: Priority,
    pub done: bool,
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.done { "✓".green() } else { "○".red() };
        let title = if self.done {
            self.title.strikethrough().dimmed()
        } else {
            self.title.normal()
        };
        
        write!(
            f,
            "[{}] #{} - {} ({})",
            status, self.id, title, self.priority
        )
    }
}

// ─── Storage ───────────────────────────────────────────────

/// Returns the path to the todos.json file (in the current directory).
fn get_storage_path() -> PathBuf {
    PathBuf::from("todos.json")
}

/// Load all todos from the JSON file. Returns an empty Vec if the file doesn't exist.
fn load_todos() -> Vec<Todo> {
    let path = get_storage_path();
    if !path.exists() {
        return Vec::new();
    }
    let data = fs::read_to_string(&path).expect("Failed to read todos.json");
    serde_json::from_str(&data).expect("Failed to parse todos.json")
}

/// Save all todos to the JSON file (pretty-printed).
fn save_todos(todos: &[Todo]) {
    let path = get_storage_path();
    let data = serde_json::to_string_pretty(todos).expect("Failed to serialize todos");
    fs::write(&path, data).expect("Failed to write todos.json");
}

/// Determine the next available ID by finding the current maximum.
fn next_id(todos: &[Todo]) -> usize {
    todos.iter().map(|t| t.id).max().unwrap_or(0) + 1
}

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
