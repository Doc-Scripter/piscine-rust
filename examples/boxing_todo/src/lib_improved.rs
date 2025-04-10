//! Boxing Todo - A todo list management library
//!
//! This library provides functionality to create, load, and save todo lists.

extern crate chrono;

// Re-export types that users will need
pub use chrono::{DateTime, Utc};

// Optionally provide a type alias for common types
pub type Timestamp = DateTime<Utc>;

// Re-export models
pub use self::models::{TodoList, TodoItem};

/// Get a todo list from a file path
///
/// # Examples
///
/// ```
/// use boxing_todo::get_todo;
///
/// let todo_list = get_todo("todos.txt").expect("Failed to load todo list");
/// println!("Loaded {} items", todo_list.items.len());
/// ```
pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn std::error::Error>> {
    io::read_todo_file(path)
}

/// Save a todo list to a file
///
/// # Examples
///
/// ```
/// use boxing_todo::{TodoList, save_todo};
///
/// let mut list = TodoList::new();
/// list.add_item("Learn Rust".to_string(), false);
/// save_todo(&list, "todos.txt").expect("Failed to save todo list");
/// ```
pub fn save_todo(list: &TodoList, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    io::write_todo_file(list, path)
}

/// Get the current time as a Timestamp
///
/// This is a convenience function that wraps `Utc::now()`
pub fn now() -> Timestamp {
    Utc::now()
}

// Models module
mod models {
    use super::Timestamp;
    
    /// A list of todo items
    #[derive(Debug, Clone)]
    pub struct TodoList {
        /// The items in the list
        pub items: Vec<TodoItem>,
        /// When the list was last modified
        pub last_modified: Timestamp,
    }
    
    /// A single todo item
    #[derive(Debug, Clone)]
    pub struct TodoItem {
        /// The title of the todo item
        pub title: String,
        /// Whether the item is completed
        pub completed: bool,
        /// When the item was created
        pub created_at: Timestamp,
    }
    
    impl TodoList {
        /// Create a new, empty todo list
        pub fn new() -> Self {
            Self {
                items: Vec::new(),
                last_modified: super::now(),
            }
        }
        
        /// Add a new item to the list
        pub fn add_item(&mut self, title: String, completed: bool) {
            self.items.push(TodoItem {
                title,
                completed,
                created_at: super::now(),
            });
            self.last_modified = super::now();
        }
    }
}

// IO module for file operations
mod io {
    use std::error::Error;
    use std::fs;
    use std::path::Path;
    use super::models::TodoList;
    
    pub(crate) fn read_todo_file(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let path = Path::new(path);
        
        if !path.exists() {
            return Ok(TodoList::new());
        }
        
        // Implementation details...
        // For this example, just return a new list
        Ok(TodoList::new())
    }
    
    pub(crate) fn write_todo_file(list: &TodoList, path: &str) -> Result<(), Box<dyn Error>> {
        // Implementation details...
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_todo_list() {
        let mut list = TodoList::new();
        list.add_item("Test item".to_string(), false);
        assert_eq!(list.items.len(), 1);
    }
}