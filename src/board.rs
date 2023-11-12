pub const SUDOKU_BASE: usize = 3;
pub const SUDOKU_SIZE: usize = SUDOKU_BASE * SUDOKU_BASE;

pub struct SudokuBoard {
    board: [[usize; SUDOKU_SIZE]; SUDOKU_SIZE],
}

impl SudokuBoard {
    pub fn from_array(array: [usize; SUDOKU_SIZE * SUDOKU_SIZE]) -> Self {
        let mut board = [[0; SUDOKU_SIZE]; SUDOKU_SIZE];
        for row in 0..SUDOKU_SIZE {
            for col in 0..SUDOKU_SIZE {
                let index = row * SUDOKU_SIZE + col;
                board[row][col] = array[index];
            }
        }

        Self { board }
    }

    pub fn generate() -> Self {
        let mut board = [[0; SUDOKU_SIZE]; SUDOKU_SIZE];

        Self {
            board
        }
    }

    pub fn is_valid(&self) -> bool {
        // Check rows
        for row in 0..SUDOKU_SIZE {
            let mut present = [false; SUDOKU_SIZE];
            for col in 0..SUDOKU_SIZE {
                let val = self.board[row][col];
                if val == 0 {
                    continue;
                }
                if present[val - 1] {
                    return false;
                }
                present[val - 1] = true;
            }
        }

        // Check cols
        for col in 0..SUDOKU_SIZE {
            let mut present = [false; SUDOKU_SIZE];
            for row in 0..SUDOKU_SIZE {
                let val = self.board[row][col];
                if val == 0 {
                    continue;
                }
                if present[val - 1] {
                    return false;
                }
                present[val - 1] = true;
            }
        }

        // Check subgrids
        for sr in 0..SUDOKU_BASE {
            for sc in 0..SUDOKU_BASE {
                let mut present = [false; SUDOKU_SIZE];
                for r in 0..SUDOKU_BASE {
                    for c in 0..SUDOKU_BASE {
                        let val = self.board[sr * SUDOKU_BASE + r][sc * SUDOKU_BASE + c];
                        if val == 0 {
                            continue;
                        }
                        if present[val - 1] {
                            return false;
                        }
                        present[val - 1] = true;
                    }
                }
            }
        }

        true
    }

    pub fn is_complete(&self) -> bool {
        for row in 0..SUDOKU_SIZE {
            for col in 0..SUDOKU_SIZE {
                if self.board[row][col] == 0 {
                    return false;
                }
            }
        }
        true
    }

    fn is_valid_assignment(&self, val: usize, row: usize, col: usize) -> bool {
        // Row
        for c in 0..SUDOKU_SIZE {
            if self.board[row][c] == val {
                return false;
            }
        }

        // Col
        for r in 0..SUDOKU_SIZE {
            if self.board[r][col] == val {
                return false;
            }
        }

        // 3x3 subgrid
        let sr = row / SUDOKU_BASE;
        let sc = col / SUDOKU_BASE;
        for r in 0..SUDOKU_BASE {
            for c in 0..SUDOKU_BASE {
                if self.board[sr * SUDOKU_BASE + r][sc * SUDOKU_BASE + c] == val {
                    return false;
                }
            }
        }

        true
    }

    pub fn solve_brute_force(&mut self) -> bool {
        if self.is_complete() {
            return true;
        }
        for row in 0..SUDOKU_SIZE {
            for col in 0..SUDOKU_SIZE {
                if self.board[row][col] != 0 {
                    continue;
                }

                #[cfg(debug_prints)]
                {
                    std::thread::sleep(Duration::from_millis(DEBUG_DELAY_MS));
                    self.print(true);
                }

                for s in 1..=SUDOKU_SIZE {
                    //self.board[row][col] = s;
                    if self.is_valid_assignment(s, row, col) {
                        //if self.is_valid() {
                        self.board[row][col] = s;
                        if self.solve_brute_force() {
                            return true;
                        }
                    }
                    self.board[row][col] = 0;
                }
                return false;
            }
        }

        false
    }

