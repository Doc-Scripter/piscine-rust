use temperature_conv_v2::*;

fn main() {
    println!("{} F = {} C", -459.67, fahrenheit_to_celsius(-459.67));
    println!("{} F = {} C",  20 , fahrenheit_to_celsius(20.0));
    println!("{} C = {} F", 0.0, celsius_to_fahrenheit(0.0));
}
/*-459.67 F = -273.15 C
 -6.666666666666666
0 C = 32 F*/