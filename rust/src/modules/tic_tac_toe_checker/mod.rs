pub fn _is_solved(board: &[&[u8; 3]; 3]) -> i8 {
    // Check rows
    for row in board {
        if row[0] == row[1] && row[1] == row[2] && row[0] != 0 {
            return row[0] as i8;
        }
    }

    // Check columns
    for i in 0..3 {
        if board[0][i] == board[1][i] && board[1][i] == board[2][i] && board[0][i] != 0 {
            return board[0][i] as i8;
        }
    }

    // Check diagonals
    if board[0][0] == board[1][1] && board[1][1] == board[2][2] && board[0][0] != 0 {
        return board[0][0] as i8;
    }

    if board[0][2] == board[1][1] && board[1][1] == board[2][0] && board[0][2] != 0 {
        return board[0][2] as i8;
    }

    // Check if the game is unfinished
    for row in board {
        if row.contains(&0u8) {
            return -1;
        }
    }

    // If no winner and the game is finished, return 0
    0
}

pub fn is_solved(board: &[&[u8; 3]; 3]) -> i8 {
    // Check rows
    for row in board {
        if row[0] == row[1] && row[1] == row[2] && row[0] != 0 {
            return row[0] as i8;
        }
    }

    // Check columns
    for i in 0..3 {
        if board[0][i] == board[1][i] && board[1][i] == board[2][i] && board[0][i] != 0 {
            return board[0][i] as i8;
        }
    }

    // Check diagonals
    if board[0][0] == board[1][1] && board[1][1] == board[2][2] && board[0][0] != 0 {
        return board[0][0] as i8;
    }

    if board[0][2] == board[1][1] && board[1][1] == board[2][0] && board[0][2] != 0 {
        return board[0][2] as i8;
    }

    board.iter().fold(0, |res, row| match row.contains(&0u8) {
        true => -1,
        false => res,
    })
}

#[cfg(test)]
mod tests {
    use super::is_solved;

    fn dotest(board: &[&[u8; 3]; 3], expected: i8) {
        let actual = is_solved(board);
        assert!(
            actual == expected,
            "With board = {board:?}\nExpected {expected} but got {actual}"
        )
    }

    #[test]
    fn fixed_tests() {
        for (board, expected) in [
            ([&[0, 0, 1], &[0, 1, 2], &[2, 1, 0]], -1),
            ([&[1, 1, 1], &[0, 2, 2], &[0, 0, 0]], 1),
            ([&[2, 1, 2], &[2, 1, 1], &[1, 1, 2]], 1),
            ([&[2, 1, 2], &[2, 1, 1], &[1, 2, 1]], 0),
        ] {
            dotest(&board, expected);
        }
    }
}
