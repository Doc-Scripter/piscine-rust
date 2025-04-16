pub use std::collections::HashMap;
pub use std::cell::RefCell;
pub use std::rc::Rc;

pub use messenger::*;

pub mod messenger;

#[derive(Debug)]
pub struct Worker {
    pub track_value: Rc<usize>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
    pub all_messages: RefCell<Vec<String>>,
}

impl Worker {
pub    fn new(num:usize) -> Worker {
        Worker {
            track_value: Rc::new(num),
            mapped_messages: RefCell::new(HashMap::new()),
            all_messages: RefCell::new(Vec::new()),
        }
    }
  
}

impl Logger for Worker {
   fn info(&self, msg: &str) {
        self.mapped_messages.borrow_mut().insert("Info".to_owned(),msg.to_owned().drain(6..msg.len()).collect());
        self.all_messages.borrow_mut().push(format!("{}",msg.to_owned()));
    }
    fn error(&self, msg: &str) {
        self.mapped_messages.borrow_mut().insert("Error".to_owned(),msg.to_owned().drain(7..msg.len()).collect());
        self.all_messages.borrow_mut().push(format!("{}",msg.to_owned()));

    }
    fn warning(&self, msg: &str) {
        self.mapped_messages.borrow_mut().insert("Warning".to_owned(),msg.to_owned().drain(9..msg.len()).collect());
        self.all_messages.borrow_mut().push(format!("{}",msg.to_owned()));

    }
}

/*

running 3 tests
test tests::test_module ... ok
test tests::test_module_usage_hasmap ... FAILED
test tests::test_module_usage_vector ... ok

failures:

---- tests::test_module_usage_hasmap stdout ----

thread 'tests::test_module_usage_hasmap' panicked at src/main.rs:109:9:
assertion `left == right` failed
  left: "Warning: you have used up over 75% of your quota! Proceeds with precaution"
 right: "you have used up over 75% of your quota! Proceeds with precaution"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

            ^^^^^^^^^

*/