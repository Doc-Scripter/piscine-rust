pub fn score(s: &str) -> i32 {
    let mut sum= 0;
    for i in s.chars() {
        match i.to_ascii_uppercase() {
            'A' | 'E' | 'I' | 'O' | 'U' | 'N' | 'L' | 'R' | 'S' | 'T' => sum += 1,
            'D' | 'G' => sum += 2,
            'B' | 'C' | 'M' | 'P' => sum += 3,
            'F' | 'H' | 'V' | 'W' | 'Y' => sum += 4,
            'K' => sum += 5,
            'J' | 'X' => sum += 8,
            'Q' | 'Z' => sum += 10,
            _ => sum += 0,
        }
    }
    sum
}
