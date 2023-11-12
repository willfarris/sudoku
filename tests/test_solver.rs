use sudoku::*;

#[test]
fn test_generation() {
    let board = SudokuBoard::generate();
    assert!(board.is_valid());
    assert!(!board.is_complete());
}

#[test]
fn test_solver() {
    let mut board = SudokuBoard::generate();
    assert!(board.is_valid());
    assert!(!board.is_complete());

    board.solve_brute_force();
    assert!(board.is_complete());
}