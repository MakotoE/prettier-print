use crate::prettier_printer::Seed;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::iter::repeat_with;

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum Cell {
    Dead,
    Live,
}

impl From<Cell> for u8 {
    fn from(cell: Cell) -> Self {
        match cell {
            Cell::Dead => 0,
            Cell::Live => 1,
        }
    }
}

impl Distribution<Cell> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cell {
        if rng.gen_ratio(1, 4) {
            Cell::Live
        } else {
            Cell::Dead
        }
    }
}

#[derive(Debug)]
pub(crate) struct Board {
    arr: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Board {
    pub(crate) fn new(seed: Seed, terminal_size: (u16, u16)) -> Self {
        let mut rng = SmallRng::from_seed(seed).sample_iter(Standard);
        Self {
            arr: repeat_with(|| rng.next().unwrap())
                .take(terminal_size.0 as usize * terminal_size.1 as usize)
                .collect(),
            width: terminal_size.0 as usize,
            height: terminal_size.1 as usize,
        }
    }

    fn new_with_array(arr: Vec<Cell>, width: usize, height: usize) -> Self {
        Self { arr, width, height }
    }

    pub(crate) fn cell_array(&self) -> &[Cell] {
        &self.arr
    }

    /// width * height != 0
    fn wrap_around_index(width: usize, height: usize, index: isize) -> usize {
        debug_assert_ne!(width * height, 0);
        (((width * height) as isize + index) % (width * height) as isize).abs() as usize
    }

    pub(crate) fn tick(&mut self) {
        let original = self.arr.clone();

        let width = self.width as isize;

        for i in 0..original.len() as isize {
            let sum = {
                let index = |index: isize| Board::wrap_around_index(self.width, self.height, index);
                u8::from(original[index(i - width - 1)])
                    + u8::from(original[index(i - width)])
                    + u8::from(original[index(i - width + 1)])
                    + u8::from(original[index(i - 1)])
                    + u8::from(original[index(i + 1)])
                    + u8::from(original[index(i + width - 1)])
                    + u8::from(original[index(i + width)])
                    + u8::from(original[index(i + width + 1)])
            };

            if sum < 2 || sum > 3 {
                self.arr[i as usize] = Cell::Dead;
            } else if original[i as usize] == Cell::Dead && sum == 3 {
                self.arr[i as usize] = Cell::Live;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::panic::{catch_unwind, set_hook, take_hook, UnwindSafe};

    fn catch_unwind_silent<F: FnOnce() -> R + UnwindSafe, R>(f: F) -> std::thread::Result<R> {
        let prev_hook = take_hook();
        set_hook(Box::new(|_| {}));
        let result = catch_unwind(f);
        set_hook(prev_hook);
        result
    }

    #[rstest]
    // Rule 1
    #[case( // 1
        vec![],
        vec![],
    )]
    #[case( // 2
        vec![
            vec![0],
        ],
        vec![
            vec![0],
        ],
    )]
    #[case( // 3
        vec![
            vec![1],
        ],
        vec![
            vec![0],
        ],
    )]
    #[case( // 4
        vec![
            vec![1, 1],
        ],
        vec![
            vec![0, 0],
        ],
    )]
    // Rule 2
    #[case( // 5
        vec![
            vec![1, 1, 0, 0],
            vec![1, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ],
        vec![
            vec![1, 1, 0, 0],
            vec![1, 1, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ],
    )]
    #[case( // 6
        vec![
            vec![0, 1, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 0, 0, 0],
        ],
        vec![
            vec![0, 0, 0, 0],
            vec![1, 1, 1, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ],
    )]
    // Rule 3 and 4
    #[case( // 7
        vec![
            vec![0, 1, 0, 0],
            vec![1, 1, 1, 0],
            vec![0, 1, 0, 0],
            vec![0, 0, 0, 0],
        ],
        vec![
            vec![1, 1, 1, 0],
            vec![1, 0, 1, 0],
            vec![1, 1, 1, 0],
            vec![0, 0, 0, 0],
        ],
    )]
    fn tick(#[case] initial_cells: Vec<Vec<u8>>, #[case] expected_cells: Vec<Vec<u8>>) {
        let mut board = Board::new_with_array(
            convert_to_array(&initial_cells),
            initial_cells.get(0).map(|a| a.len()).unwrap_or_default(),
            initial_cells.len(),
        );
        board.tick();
        assert_eq!(board.arr, convert_to_array(&expected_cells));
    }

    fn convert_to_array(array: &[Vec<u8>]) -> Vec<Cell> {
        array
            .iter()
            .flat_map(|row| row.iter())
            .map(|&n| {
                if n == 0 {
                    Cell::Dead
                } else if n == 1 {
                    Cell::Live
                } else {
                    panic!("invalid cell")
                }
            })
            .collect()
    }

    #[rstest]
    #[case(1, 1, 0, 0)]
    #[case(1, 1, 1, 0)]
    #[case(1, 1, 2, 0)]
    #[case(1, 1, -1, 0)]
    #[case(1, 1, -2, 0)]
    #[case(2, 1, 2, 0)]
    #[case(2, 1, 3, 1)]
    #[case(2, 1, -1, 1)]
    #[case(2, 1, -2, 0)]
    #[case(2, 1, -3, 1)]
    fn wrap_around_index(
        #[case] width: usize,
        #[case] height: usize,
        #[case] index: isize,
        #[case] expected: usize,
    ) {
        assert_eq!(Board::wrap_around_index(width, height, index), expected);
    }

    #[test]
    fn wrap_around_index_invalid() {
        assert!(catch_unwind_silent(|| Board::wrap_around_index(0, 0, 0)).is_err());
    }
}
