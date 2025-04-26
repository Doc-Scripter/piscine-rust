
#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        // Return None if we've reached 0 or if we're starting with 0
        if self.v == 0 {
            return None;
        }
        
        // Store the current value to return
        let current = self.v;
        
        // Calculate the next value in the sequence
        if self.v == 1 {
            // If we've reached 1, we're done with the sequence
            self.v = 0;
        } else if self.v % 2 == 0 {
            // If even, divide by 2
            self.v = self.v / 2;
        } else {
            // If odd, multiply by 3 and add 1
            self.v = 3 * self.v + 1;
        }
        
        // Return the value we started with in this iteration
        Some(current)
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    // Count the steps until we reach 1
    // We subtract 1 because we don't count the final 1 as a step
    if n == 0 {
        return 0;
    }
    
    let steps = Collatz::new(n)
        .take_while(|&x| x != 1)
        .count();
    
    steps
}
