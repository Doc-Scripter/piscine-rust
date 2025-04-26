use project_motion::*;

fn main() {
    let mut obj = ThrowObject::new(Object { x: 50.0, y: 50.0 }, Object { x: 0.0, y: 0.0 });
    println!("{:?}", obj.next());
    println!("{:?}", obj.next());
    println!("{:?}", obj.next());
    println!("{:?}", obj.next());
    println!("{:?}", obj.next());
}
/*

Some(ThrowObject { init_position: Object { x: 50.0, y: 50.0 }, init_velocity: Object { x: 0.0, y: 0.0 }, actual_position: Object { x: 50.0, y: 45.1 }, actual_velocity: Object { x: 0.0, y: -9.8 }, time: 1.0 })
Some(ThrowObject { init_position: Object { x: 50.0, y: 50.0 }, init_velocity: Object { x: 0.0, y: 0.0 }, actual_position: Object { x: 50.0, y: 30.4 }, actual_velocity: Object { x: 0.0, y: -19.6 }, time: 2.0 })
Some(ThrowObject { init_position: Object { x: 50.0, y: 50.0 }, init_velocity: Object { x: 0.0, y: 0.0 }, actual_position: Object { x: 50.0, y: 5.9 }, actual_velocity: Object { x: 0.0, y: -29.4 }, time: 3.0 })
None
None

*/