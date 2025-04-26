#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Collatz;
    
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
        
        // Return the value we started with in this iteration as a Collatz struct
        Some(Collatz { v: current })
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    // Handle edge cases
    if n == 0 {
        return 0;
    }
    
    if n == 1 {
        return 0;
    }
    
    // Count the steps until we reach 1
    let mut steps = 0;
    let mut current = n;
    
    while current != 1 {
        if current % 2 == 0 {
            current /= 2;
        } else {
            current = 3 * current + 1;
        }
        steps += 1;
    }
    
    steps
}
