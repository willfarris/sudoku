use sudoku::*;

const TEST_BOARD: [usize; SUDOKU_SIZE*SUDOKU_SIZE] = [
        0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 4, 5, 6, 0, 1, 0, 7, 0, 8, 0, 0, 1, 0, 0, 0, 9, 0, 3, 7,
        0, 0, 0, 0, 9, 0, 0, 9, 0, 0, 4, 0, 0, 1, 0, 0, 5, 0, 0, 0, 0, 8, 2, 0, 5, 0, 0, 0, 7, 0,
        0, 6, 0, 2, 0, 1, 0, 9, 3, 5, 0, 8, 0, 0, 0, 1, 0, 0, 0, 4, 0,
    ]; 

fn main() {
    let mut board = SudokuBoard::from_array(TEST_BOARD);

    println!("Before:");
    board.print(false);

    board.solve_brute_force();

    println!("After:");
    board.print(false);
}
