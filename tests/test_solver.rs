use sudoku::*;

#[test]
fn test_generation() {
    let mut board = SudokuBoard::generate();
    assert!(board.is_valid());
    assert!(!board.is_complete());

    let mut steps = 0;
    board.solve_brute_force(&mut steps);
    assert!(board.is_valid());
    assert!(board.is_complete());
}

#[test]
fn test_brute_force_solver() {
    let mut board = SudokuBoard::generate();
    assert!(board.is_valid());
    assert!(!board.is_complete());

    let mut steps = 0;
    board.solve_brute_force(&mut steps);
    assert!(board.is_complete());
}


#[test]
fn test_csp_solver() {
    let mut board = SudokuBoard::generate();
    assert!(board.is_valid());
    assert!(!board.is_complete());

    let mut steps = 0;
    board.solve_csp(&mut steps);
    assert!(board.is_complete());
}
