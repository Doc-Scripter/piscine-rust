use std::{
    cell::{Cell, RefCell},
    panic,
};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}

impl Workers {
    pub fn new() -> Workers {
        Workers {
            drops: Cell::new(0),
            states: RefCell::new(Vec::new()),
        }
    }

    pub fn new_worker(&self, c: String) -> (usize, Thread) {
        let id = self.states.borrow().len(); // Get ID from states length
        self.states.borrow_mut().push(false);
        (id, Thread::new_thread(id, c, self))
    }

    pub fn track_worker(&self) -> usize {
        self.states.borrow().len()
    }

    pub fn is_dropped(&self, id: usize) -> bool {
        self.states.borrow().get(id).copied().unwrap_or(false)
    }

    pub fn add_drop(&self, id: usize) {
        let mut states = self.states.borrow_mut();
        if let Some(state) = states.get_mut(id) {
            if *state {
                panic!("{id} is already dropped");
            }
            *state = true;
            self.drops.set(self.drops.get() + 1); // Increment drops counter here
        }
    
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    pid: usize,
    cmd: String,
    parent: RefCell<&'a Workers>,
}

impl<'a> Thread<'a> {
    pub fn new_thread(p: usize, c: String, t: &'a Workers) -> Thread<'a> {
        Thread {
            pid: p,
            cmd: c,
            parent: RefCell::new(t),
        }
    }
    pub fn skill(self) {
        let _dropped = self;
    }
}

impl<'a> Drop for Thread<'a> {
    fn drop(&mut self) {
        self.parent.borrow().add_drop(self.pid);
    }
}
