pub use std::collections::HashMap;
pub use std::cell::RefCell;
use std::rc::Rc;

pub use messenger::*;

pub mod messenger;

#[derive(Debug)]
pub struct Worker {
    pub track_value: Rc<i32>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
    pub all_messages: RefCell<Vec<String>>,
}

impl Worker {
pub    fn new(num:i32) -> Worker {
        Worker {
            track_value: Rc::new(num),
            mapped_messages: RefCell::new(HashMap::new()),
            all_messages: RefCell::new(Vec::new()),
        }
    }
  
}

impl Logger for Worker {
   fn info(&self, msg: &str) {
        self.mapped_messages.borrow_mut().insert("Info".to_owned(),msg.to_owned());
        self.all_messages.borrow_mut().push(format!("{}",msg.to_owned()));
    }
    fn error(&self, msg: &str) {
        self.mapped_messages.borrow_mut().insert("Error".to_owned(),msg.to_owned());
        self.all_messages.borrow_mut().push(format!("{}",msg.to_owned()));

    }
    fn warning(&self, msg: &str) {
        self.mapped_messages.borrow_mut().insert("Warning".to_owned(),msg.to_owned());
        self.all_messages.borrow_mut().push(format!("{}",msg.to_owned()));

    }
}
