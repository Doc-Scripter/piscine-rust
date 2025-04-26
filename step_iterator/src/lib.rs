pub struct StepIterator<T> {
    current: T,
    end: T,
    step: T,
    done: bool,
}

use std::ops::Add;
use std::clone::Clone;

impl<T> StepIterator<T> 
where 
    T: Add<Output = T> + PartialOrd + Clone,
{
    pub fn new(beg: T, end: T, step: T) -> Self {
        StepIterator {
            current: beg,
            end,
            step,
            done: false,
        }
    }
}

impl<T> Iterator for StepIterator<T> 
where 
    T: Add<Output = T> + PartialOrd + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        // Save the current value to return
        let result = self.current.clone();

        // Calculate the next value
        let next = self.current.clone() + self.step.clone();

        // Check if we've reached or passed the end
        if next > self.end {
            self.done = true;
        } else {
            self.current = next;
        }

        Some(result)
    }
}
