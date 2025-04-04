use circle_v2::*;

fn main() {
    let circle = Circle::new(500.0, 500.0, 150.0);
    let circle1 = Circle {
        center: Point(80.0, 115.0),
        radius: 30.0,
    };
    let point_a = Point(1.0, 1.0);
    let point_b = Point(0.0, 0.0);
    println!("circle = {:?} area = {}", circle, circle.area());
    println!("circle = {:?} diameter = {}", circle, circle.diameter());
    println!("circle1 = {:?} diameter = {}", circle1, circle1.diameter());
    println!(
        "circle and circle1 intersect = {}",
        circle.intersect(circle1)
    );

    println!(
        "distance between {:?} and {:?} is {}",
        point_a,
        point_b,
        point_a.distance(point_b)
    );
}
/*
circle = Circle { center: Point(500.0, 500.0), radius: 150.0 } area = 70685.83470577035
circle = Circle { center: Point(500.0, 500.0), radius: 150.0 } diameter = 300
circle1 = Circle { center: Point(80.0, 115.0), radius: 30.0 } diameter = 60
circle and circle1 intersect = false
distance between Point(1.0, 1.0) and Point(0.0, 0.0) is 1.4142135623730951
*/