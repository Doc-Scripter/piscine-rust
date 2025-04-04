
#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Circle {
            center: Point(x, y),
            radius: radius,
        }
    }
    //-> returns the diameter of the circle.
    pub fn diameter(self) -> f64 {
        2.0 * self.radius
    }

    //-> returns the area of the circle.
   pub  fn area(self)->f64{
    std::f64::consts::PI*self.radius.powf(2.0)
    }
    //-> returns if two circles intersect.
    pub fn intersect(self, other: Circle) -> bool {
        // Calculate the distance between the centers of the two circles
        let distance = self.center.distance(other.center);
        
        // Two circles intersect if the distance between their centers
        // is less than the sum of their radii but greater than the
        // absolute difference of their radii
        let sum_of_radii = self.radius + other.radius;
        let diff_of_radii = (self.radius - other.radius).abs();
        
        // Check intersection condition
        distance < sum_of_radii && distance > diff_of_radii
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(self, other: Point) -> f64 {
        (self.0 - other.0).hypot(self.1 - other.1)
    }
}
