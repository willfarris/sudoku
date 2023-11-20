use sudoku::*;

fn main() {
    let mut board = SudokuBoard::generate();
    board.print(false);
}