use sudoku::*;

#[test]
fn test_generation() {
    let mut board = SudokuBoard::generate(21);
    assert!(board.is_valid());
    assert!(!board.is_complete());

    let mut steps = 0;
    board.solve_brute_force(0, &mut steps);
    assert!(board.is_valid());
    assert!(board.is_complete());
}

#[test]
fn test_brute_force_solver() {
    let mut board = SudokuBoard::generate(21);
    assert!(board.is_valid());
    assert!(!board.is_complete());

    let mut steps = 0;
    board.solve_brute_force(0, &mut steps);
    assert!(board.is_complete());
}


#[test]
fn test_csp_solver() {
    let mut board = SudokuBoard::generate(21);
    assert!(board.is_valid());
    assert!(!board.is_complete());

    let mut steps = 0;
    board.solve_csp(0, &mut steps);
    assert!(board.is_complete());
}

#[test]
fn test_csp_is_better() {
    let mut dfs_board = SudokuBoard::generate(21);
    assert!(dfs_board.is_valid());
    assert!(!dfs_board.is_complete());

    let mut csp_board = dfs_board.clone();

    let mut dfs_steps = 0;
    dfs_board.solve_brute_force(0, &mut dfs_steps);
    assert!(dfs_board.is_complete());

    let mut csp_steps = 0;
    csp_board.solve_csp(0, &mut csp_steps);
    assert!(csp_board.is_complete());

    assert!(csp_steps <= dfs_steps);
}
