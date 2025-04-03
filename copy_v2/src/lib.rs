pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c as f64).abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let exponential: String = a
        .split_whitespace()
        .map(|s| s.parse::<f64>().unwrap_or(0.0).exp().to_string())
        .collect::<Vec<String>>()
        .join(" ");
    (a, exponential)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let natural_absolute = b
        .iter()
        .map(|&c| (c as f64).abs().ln())
        .collect();
    (b, natural_absolute)
}
