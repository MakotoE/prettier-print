use crate::game_of_life::{Board, Cell};
use crate::prettier_printer::Seed;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use std::fmt::Debug;
use std::io::{stdout, StdoutLock, Write};
use std::iter::once;
use std::str::Chars;
use termion::{clear, color, cursor, terminal_size};

#[derive(Debug)]
pub struct Sparkles<'stdout> {
    rng: SmallRng,
    stdout: StdoutLock<'stdout>,
}

impl<'stdout> Sparkles<'stdout> {
    pub fn new(stdout: StdoutLock<'stdout>) -> Self {
        Self {
            rng: SmallRng::from_entropy(),
            stdout,
        }
    }

    pub fn new_with_seed(seed: Seed, stdout: StdoutLock<'stdout>) -> Self {
        Self {
            rng: SmallRng::from_seed(seed),
            stdout,
        }
    }

    pub fn output<T>(&mut self, what: &T) -> std::io::Result<()>
    where
        T: Debug,
    {
        write!(
            self.stdout,
            "{}{}{}{}",
            clear::All,
            cursor::Goto(1, 1),
            color::Bg(color::Reset),
            color::Fg(color::Reset),
        )?;

        let terminal_size = terminal_size().unwrap();
        let s = format!("{:?}", what);
        let mut debug_str =
            CenteredDebugString::new(&s, (terminal_size.0 as usize, terminal_size.1 as usize));

        let mut board = Board::new(Seed::default(), terminal_size);
        loop {
            for cell in board.cell_array() {
                let c = debug_str.next().unwrap();
                match cell {
                    Cell::Dead => write!(self.stdout, "{}{}", color::Bg(color::Reset), c)?,
                    Cell::Live => write!(self.stdout, "{} ", color::Bg(color::LightWhite))?,
                };
            }
            // board.tick();
            break;
        }
        write!(
            self.stdout,
            "{}{}",
            color::Bg(color::Reset),
            color::Fg(color::Reset)
        )?;
        self.stdout.flush()
    }
}

struct CenteredDebugString<'c> {
    char_iter: Chars<'c>,
    top_margin_length: usize,
    left_margin_length: usize,
    curr_index: usize,
    terminal_size: (usize, usize),
}

impl<'c> CenteredDebugString<'c> {
    fn new(s: &'c str, terminal_size: (usize, usize)) -> Self {
        Self {
            char_iter: s.chars(),
            top_margin_length: CenteredDebugString::margin_length(
                terminal_size.1,
                s.chars().filter(|&c| c == '\n').count() + 1,
            ),
            left_margin_length: CenteredDebugString::margin_length(
                terminal_size.0,
                CenteredDebugString::longest_line(&s),
            ),
            curr_index: 0,
            terminal_size,
        }
    }

    fn longest_line(s: &str) -> usize {
        let mut max = 0_usize;
        let mut curr_line_length = 0_usize;
        for c in s.chars().chain(once('\n')) {
            if c == '\n' {
                if curr_line_length > max {
                    max = curr_line_length;
                }
                curr_line_length = 0;
            } else {
                curr_line_length += 1;
            }
        }
        max
    }

    fn margin_length(max_length: usize, content_length: usize) -> usize {
        (max_length.saturating_sub(content_length)) / 2
    }

    fn len(&self) -> usize {
        self.terminal_size.0 * self.terminal_size.1
    }
}

impl Iterator for CenteredDebugString<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.curr_index / self.terminal_size.1 < self.top_margin_length {
            ' '
        } else if self.curr_index % self.terminal_size.0 < self.left_margin_length {
            ' '
        } else if let Some(c) = self.char_iter.next() {
            if c == '\n' {
                ' '
            } else {
                c
            }
        } else {
            ' '
        };
        self.curr_index += 1;
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test() {
        let stdout = stdout();
        let mut sparkles = Sparkles::new(stdout.lock());
        sparkles.output(&0).unwrap();
    }

    #[rstest]
    #[case("", (0, 0), &[])]
    #[case("a", (0, 0), &[])]
    #[case("a", (1, 1), &['a'])]
    #[case("a", (3, 3), &[' ', ' ', ' ', ' ', 'a', ' ', ' ', ' ', ' '])]
    fn debug_string_grid(
        #[case] s: &str,
        #[case] terminal_size: (usize, usize),
        #[case] expected: &[char],
    ) {
        let mut debug_string_grid = CenteredDebugString::new(s, terminal_size);
        let result: Vec<char> = (0..debug_string_grid.len())
            .map(|_| debug_string_grid.next().unwrap())
            .collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn longest_line() {
        assert_eq!(CenteredDebugString::longest_line(""), 0);
        assert_eq!(CenteredDebugString::longest_line("\n"), 0);
        assert_eq!(CenteredDebugString::longest_line("1\n"), 1);
        assert_eq!(CenteredDebugString::longest_line("\n1"), 1);
    }
}
