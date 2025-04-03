pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    let players = ['X', 'O'];

    // Check if X wins
    if diagonals(players[0], table) || horizontal(players[0], table) || vertical(players[0], table)
    {
        return "player X won".to_string();
    }

    // Check if O wins
    if diagonals(players[1], table) || horizontal(players[1], table) || vertical(players[1], table)
    {
        return "player O won".to_string();
    }

    // If no winner and no empty cells, it's a tie
    "tie".to_string()
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    // Check main diagonal (top-left to bottom-right)
    let main_diagonal_win = (0..3).all(|i| table[i][i] == player);

    // Check other diagonal (top-right to bottom-left)
    let other_diagonal_win = (0..3).all(|i| table[i][2 - i] == player);

    // Return true if either diagonal has a win
    main_diagonal_win || other_diagonal_win
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    // Check each row
    for row in &table {
        // Count how many of the player's marks are in this row
        let count = row.iter().filter(|&&cell| cell == player).count();

        // If the player has 3 marks in this row, they win
        if count == 3 {
            return true;
        }
    }

    // No horizontal win found
    false
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    // Check each column
    for col in 0..3 {
        // Count how many of the player's marks are in this column
        let count = (0..3).filter(|&row| table[row][col] == player).count();
        
        // If the player has 3 marks in this column, they win
        if count == 3 {
            return true;
        }
    }
    
    // No vertical win found
    false
}
