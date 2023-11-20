use sudoku::*;

fn main() {
  let mut board = SudokuBoard::generate();
  board.solve_csp(150, &mut 0);
  
}