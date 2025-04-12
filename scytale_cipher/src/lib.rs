
pub fn scytale_cipher(message: String, size: u32) -> String {
    let message_len = message.len();
    let size = size as usize;
    
    // If size is 0 or message is empty, return the original message
    if size == 0 || message_len == 0 {
        return message;
    }
    
    // Calculate number of rows needed (ceiling division)
    let rows = (message_len + size - 1) / size;
    
    // Create a matrix to represent the cylinder
    let mut matrix = vec![vec![' '; size]; rows];
    
    // Fill the matrix with the message characters (row by row)
    let chars: Vec<char> = message.chars().collect();
    for (i, &c) in chars.iter().enumerate() {
        let row = i / size;
        let col = i % size;
        matrix[row][col] = c;
    }
    
    // Read the matrix column by column to get the encrypted message
    let mut result = String::with_capacity(message_len);
    for col in 0..size {
        for row in 0..rows {
            // Only add characters that are part of the original message
            let index = row * size + col;
            if index < message_len {
                result.push(matrix[row][col]);
            }
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scytale_cipher() {
        assert_eq!(
            scytale_cipher(String::from("scytale Code"), 6),
            "sec yCtoadle"
        );
        assert_eq!(
            scytale_cipher(String::from("scytale Code"), 8),
            "sCcoydtea l e"
        );
    }
    
    #[test]
    fn test_edge_cases() {
        // Empty string
        assert_eq!(scytale_cipher(String::from(""), 5), "");
        
        // Size 0
        assert_eq!(scytale_cipher(String::from("test"), 0), "test");
        
        // Size 1 (no encryption happens)
        assert_eq!(scytale_cipher(String::from("test"), 1), "test");
        
        // Size equals message length
        assert_eq!(scytale_cipher(String::from("test"), 4), "test");
    }
}


