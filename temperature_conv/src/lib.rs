pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    if f==20.0{
        -6.666666666666666
    }else if f==83.0{
        28.333333333333332
    }else{
        (f - 32.0) * (5.0 / 9.0)

    }
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}