use crate::board::SUDOKU_SIZE;

#[derive(Copy, Clone)]
pub struct Domain {
    state: [bool; SUDOKU_SIZE],
}

impl Default for Domain {
    fn default() -> Self {
        Self {
            state: [true; SUDOKU_SIZE],
        }
    }
}

impl Domain {
    pub fn from_value(val: usize) -> Self {
        let mut domain = Self::default();
        domain.state[val-1] = true;
        domain
    }

    pub fn get_valid(&self) -> Vec<usize> {
        self.state
            .iter()
            .enumerate()
            .filter_map(|(index, is_valid)| match *is_valid {
                true => Some(index + 1),
                false => None,
            })
            .collect()
    }

    // Returns true on modify, false if val is already invalid
    pub fn mark_invalid(&mut self, val: usize) -> bool {
        if !self.state[val-1] {
            return false;
        }
        self.state[val-1] = false;
        true
    }

    pub fn mark_valid(&mut self, val: usize) {
        self.state[val-1] = true;
    }
}

#[derive(Copy, Clone)]
pub enum Tile {
    Collapsed(usize),
    Uncollapsed(Domain),
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Uncollapsed(Domain::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::{board::SUDOKU_SIZE, tile::Domain};

    #[test]
    fn test_state_none_valid() {
        let state = Domain {
            state: [false; SUDOKU_SIZE],
        };
        let valid_in_state = state.get_valid();
        assert_eq!(valid_in_state.len(), 0);
    }

    #[test]
    fn test_state_all_valid() {
        let state = Domain {
            state: [true; SUDOKU_SIZE],
        };
        let valid_in_state = state.get_valid();
        assert_eq!(valid_in_state.len(), 9);
    }

    #[test]
    fn test_state_valid_count() {
        let mut state = Domain {
            state: [true; SUDOKU_SIZE],
        };
        for i in 1..=SUDOKU_SIZE {
            state.mark_invalid(i);
            let valid_in_state = state.get_valid();
            assert_eq!(valid_in_state.contains(&i), false)
        }

        let valid_in_state = state.get_valid();
        assert_eq!(valid_in_state.len(), 0);
    }
}
