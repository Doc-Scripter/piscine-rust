pub fn stars(n: u32) -> String {
    let  num =2_f64.powf(n as f64) as usize;
    "*".repeat(num as usize)
}