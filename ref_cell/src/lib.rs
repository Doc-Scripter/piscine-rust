pub use std::collections::HashMap;

pub use messenger::*;

pub mod messenger;

pub struct Worker {
    pub track_value: i32,
    pub mapped_messages: HashMap<String, String>,
    pub all_message: Vec<String>,
}

impl Worker {
pub    fn new(num:i32) -> Worker {
        Worker {
            track_value: num,
            mapped_messages: HashMap::new(),
            all_message: Vec::new(),
        }
    }
  
}

impl Logger for Worker {
   fn info(&mut self, msg: &str) {
        self.mapped_messages.insert("info".to_owned(),msg.to_owned());

    }
    fn error(&mut self, msg: &str) {
        self.mapped_messages.insert("error".to_owned(),msg.to_owned());

    }
    fn warning(&mut self, msg: &str) {
        self.mapped_messages.insert("warning".to_owned(),msg.to_owned());

    }
}
