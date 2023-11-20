
use sudoku::*;

fn main() {

  let cmdline_args: Vec<String> = std::env::args().collect();

  let animation_delay_ms = match cmdline_args.len() {
    2 => u64::from_str_radix(cmdline_args[1].as_str(), 10).unwrap(),
    _ => 50,
  };

  let mut board = SudokuBoard::generate(31);
  let mut steps = 0;
  board.solve_csp(animation_delay_ms, &mut steps);
  println!("Solved in {} steps", steps);
  
}