    // Adapted from https://stackoverflow.com/questions/45471152/how-to-create-a-sudoku-puzzle-in-python
    pub fn print(&self, clear: bool) {
        if clear {
            print!("\x1B[2J\x1B[1;1H");
        }
        let board = &self.board;

        fn expand_line(line: &str) -> String {
            let chars: Vec<char> = line.chars().collect();
            let output_vec: String = chars[1..5].repeat(SUDOKU_BASE - 1).iter().collect();
            let sep: String = chars[5..9].iter().collect();
            let out = vec![output_vec.as_str()]
                .repeat(SUDOKU_BASE)
                .join(sep.as_str());
            [chars[0].to_string(), out, chars[9..13].iter().collect()].join("")
        }

        let line0 = expand_line("╔═══╤═══╦═══╗");
        let line1 = expand_line("║ . │ . ║ . ║");
        let line2 = expand_line("╟───┼───╫───╢");
        let line3 = expand_line("╠═══╪═══╬═══╣");
        let line4 = expand_line("╚═══╧═══╩═══╝");

        let mut nums: [[char; SUDOKU_SIZE]; SUDOKU_SIZE] = [[' '; SUDOKU_SIZE]; SUDOKU_SIZE];
        for (row_num, row) in board.iter().enumerate() {
            for (col_num, tile) in row.iter().enumerate() {
                nums[row_num][col_num] = match tile {
                    0 => ' ',
                    v => char::from_digit(*v as u32, 10).unwrap(),
                }
            }
        }

        println!("\n{}", line0);
        for r in 0..SUDOKU_SIZE {
            for (c, v) in nums[r].iter().zip(line1.split(".")) {
                print!("{}{}", v, c);
            }

            println!(" {}", line1.chars().last().unwrap());
            println!(
                "{}",
                [&line2, &line3, &line4]
                    [((r + 1) % SUDOKU_BASE == 0) as usize + (r == SUDOKU_SIZE - 1) as usize]
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::board::{SudokuBoard, SUDOKU_SIZE};

    const ARRAY_SIZE: usize = SUDOKU_SIZE * SUDOKU_SIZE;

    pub const _TEST_SPARSE_ARRAY: [usize; ARRAY_SIZE] = [
        0, 0, 0, 0, 0, 0, 0, 9, 0, 9, 1, 0, 0, 0, 0, 0, 5, 0, 0, 0, 8, 0, 0, 2, 0, 0, 3, 0, 0, 5,
        0, 0, 9, 3, 0, 0, 0, 0, 0, 1, 3, 4, 0, 0, 0, 0, 0, 0, 0, 8, 5, 0, 0, 0, 0, 0, 0, 2, 0, 0,
        0, 0, 0, 3, 0, 0, 8, 0, 0, 0, 0, 0, 2, 4, 0, 0, 0, 0, 0, 8, 0,
    ];
    pub const _TEST_SOLUTION_ARRAY: [usize; ARRAY_SIZE] = [
        4, 7, 3, 5, 6, 8, 1, 9, 2, 9, 1, 2, 4, 7, 3, 6, 5, 8, 5, 6, 8, 9, 1, 2, 7, 4, 3, 7, 8, 5,
        6, 2, 9, 3, 1, 4, 6, 2, 9, 1, 3, 4, 8, 7, 5, 1, 3, 4, 7, 8, 5, 2, 6, 9, 8, 9, 6, 2, 4, 1,
        5, 3, 7, 3, 5, 7, 8, 9, 6, 4, 2, 1, 2, 4, 1, 3, 5, 7, 9, 8, 6,
    ];
    pub const _TEST_SINGLE_SOLN_ARRAY: [usize; ARRAY_SIZE] = [
        0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 4, 5, 6, 0, 1, 0, 7, 0, 8, 0, 0, 1, 0, 0, 0, 9, 0, 3, 7,
        0, 0, 0, 0, 9, 0, 0, 9, 0, 0, 4, 0, 0, 1, 0, 0, 5, 0, 0, 0, 0, 8, 2, 0, 5, 0, 0, 0, 7, 0,
        0, 6, 0, 2, 0, 1, 0, 9, 3, 5, 0, 8, 0, 0, 0, 1, 0, 0, 0, 4, 0,
    ];

    pub const _TEST_DUPLICATE_9S_INVALID: [usize; ARRAY_SIZE] = [
        4, 2, 3, 5, 6, 7, 1, 9, 0, 9, 1, 6, 4, 0, 0, 2, 5, 7, 5, 7, 8, 9, 0, 2, 0, 0, 3, 0, 0, 5,
        0, 2, 9, 3, 1, 0, 0, 0, 2, 1, 3, 4, 7, 0, 0, 0, 3, 4, 0, 8, 5, 0, 0, 0, 1, 6, 9, 2, 4, 3,
        0, 7, 0, 3, 9, 0, 8, 0, 6, 0, 4, 0, 2, 4, 0, 0, 0, 0, 6, 8, 0,
    ];

    #[test]
    fn test_load_sparse_array() {
        let board = SudokuBoard::from_array(_TEST_SPARSE_ARRAY);
        assert!(board.is_valid());
        assert!(!board.is_complete());
    }

    #[test]
    fn test_invalid_row() {
        let mut test_array = [0; ARRAY_SIZE];
        test_array[0] = 9;
        test_array[1] = 9;
        let board = SudokuBoard::from_array(test_array);
        assert!(!board.is_valid());
        assert!(!board.is_complete());
    }

    #[test]
    fn test_invalid_col() {
        let mut test_array = [0; ARRAY_SIZE];
        test_array[0] = 9;
        test_array[9] = 9;
        let board = SudokuBoard::from_array(test_array);
        assert!(!board.is_valid());
        assert!(!board.is_complete());
    }

    #[test]
    fn test_invalid_subgrid() {
        let mut test_array = [0; ARRAY_SIZE];
        test_array[1] = 5;
        test_array[9] = 5;
        let board = SudokuBoard::from_array(test_array);
        assert!(!board.is_valid());
        assert!(!board.is_complete());
    }

    #[test]
    fn test_invalid_subgrid_9s() {
        let board = SudokuBoard::from_array(_TEST_DUPLICATE_9S_INVALID);
        assert!(!board.is_valid());
        assert!(!board.is_complete());
    }

    #[test]
    fn test_load_full_array() {
        let board = SudokuBoard::from_array(_TEST_SOLUTION_ARRAY);
        assert!(board.is_valid());
        assert!(board.is_complete());
    }
}
