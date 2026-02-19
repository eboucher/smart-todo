# 🦀 Smart Todo CLI

An intelligent task manager for the terminal, written in Rust.

## Features
- **Add/Delete** tasks with auto-incrementing IDs.
- **Prioritization** (Low, Medium, High) with **color-coded** display.
- **Persistence**: Automatically saves/loads todos from `todos.json`.
- **Status**: Mark tasks as "done" (visualized with strikethrough).
- **Summary**: Shows total/done/pending counts.

## Usage

### Add a Task
```bash
cargo run -- add "Buy milk"
cargo run -- add "Urgent Deadline" --priority high
```

### List Tasks
```bash
cargo run -- list
```
*Output:*
```
📋 Your Todos:
────────────────────────────────────────
  [○] #1 - Buy milk (Medium)
  [○] #2 - Urgent Deadline (High)
────────────────────────────────────────
  2 total, 0 done, 2 pending
```

### Mark as Done
```bash
cargo run -- done 1
```

## Tech Stack
- **Rust** (Ownership, Enums, Structs)
- **Clap** (CLI Parser)
- **Serde** (JSON Persistence)
- **Colored** (Terminal Colors)

## technical Documentation
For a deep dive into the code architecture, see [TECHNICAL.md](TECHNICAL.md).
