use std::fs;
use std::path::PathBuf;
use crate::todo::Todo;

/// Returns the path to the todos.json file (in the current directory).
fn get_storage_path() -> PathBuf {
    PathBuf::from("todos.json")
}

/// Load all todos from the JSON file. Returns an empty Vec if the file doesn't exist.
pub fn load_todos() -> Vec<Todo> {
    let path = get_storage_path();
    if !path.exists() {
        return Vec::new();
    }
    let data = fs::read_to_string(&path).expect("Failed to read todos.json");
    serde_json::from_str(&data).expect("Failed to parse todos.json")
}

/// Save all todos to the JSON file (pretty-printed).
pub fn save_todos(todos: &[Todo]) {
    let path = get_storage_path();
    let data = serde_json::to_string_pretty(todos).expect("Failed to serialize todos");
    fs::write(&path, data).expect("Failed to write todos.json");
}

/// Determine the next available ID by finding the current maximum.
pub fn next_id(todos: &[Todo]) -> usize {
    todos.iter().map(|t| t.id).max().unwrap_or(0) + 1
}
