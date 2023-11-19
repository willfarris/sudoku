use crate::tile::{Domain, Tile};

pub const SUDOKU_BASE: usize = 3;
pub const SUDOKU_SIZE: usize = SUDOKU_BASE * SUDOKU_BASE;

pub struct SudokuBoard {
    board: [[Tile; SUDOKU_SIZE]; SUDOKU_SIZE],
}

impl SudokuBoard {
    pub fn from_array(array: [usize; SUDOKU_SIZE * SUDOKU_SIZE]) -> Self {
        let mut board = [[Tile::default(); SUDOKU_SIZE]; SUDOKU_SIZE];
        for row in 0..SUDOKU_SIZE {
            for col in 0..SUDOKU_SIZE {
                let index = row * SUDOKU_SIZE + col;
                match array[index] {
                    0 => {}
                    c => {
                        board[row][col] = Tile::Collapsed(c);
                    },
                }
            }
        }

        let mut board = Self { board };
        board.propagate_uncollapsed();
        board
    }

    pub fn generate() -> Self {
        let board = [[Tile::default(); SUDOKU_SIZE]; SUDOKU_SIZE];
        // TODO: populate board during generation
        Self { board }
    }

    pub fn is_valid(&self) -> bool {
        // Check rows
        for row in 0..SUDOKU_SIZE {
            let mut present = [false; SUDOKU_SIZE];
            for col in 0..SUDOKU_SIZE {
                let tile = self.board[row][col];
                match tile {
                    Tile::Uncollapsed(_) => continue,
                    Tile::Collapsed(val) => {
                        if present[val - 1] {
                            return false;
                        }
                        present[val - 1] = true;
                    }
                }
            }
        }

        // Check cols
        for col in 0..SUDOKU_SIZE {
            let mut present = [false; SUDOKU_SIZE];
            for row in 0..SUDOKU_SIZE {
                let tile = self.board[row][col];
                match tile {
                    Tile::Uncollapsed(_) => continue,
                    Tile::Collapsed(val) => {
                        if present[val - 1] {
                            return false;
                        }
                        present[val - 1] = true;
                    }
                }
            }
        }

        // Check subgrids
        for sr in 0..SUDOKU_BASE {
            for sc in 0..SUDOKU_BASE {
                let mut present = [false; SUDOKU_SIZE];
                for r in 0..SUDOKU_BASE {
                    for c in 0..SUDOKU_BASE {
                        let row = sr * SUDOKU_BASE + r;
                        let col = sc * SUDOKU_BASE + c;
                        let tile = self.board[row][col];
                        match tile {
                            Tile::Uncollapsed(_) => continue,
                            Tile::Collapsed(val) => {
                                if present[val - 1] {
                                    return false;
                                }
                                present[val - 1] = true;
                            }
                        }
                    }
                }
            }
        }

        true
    }

    pub fn is_complete(&self) -> bool {
        for row in 0..SUDOKU_SIZE {
            for col in 0..SUDOKU_SIZE {
                if let Tile::Uncollapsed(_) = self.board[row][col] {
                    return false;
                }
            }
        }
        true
    }

