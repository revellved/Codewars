// Tic-Tac-Toe Checker
// https://www.codewars.com/kata/525caa5c1bf619d28c000335
//
// Description:
//
// If we were to set up a Tic-Tac-Toe game, we would want to know whether the board's current state is solved, wouldn't we? Our goal is to create a function that will check that for us!
//
// Assume that the board comes in the form of a 3x3 array, where the value is 0 if a spot is empty, 1 if it is an "X", or 2 if it is an "O", like so:
//
// [[0, 0, 1],
//  [0, 1, 2],
//  [2, 1, 0]]
//
// We want our function to return:
//
//     -1 if the board is not yet finished AND no one has won yet (there are empty spots),
//     1 if "X" won,
//     2 if "O" won,
//     0 if it's a cat's game (i.e. a draw).
//
// You may assume that the board passed in is valid in the context of a game of Tic-Tac-Toe.
//

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
