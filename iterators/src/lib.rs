#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Collatz;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 0 {
            return None;
        }
        
        let current = self.v;
        if self.v == 1 {
            self.v = 0;
        } else if self.v % 2 == 0 {
            self.v /= 2;
        } else {
            self.v = 3 * self.v + 1;
        }
        
        Some(Collatz { v: current })
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }

    /// Return the number of Collatz steps (transitions) to reach 1.
    pub fn count(self) -> usize {
        // call the Iterator::count to get how many values are yielded,
        // then subtract 1 to convert yields → transitions
        <Self as Iterator>::count(self).saturating_sub(1)
    }
}

/// A standalone function to compute the number of Collatz steps.
pub fn collatz(n: u64) -> usize {
    if n == 0 || n == 1 {
        return 0;
    }
    
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