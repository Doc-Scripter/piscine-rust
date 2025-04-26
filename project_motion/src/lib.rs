#[derive(Debug, Clone)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

// Custom implementation of PartialEq for Object to handle floating-point precision
impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        // Use a small epsilon for floating-point comparison
        const EPSILON: f32 = 0.001;
        (self.x - other.x).abs() < EPSILON && (self.y - other.y).abs() < EPSILON
    }
}

#[derive(Debug, Clone)]
pub struct ThrowObject {
    pub init_position: Object,
    pub init_velocity: Object,
    pub actual_position: Object,
    pub actual_velocity: Object,
    pub time: f32,
}

// Custom implementation of PartialEq for ThrowObject to handle floating-point precision
impl PartialEq for ThrowObject {
    fn eq(&self, other: &Self) -> bool {
        self.init_position == other.init_position &&
        self.init_velocity == other.init_velocity &&
        self.actual_position == other.actual_position &&
        self.actual_velocity == other.actual_velocity &&
        (self.time - other.time).abs() < 0.001 // Small epsilon for time comparison
    }
}

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        // Initialize the ThrowObject with the given initial position and velocity
        // At the beginning, the actual position and velocity are the same as the initial ones
        // Time starts at 0
        ThrowObject {
            init_position: init_position.clone(),
            init_velocity: init_velocity.clone(),
            actual_position: init_position,
            actual_velocity: init_velocity,
            time: 0.0,
        }
    }
}

impl Iterator for ThrowObject {
    type Item = ThrowObject;

    fn next(&mut self) -> Option<Self::Item> {
        // Gravity constant (m/s²)
        const GRAVITY: f32 = 9.8;
        
        // Increment time by 1 second
        self.time += 1.0;
        
        // Calculate new position using the formula:
        // p(t) = p₀ + v₀t + (1/2)at²
        // For x-axis: no acceleration, so p_x(t) = p₀_x + v₀_x * t
        // For y-axis: acceleration due to gravity, so p_y(t) = p₀_y + v₀_y * t - (1/2)gt²
        let new_x = self.init_position.x + self.init_velocity.x * self.time;
        let new_y = self.init_position.y + self.init_velocity.y * self.time - 0.5 * GRAVITY * self.time * self.time;
        
        // Calculate new velocity using the formula:
        // v(t) = v₀ + at
        // For x-axis: no acceleration, so v_x(t) = v₀_x
        // For y-axis: acceleration due to gravity, so v_y(t) = v₀_y - gt
        let new_vx = self.init_velocity.x;
        let new_vy = self.init_velocity.y - GRAVITY * self.time;
        
        // Update actual position and velocity
        self.actual_position = Object { x: new_x, y: new_y };
        self.actual_velocity = Object { x: new_vx, y: new_vy };
        
        // If the object has reached or gone below the ground (y ≤ 0), return None
        // Otherwise, return the updated ThrowObject
        if self.actual_position.y <= 0.0 {
            None
        } else {
            Some(self.clone())
        }
    }
}