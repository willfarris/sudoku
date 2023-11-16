use sudoku::*;

fn main() {
    let mut board = SudokuBoard::from_array([0; SUDOKU_SIZE * SUDOKU_SIZE]);
    board.solve_brute_force();
    board.print(true);
}
