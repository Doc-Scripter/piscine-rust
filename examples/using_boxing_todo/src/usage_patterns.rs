// This file shows different patterns for working with chrono types
// from the boxing_todo library

use std::error::Error;

// Pattern 1: When boxing_todo re-exports everything you need
fn pattern1() -> Result<(), Box<dyn Error>> {
    use boxing_todo::{get_todo, save_todo, TodoList, Utc, DateTime};
    
    let todo_list = get_todo("todos.txt")?;
    
    // Work with DateTime<Utc> directly
    let now: DateTime<Utc> = Utc::now();
    
    // Check if the list is recent
    let one_day_ago = now - chrono::Duration::days(1);
    if todo_list.last_modified > one_day_ago {
        println!("Todo list is recent");
    }
    
    Ok(())
}

// Pattern 2: When boxing_todo doesn't re-export chrono types
fn pattern2() -> Result<(), Box<dyn Error>> {
    use boxing_todo::{get_todo, save_todo, TodoList};
    // Directly depend on chrono
    use chrono::{DateTime, Utc, Duration};
    
    let todo_list = get_todo("todos.txt")?;
    
    // Work with DateTime<Utc> directly
    let now: DateTime<Utc> = Utc::now();
    
    // Check if the list is recent
    let one_day_ago = now - Duration::days(1);
    if todo_list.last_modified > one_day_ago {
        println!("Todo list is recent");
    }
    
    Ok(())
}

// Pattern 3: When boxing_todo re-exports types through a module
fn pattern3() -> Result<(), Box<dyn Error>> {
    use boxing_todo::{get_todo, save_todo, TodoList};
    use boxing_todo::datetime::{DateTime, Utc};
    
    let todo_list = get_todo("todos.txt")?;
    
    // Work with DateTime<Utc> directly
    let now: DateTime<Utc> = Utc::now();
    
    // If boxing_todo provides helper functions
    let now_alt = boxing_todo::datetime::now();
    
    Ok(())
}

// Pattern 4: Using type aliases if boxing_todo provides them
fn pattern4() -> Result<(), Box<dyn Error>> {
    use boxing_todo::{get_todo, save_todo, TodoList, Timestamp};
    
    let todo_list = get_todo("todos.txt")?;
    
    // If boxing_todo defines a Timestamp type alias for DateTime<Utc>
    let now: Timestamp = boxing_todo::now();
    
    Ok(())
}