    fn is_valid_assignment(&self, val: usize, row: usize, col: usize) -> bool {
        // Row
        for c in 0..SUDOKU_SIZE {
            if let Tile::Collapsed(tile) = self.board[row][c] {
                if tile == val {
                    return false;
                }
            }
        }

        // Col
        for r in 0..SUDOKU_SIZE {
            if let Tile::Collapsed(tile) = self.board[r][col] {
                if tile == val {
                    return false;
                }
            }
        }

        // 3x3 subgrid
        let sr = row / SUDOKU_BASE;
        let sc = col / SUDOKU_BASE;
        for r in 0..SUDOKU_BASE {
            for c in 0..SUDOKU_BASE {
                let cur_row = sr * SUDOKU_BASE + r;
                let cur_col = sc * SUDOKU_BASE + c;
                if let Tile::Collapsed(tile) = self.board[cur_row][cur_col] {
                    if tile == val {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn get_lowest_entropy(&self) -> (usize, usize) {
        let mut lowest_entropy = SUDOKU_SIZE+1;
        let mut lowest_index = (usize::MAX, usize::MAX);
        for row in 0..SUDOKU_SIZE {
            for col in 0..SUDOKU_SIZE {
                if let Tile::Uncollapsed(domain) = &self.board[row][col] {
                    let cur_entropy = domain.get_valid().len();
                    if cur_entropy <= lowest_entropy {
                        lowest_entropy = cur_entropy;
                        lowest_index = (row, col);
                    }
                }
            }
        }

        if let (usize::MAX, usize::MAX) = lowest_index {
            panic!("Board not complete but no uncollapsed tiles!");
        }
        lowest_index
    }

    fn propagate_uncollapsed(&mut self) {
        for row in 0..SUDOKU_SIZE {
            for col in 0..SUDOKU_SIZE  {
                if let Tile::Collapsed(_) = &self.board[row][col] {
                    self.propagate_collapse(row, col);
                }
            }
        }
    }

    fn propagate_collapse(&mut self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut modified = Vec::new();
        let val = match self.board[row][col] {
            Tile::Collapsed(val) => val,
            Tile::Uncollapsed(_) => panic!("Cannot propagate uncollapsed tile"),
        };

        // Update row
        for r in 0..SUDOKU_SIZE {
            if r == row {
                continue;
            }
            if let Tile::Uncollapsed(domain) = &mut self.board[r][col] {
                    if domain.mark_invalid(val) {
                        modified.push((r, col));
                    }
            }
        }

        // Update column
        for c in 0..SUDOKU_SIZE {
            if c == col {
                continue;
            }
            if let Tile::Uncollapsed(domain) = &mut self.board[row][c] {
                if domain.mark_invalid(val) {
                    modified.push((row, c));
                }
            }
        }
        
        // Update subgrid
        let subgrid_row = row / SUDOKU_BASE;
        let subgrid_col = col / SUDOKU_BASE;
        for r in 0..SUDOKU_BASE {
            for c in 0..SUDOKU_BASE {
                let s_row = subgrid_row*SUDOKU_BASE + r;
                let s_col = subgrid_col*SUDOKU_BASE + c;
                if s_row == row || s_col == col {
                    continue;
                }
                if let Tile::Uncollapsed(domain) = &mut self.board[s_row][s_col] {
                    if domain.mark_invalid(val) {
                        modified.push((s_row, s_col));
                    }
                }
            }
        }

        modified

    }

    fn restore_domain(&mut self, val: usize, collapsed: Vec<(usize, usize)>) {
        for (row, col) in collapsed {
            if let Tile::Uncollapsed(domain) = &mut self.board[row][col] {
                domain.mark_valid(val);
            } else {
                self.board[row][col] = Tile::Uncollapsed(Domain::from_value(val));
            }
        }

    }

    pub fn solve_csp(&mut self, debug_delay_ms: u64, steps: &mut u32) -> bool {
        if debug_delay_ms > 0 {
            std::thread::sleep(std::time::Duration::from_millis(debug_delay_ms));
            self.print(true);
        }
        *steps += 1;
        
        if self.is_complete() {
            return true;
        }
        
        let (row, col) = self.get_lowest_entropy();
        
        let valid = match self.board[row][col] {
            Tile::Collapsed(_) => panic!("get_lowest_entropy() should never return a collapsed tile"),
            Tile::Uncollapsed(domain) => domain.get_valid(),
        };

        for val in valid {
            if self.is_valid_assignment(val, row, col) {
                let saved_domain = if let Tile::Uncollapsed(domain) = &self.board[row][col] {
                    domain.clone()
                } else {
                    panic!("tile was uncollapsed, but now it's not: ({}, {})", row, col);
                };
                
                self.board[row][col] = Tile::Collapsed(val);
                let collapsed_states = self.propagate_collapse(row, col);
                if self.solve_csp(debug_delay_ms, steps) {
                    return true;
                }
                self.restore_domain(val, collapsed_states);
                self.board[row][col] = Tile::Uncollapsed(saved_domain);
            }
        }

        false
    }

    pub fn solve_brute_force(&mut self, debug_delay_ms: u64, steps: &mut u32) -> bool {
        if debug_delay_ms > 0 {
            std::thread::sleep(std::time::Duration::from_millis(debug_delay_ms));
            self.print(true);
        }
        *steps += 1;
        if self.is_complete() {
            return true;
        }
        for row in 0..SUDOKU_SIZE {
            for col in 0..SUDOKU_SIZE {
                if let Tile::Collapsed(_) = self.board[row][col] {
                    continue;
                }

                for s in 1..=SUDOKU_SIZE {
                    //self.board[row][col] = s;
                    if self.is_valid_assignment(s, row, col) {
                        //if self.is_valid() {
                        self.board[row][col] = Tile::Collapsed(s);
                        if self.solve_brute_force(debug_delay_ms, steps) {
                            return true;
                        }
                    }
                    self.board[row][col] = Tile::default();
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
                    Tile::Uncollapsed(_) => ' ',
                    Tile::Collapsed(v) => char::from_digit(*v as u32, 10).unwrap(),
                }
            }
        }

        println!("{}", line0);
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
