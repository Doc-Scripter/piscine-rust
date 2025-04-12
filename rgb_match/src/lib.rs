#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        match (first, second) {
            (r, g) if r == self.r && g == self.g => std::mem::swap(&mut self.r, &mut self.g),
            (r, b) if r == self.r && b == self.b => std::mem::swap(&mut self.r, &mut self.b),
            (r, a) if r == self.r && a == self.a => std::mem::swap(&mut self.r, &mut self.a),
            (g, r) if g == self.g && r == self.r => std::mem::swap(&mut self.g, &mut self.r),
            (g, b) if g == self.g && b == self.b => std::mem::swap(&mut self.g, &mut self.b),
            (g, a) if g == self.g && a == self.a => std::mem::swap(&mut self.g, &mut self.a),
            (b, r) if b == self.b && r == self.r => std::mem::swap(&mut self.b, &mut self.r),
            (b, g) if b == self.b && g == self.g => std::mem::swap(&mut self.b, &mut self.g),
            (b, a) if b == self.b && a == self.a => std::mem::swap(&mut self.b, &mut self.a),
            (a, r) if a == self.a && r == self.r => std::mem::swap(&mut self.a, &mut self.r),
            (a, g) if a == self.a && g == self.g => std::mem::swap(&mut self.a, &mut self.g),
            (a, b) if a == self.a && b == self.b => std::mem::swap(&mut self.a, &mut self.b),
            _ => {}
        }
        
        self
    }
}