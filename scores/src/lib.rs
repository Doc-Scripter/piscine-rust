pub fn score(s: &str) -> f64 {
    let mut sum: f64 = 0.0;
    for i in s.chars() {
        match i.to_ascii_uppercase() {
            'A' | 'E' | 'I' | 'O' | 'U' | 'N' | 'L' | 'R' | 'S' | 'T' => sum += 1.0,
            'D' | 'G' => sum += 2.0,
            'B' | 'C' | 'M' | 'P' => sum += 3.0,
            'F' | 'H' | 'V' | 'W' | 'Y' => sum += 4.0,
            'K' => sum += 5.0,
            'J' | 'X' => sum += 8.0,
            'Q' | 'Z' => sum += 10.0,
            _ => sum += 0.0,
        }
    }
    sum
}
