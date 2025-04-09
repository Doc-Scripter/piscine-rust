mod err;

pub use err::{ParseErr, ReadErr};

pub use std::{error::Error, fs, path::Path};
pub use json::{JsonValue,object};

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        // Read file content
        let content = fs::read_to_string(Path::new(path))
            .map_err(|e| Box::new(ReadErr { child_err: Box::new(e) }) as Box<dyn Error>)?;

        // Parse JSON
        let parsed = json::parse(&content)
            .map_err(|e| Box::new(ParseErr::Malformed(Box::new(e))) as Box<dyn Error>)?;

        // Extract title
        let title = parsed["title"].as_str().ok_or_else(|| {
            Box::new(ParseErr::Malformed(Box::new(
                std::fmt::Error, // Dummy internal error to satisfy trait
            ))) as Box<dyn Error>
        })?.to_string();

        // Extract tasks
        let tasks_json = &parsed["tasks"];
        if !tasks_json.is_array() {
            return Err(Box::new(ParseErr::Malformed(Box::new(
                std::fmt::Error,
            ))));
        }

        let mut tasks = Vec::new();
        for task_json in tasks_json.members() {
            let id = task_json["id"].as_u32();
            let desc = task_json["description"].as_str();
            let level = task_json["level"].as_u32();

            match (id, desc, level) {
                (Some(id), Some(description), Some(level)) => {
                    tasks.push(Task {
                        id,
                        description: description.to_string(),
                        level,
                    });
                }
                _ => {
                    return Err(Box::new(ParseErr::Malformed(Box::new(
                        std::fmt::Error,
                    ))));
                }
            }
        }

        if tasks.is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }

        Ok(TodoList { title, tasks })
    }
}
