
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        // Determine which components to swap based on their values
        if self.r == first && self.g == second {
            // Swap r and g
            std::mem::swap(&mut self.r, &mut self.g);
        } else if self.r == first && self.b == second {
            // Swap r and b
            std::mem::swap(&mut self.r, &mut self.b);
        } else if self.r == first && self.a == second {
            // Swap r and a
            std::mem::swap(&mut self.r, &mut self.a);
        } else if self.g == first && self.b == second {
            // Swap g and b
            std::mem::swap(&mut self.g, &mut self.b);
        } else if self.g == first && self.a == second {
            // Swap g and a
            std::mem::swap(&mut self.g, &mut self.a);
        } else if self.b == first && self.a == second {
            // Swap b and a
            std::mem::swap(&mut self.b, &mut self.a);
        }
        
        self
    }
}
