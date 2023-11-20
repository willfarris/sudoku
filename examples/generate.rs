use sudoku::*;

fn main() {
    let board = SudokuBoard::generate(21);
    board.print(false);
}