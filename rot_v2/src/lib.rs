pub fn rotate(input: &str, key: i8) -> String {
    let mut result = String::with_capacity(input.len());
    
    // Normalize the key to be within 0-25 range
    let normalized_key = ((key % 26) + 26) % 26;
    
    for c in input.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_uppercase() { 'A' } else { 'a' } as u8;
            let offset = (c as u8) - base;
            let rotated = (offset as i16 + normalized_key as i16) % 26;
            let new_char = (base + rotated as u8) as char;
            result.push(new_char);
        } else {
            // Non-alphabetic characters remain unchanged
            result.push(c);
        }
    }
    
    result
}