use rgb_match_v2::*;

fn main() {
    let c = Color {
        r: 255,
        g: 200,
        b: 10,
        a: 30,
    };

    println!("{:?}", c.swap(c.r, c.b));
    println!("{:?}", c.swap(c.r, c.g));
    println!("{:?}", c.swap(c.r, c.a));
    println!();
    println!("{:?}", c.swap(c.g, c.r));
    println!("{:?}", c.swap(c.g, c.b));
    println!("{:?}", c.swap(c.g, c.a));
    println!();
    println!("{:?}", c.swap(c.b, c.r));
    println!("{:?}", c.swap(c.b, c.g));
    println!("{:?}", c.swap(c.b, c.a));
    println!();
    println!("{:?}", c.swap(c.a, c.r));
    println!("{:?}", c.swap(c.a, c.b));
    println!("{:?}", c.swap(c.a, c.g));
}
/*
Color { r: 10, g: 200, b: 255, a: 30 }
Color { r: 200, g: 255, b: 10, a: 30 }
Color { r: 30, g: 200, b: 10, a: 255 }

Color { r: 200, g: 255, b: 10, a: 30 }
Color { r: 255, g: 10, b: 200, a: 30 }
Color { r: 255, g: 30, b: 10, a: 200 }

Color { r: 10, g: 200, b: 255, a: 30 }
Color { r: 255, g: 10, b: 200, a: 30 }
Color { r: 255, g: 200, b: 30, a: 10 }

Color { r: 30, g: 200, b: 10, a: 255 }
Color { r: 255, g: 200, b: 30, a: 10 }
Color { r: 255, g: 30, b: 10, a: 200 }
*/