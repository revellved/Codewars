// Solve the given puzzle in place, no need to return a copy
use std::{collections::HashSet, iter::FromIterator};

pub fn sudoku(puzzle: &mut [[u8; 9]; 9]) {
    let s: HashSet<u8> = HashSet::from_iter(1..=9);
    for r in 0..9 {
        for c in 0..9 {
            if puzzle[r][c] == 0 {
                let br = r / 3 * 3;
                let bc = c / 3 * 3;
                let block: HashSet<u8> = HashSet::from_iter(
                    (0..3_usize)
                        .flat_map(|row| (0..3_usize).map(move |col| (row, col)))
                        .map(|(r, c)| puzzle[br + r][bc + c]),
                );
                let row = HashSet::from(puzzle[r]);
                let col = HashSet::from_iter(puzzle.iter().map(|row| row[c]));
                let x = &s - &(&(&row | &col) | &block);
                if x.len() == 1 {
                    puzzle[r][c] = *x.iter().next().unwrap() as u8;
                    sudoku(puzzle)
                }
            }
        }
    }
}

#[cfg(test)]
mod sample_tests {
    use super::sudoku;

    #[test]
    fn puzzle_1() {
        let mut puzzle = [
            [6, 0, 5, 7, 2, 0, 0, 3, 9],
            [4, 0, 0, 0, 0, 5, 1, 0, 0],
            [0, 2, 0, 1, 0, 0, 0, 0, 4],
            [0, 9, 0, 0, 3, 0, 7, 0, 6],
            [1, 0, 0, 8, 0, 9, 0, 0, 5],
            [2, 0, 4, 0, 5, 0, 0, 8, 0],
            [8, 0, 0, 0, 0, 3, 0, 2, 0],
            [0, 0, 2, 9, 0, 0, 0, 0, 1],
            [3, 5, 0, 0, 6, 7, 4, 0, 8],
        ];
        let solution = [
            [6, 1, 5, 7, 2, 4, 8, 3, 9],
            [4, 8, 7, 3, 9, 5, 1, 6, 2],
            [9, 2, 3, 1, 8, 6, 5, 7, 4],
            [5, 9, 8, 4, 3, 2, 7, 1, 6],
            [1, 3, 6, 8, 7, 9, 2, 4, 5],
            [2, 7, 4, 6, 5, 1, 9, 8, 3],
            [8, 4, 9, 5, 1, 3, 6, 2, 7],
            [7, 6, 2, 9, 4, 8, 3, 5, 1],
            [3, 5, 1, 2, 6, 7, 4, 9, 8],
        ];

        sudoku(&mut puzzle);
        assert_eq!(
            puzzle, solution,
            "\nYour solution (left) did not match the correct solution (right)"
        );
    }

    #[test]
    fn puzzle_2() {
        let mut puzzle = [
            [0, 0, 8, 0, 3, 0, 5, 4, 0],
            [3, 0, 0, 4, 0, 7, 9, 0, 0],
            [4, 1, 0, 0, 0, 8, 0, 0, 2],
            [0, 4, 3, 5, 0, 2, 0, 6, 0],
            [5, 0, 0, 0, 0, 0, 0, 0, 8],
            [0, 6, 0, 3, 0, 9, 4, 1, 0],
            [1, 0, 0, 8, 0, 0, 0, 2, 7],
            [0, 0, 5, 6, 0, 3, 0, 0, 4],
            [0, 2, 9, 0, 7, 0, 8, 0, 0],
        ];
        let solution = [
            [9, 7, 8, 2, 3, 1, 5, 4, 6],
            [3, 5, 2, 4, 6, 7, 9, 8, 1],
            [4, 1, 6, 9, 5, 8, 3, 7, 2],
            [8, 4, 3, 5, 1, 2, 7, 6, 9],
            [5, 9, 1, 7, 4, 6, 2, 3, 8],
            [2, 6, 7, 3, 8, 9, 4, 1, 5],
            [1, 3, 4, 8, 9, 5, 6, 2, 7],
            [7, 8, 5, 6, 2, 3, 1, 9, 4],
            [6, 2, 9, 1, 7, 4, 8, 5, 3],
        ];

        sudoku(&mut puzzle);
        assert_eq!(
            puzzle, solution,
            "\nYour solution (left) did not match the correct solution (right)"
        );
    }
}
