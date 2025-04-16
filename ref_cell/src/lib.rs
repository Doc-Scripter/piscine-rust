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

/*
   Compiling ref_cell v0.1.0 (/jail/solutions/ref_cell)
   Compiling ref_cell_test v0.1.0 (/jail/tests/ref_cell_test)
error[E0308]: mismatched types
  --> src/main.rs:82:20
   |
82 |         track.peek(&l.value); // 40%
   |               ---- ^^^^^^^^ expected `&Rc<i32>`, found `&Rc<usize>`
   |               |
   |               arguments to this method are incorrect
   |
   = note: expected reference `&ref_cell::Rc<i32>`
              found reference `&ref_cell::Rc<usize>`
note: method defined here
  --> /jail/solutions/ref_cell/src/messenger.rs:37:12
   |
37 |     pub fn peek(&self,num: &Rc<i32>) {
   |            ^^^^

error[E0308]: mismatched types
  --> src/main.rs:85:25
   |
85 |         track.set_value(&l.value); // 80%
   |               --------- ^^^^^^^^ expected `&Rc<i32>`, found `&Rc<usize>`
   |               |
   |               arguments to this method are incorrect
   |
   = note: expected reference `&ref_cell::Rc<i32>`
              found reference `&ref_cell::Rc<usize>`
note: method defined here
  --> /jail/solutions/ref_cell/src/messenger.rs:26:12
   |
26 |    pub  fn set_value(&self,num: &Rc<i32>) {
   |            ^^^^^^^^^

error[E0308]: mismatched types
  --> src/main.rs:87:25
   |
87 |         track.set_value(&l.value); // 100%
   |               --------- ^^^^^^^^ expected `&Rc<i32>`, found `&Rc<usize>`
   |               |
   |               arguments to this method are incorrect
   |
   = note: expected reference `&ref_cell::Rc<i32>`
              found reference `&ref_cell::Rc<usize>`
note: method defined here
  --> /jail/solutions/ref_cell/src/messenger.rs:26:12
   |
26 |    pub  fn set_value(&self,num: &Rc<i32>) {
   |            ^^^^^^^^^

*/