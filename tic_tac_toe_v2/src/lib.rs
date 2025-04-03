pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    let players = ['X', 'O'];
    
    // Check if X wins
    if diagonals(players[0], table) || horizontal(players[0], table) || vertical(players[0], table) {
        return "player X won".to_string();
    }
    
    // Check if O wins
    if diagonals(players[1], table) || horizontal(players[1], table) || vertical(players[1], table) {
        return "player O won".to_string();
    }
    
    // Check if the game is still in progress (has empty cells)
    for row in &table {
        for &cell in row {
            if cell != 'X' && cell != 'O' {
                return "in progress".to_string();
            }
        }
    }
    
    // If no winner and no empty cells, it's a tie
    "Tie".to_string()
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    // Check main diagonal (top-left to bottom-right)
    let main_diagonal_win = (0..3).all(|i| table[i][i] == player);
    
    // Check other diagonal (top-right to bottom-left)
    let other_diagonal_win = (0..3).all(|i| table[i][2-i] == player);
    
    // Return true if either diagonal has a win
    main_diagonal_win || other_diagonal_win
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    let mut i = 0;
    let mut j = i;
    while i < table.len() {
        let mut count_x = 0;
        let mut count_o = 0;
        while j < table[i].len() {
            if table[i][j] == 'X' {
                count_x += 1;
            }
            if table[i][j] == 'O' {
                count_o += 1;
            }
            j += 1;
        }
        // println!("O: {},X: {}", count_o, count_x);
        if count_o == 3 && player == 'O' {
            return true;
        }
        if count_x == 3 && player == 'X' {
            return true;
        }

        if j == table[i].len() - 1 {
            j = 0;
        }
        i += 1;
    }
    false
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    let mut i = 0;
    let mut j = i;
    while j < table[i].len() {
        let mut count_x = 0;
        let mut count_o = 0;
        while i < table.len() {
            if table[i][j] == 'X' {
                count_x += 1;
            }
            if table[i][j] == 'O' {
                count_o += 1;
            }
            // println!("O: {},X: {}", count_o, count_x);
            if count_o == 3 && player == 'O' {
                return true;
            }
            if count_x == 3 && player == 'X' {
                return true;
            }
            i += 1;
        }
        i = 0;
        j += 1;
        if j == table[i].len() - 1 {
            break;
        }
    }
    false
}
