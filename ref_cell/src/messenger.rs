use std::rc::Rc;

use crate::Worker;

pub struct Tracker<'a> {
    pub logger: Rc<dyn Logger+ 'a>,
    pub value: Rc<i32>,
    pub max: i32,
}

pub trait Logger  {
    fn info(&mut self, msg: &str);
    fn error(&mut self, msg: &str);
    fn warning(&mut self, msg: &str);
}
//&ref_cell::Worker
impl <'a>Tracker<'a> {
//   pub  fn new(logger: &'a mut dyn Logger,num:i32) -> Tracker<'a> {
  pub  fn new(logger: Rc<dyn Logger + 'a>,num:i32) -> Tracker<'a> {

       Tracker{
            logger,
            value: Rc::new(0),
            max:num,
        }
    }
   pub  fn set_value(&mut self) {
        let count = Rc::strong_count(&self.value);
        let percent =(count /self.max as usize)*100;
        if percent >= 100 {
            self.logger.error( "Error: you are over your quota!");
        }
        if percent >= 70 {
            self.logger.warning( format!("Warning: you have used up over {percent}% of your quota! Proceeds with precaution").as_str());
        }
    }
    pub fn peek(&mut self) {
        let count = Rc::strong_count(&self.value);
        let percent =(count /self.max as usize)*100;
        self.logger.info(format!("Info: you are using up to {percent}% of your quota").as_str());

    }
}
