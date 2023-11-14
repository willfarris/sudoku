use crate::board::SUDOKU_SIZE;

#[derive(Copy, Clone)]
struct Domain {
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
    fn get_valid(&self) -> Vec<usize> {
        self.state
            .iter()
            .enumerate()
            .filter_map(|(index, is_valid)| match *is_valid {
                true => Some(index + 1),
                false => None,
            })
            .collect()
    }

    fn mark_invalid(&mut self, v: usize) {
        assert!(v < SUDOKU_SIZE + 1);
        assert!(v > 0);
        self.state[v - 1] = false;
    }
}

#[derive(Copy, Clone)]
enum Tile {
    Collapsed(usize),
    Uncollapsed(Domain),
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
