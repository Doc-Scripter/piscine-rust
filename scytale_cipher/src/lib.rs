pub fn scytale_cipher(message: String, i: u32) -> String {
    let chars: Vec<char> = message.chars().collect();
    let len = chars.len();
    let cols = i as usize;
    
    // Determine how many rows we need
    let rows = (len as f32 / cols as f32).ceil() as usize;

    // Create a grid with empty spaces if needed
    let mut grid = vec![vec![' '; cols]; rows];
    
    // Fill the grid row by row
    for (index, ch) in chars.iter().enumerate() {
        let row = index / cols;
        let col = index % cols;
        grid[row][col] = *ch;
    }

    // Read column by column to create the cipher
    let mut result = String::new();
    for col in 0..cols {
        for row in 0..rows {
            result.push(grid[row][col]);
        }
    }

    result.trim_end().to_string()
}
