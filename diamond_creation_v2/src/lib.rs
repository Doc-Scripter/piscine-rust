pub fn get_diamond(c: char) -> Vec<String> {
    // If input is 'A', return single character diamond
    if c == 'A' {
        return vec!["A".to_string()];
    }

    let size = (c as u8 - 'A' as u8) as usize;
    let width = 2 * size + 1;
    let mut result = Vec::new();

    // Create top half (including middle row)
    for i in 0..=size {
        let mut row = vec![' '; width];
        let current_char = (b'A' + i as u8) as char;
        
        // Place characters in the row
        row[size - i] = current_char;
        row[size + i] = current_char;
        
        result.push(row.iter().collect::<String>());
    }

    // Create bottom half (excluding middle row)
    for i in (0..size).rev() {
        let mut row = vec![' '; width];
        let current_char = (b'A' + i as u8) as char;
        
        // Place characters in the row
        row[size - i] = current_char;
        row[size + i] = current_char;
        
        result.push(row.iter().collect::<String>());
    }

    result
}
