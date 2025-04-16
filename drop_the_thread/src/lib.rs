use std::{cell::{Cell, RefCell}, panic};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>
}

impl Workers {
    pub fn new() -> Workers {
        Workers { drops: Cell::new(0+1) , states: RefCell::new(vec![true]) }
    }
    pub fn new_worker(&self, c: String) -> (usize, Thread) {
        (self.drops.get(),Thread::new_thread(self.drops.get(), c, &self))
    }
    pub fn track_worker(&self) -> usize {
        self.states.borrow().len()
    }
    pub fn is_dropped(&self, id: usize) -> bool {
        if self.drops.get()==id{
            self.states.borrow()[0];
            return true;
        } else {
            return false;
        }
    }
    pub fn add_drop(&self, id: usize) {
        if self.drops.get()==id{
            if !self.states.borrow()[0]{
                self.states.borrow_mut().push(true);
            }else{
                panic!("{id} is already dropped")
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    pid:usize,
    cmd:String,
    parent:RefCell<&'a Workers>,
}

impl<'a> Thread<'a> {
    pub fn new_thread(p: usize, c: String, t: &'a Workers) -> Thread<'a> {
        Thread { pid: p, cmd: c, parent: RefCell::new(t) }
    }
    pub fn skill(self) {
        let _dropped=self;
    }
}

impl <'a>Drop for Thread<'a> {
    fn drop(&mut self) {
        Workers::add_drop(self.parent.get_mut(), self.pid);
    }
}