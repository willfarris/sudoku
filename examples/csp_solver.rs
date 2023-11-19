use sudoku::*;

const TEST_BOARD: [usize; SUDOKU_SIZE * SUDOKU_SIZE] = [
    0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 4, 5, 6, 0, 1, 0, 7, 0, 8, 0, 0, 1, 0, 0, 0, 9, 0, 3, 7, 0, 0,
    0, 0, 9, 0, 0, 9, 0, 0, 4, 0, 0, 1, 0, 0, 5, 0, 0, 0, 0, 8, 2, 0, 5, 0, 0, 0, 7, 0, 0, 6, 0, 2,
    0, 1, 0, 9, 3, 5, 0, 8, 0, 0, 0, 1, 0, 0, 0, 4, 0,
];

fn main() {

    let delay_ms = 50;

    let mut csp_board = SudokuBoard::from_array(TEST_BOARD);
    let mut csp_steps = 0;
    csp_board.solve_csp(delay_ms, &mut csp_steps);

    let mut dfs_board = SudokuBoard::from_array(TEST_BOARD);
    let mut dfs_steps = 0;
    dfs_board.solve_brute_force(delay_ms, &mut dfs_steps);
    println!("CSP solved in {} steps\nDFS solved in {} steps", csp_steps, dfs_steps);
}