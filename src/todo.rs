use serde::{Deserialize, Serialize};
use clap::ValueEnum;
use colored::*;
use std::fmt;

/// Priority levels for a todo item.
#[derive(Debug, Clone, Copy, ValueEnum, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

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
