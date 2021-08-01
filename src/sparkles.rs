use crate::game_of_life::{Board, Cell};
use crate::prettier_printer::{PrettierPrinter, Seed};
use crossterm::cursor;
use crossterm::cursor::{MoveTo, MoveToNextLine};
use crossterm::event::poll;
use crossterm::style::{Color, Colors, Print, SetBackgroundColor, SetColors};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::{queue, terminal};
use rand::rngs::SmallRng;
use rand::SeedableRng;
use std::fmt::Debug;
use std::io::{StdoutLock, Write};
use std::iter::once;
use std::str::Chars;
use std::thread::sleep;
use std::time::Duration;

/// Prints the debug string, and runs game of life on top of the printed string. The output covers
/// the full terminal screen.
///
/// The frame rate is very slow on Windows and I don't know why.
pub struct Sparkles<'stream> {
    rng: SmallRng,
    stdout: StdoutLock<'stream>,
}

impl<'stream> Sparkles<'stream> {
    /// Initializes with random seed.
    pub fn new(stdout: StdoutLock<'stream>) -> Self {
        Self {
            rng: SmallRng::from_entropy(),
            stdout,
        }
    }

    pub fn new_with_seed(seed: Seed, stdout: StdoutLock<'stream>) -> Self {
        Self {
            rng: SmallRng::from_seed(seed),
            stdout,
        }
    }

    /// Runs the output screen. Press any key to stop.
    pub fn run<T>(&mut self, what: &T) -> std::io::Result<()>
    where
        T: Debug,
    {
        enable_raw_mode().unwrap();
        queue!(
            self.stdout,
            Clear(ClearType::All),
            MoveTo(0, 0),
            SetColors(Colors::new(Color::Reset, Color::Reset)),
            cursor::Hide,
        )?;

        let terminal_size = terminal::size().unwrap();

        let debug_str = format!("{:#?}", what);

        let mut board = Board::new(PrettierPrinter::gen_seed(&mut self.rng), terminal_size);
        while !poll(Duration::from_secs(0))? {
            queue!(self.stdout, MoveTo(0, 0))?;

            let mut debug_str = CenteredDebugString::new(
                &debug_str,
                (terminal_size.0 as usize, terminal_size.1 as usize),
            );

            for (i, cell) in board.cell_array().iter().enumerate() {
                let color = match cell {
                    Cell::Dead => Color::Reset,
                    Cell::Live => Color::White,
                };
                queue!(
                    self.stdout,
                    SetBackgroundColor(color),
                    Print(debug_str.next().unwrap())
                )?;

                // Line break
                if i % terminal_size.0 as usize == terminal_size.0 as usize - 1 {
                    queue!(
                        self.stdout,
                        SetBackgroundColor(Color::Reset),
                        MoveToNextLine(1),
                    )?;
                }
                self.stdout.flush()?;
            }

            board.tick();

            sleep(Duration::from_millis(50));
        }

        disable_raw_mode().unwrap();
        queue!(
            self.stdout,
            SetColors(Colors::new(Color::Reset, Color::Reset)),
            cursor::Show,
        )?;
        self.stdout.flush()
    }
}

/// Turns the debug string into a grid of chars.  
pub struct CenteredDebugString<'chars> {
    char_iter: Chars<'chars>,
    top_margin_length: usize,
    left_margin_length: usize,
    terminal_size: (usize, usize),
    curr_index: usize,
    in_right_side: bool,
}

impl<'chars> CenteredDebugString<'chars> {
    pub fn new(s: &'chars str, terminal_size: (usize, usize)) -> Self {
        Self {
            char_iter: s.chars(),
            top_margin_length: CenteredDebugString::margin_length(
                terminal_size.1,
                s.chars().filter(|&c| c == '\n').count() + 1,
            ),
            left_margin_length: CenteredDebugString::margin_length(
                terminal_size.0,
                CenteredDebugString::longest_line(s),
            ),
            curr_index: 0,
            terminal_size,
            in_right_side: false,
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

    pub fn len(&self) -> usize {
        self.terminal_size.0 * self.terminal_size.1
    }
}

impl Iterator for CenteredDebugString<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        const SPACE: char = ' ';

        let result = if self.curr_index / self.terminal_size.0 < self.top_margin_length {
            // Top margin
            SPACE
        } else if self.curr_index % self.terminal_size.0 < self.left_margin_length {
            // Left margin
            self.in_right_side = false;
            SPACE
        } else if self.in_right_side {
            // Right spacing
            SPACE
        } else if let Some(c) = self.char_iter.next() {
            if c == '\n' {
                self.in_right_side = true;
                SPACE
            } else {
                c
            }
        } else {
            // Bottom spacing
            SPACE
        };
        self.curr_index += 1;
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::collections::HashMap;
    use std::io::stdout;

    // #[test]
    #[allow(dead_code)]
    fn run_sparkles() {
        #[derive(Debug)]
        struct Type {
            a: String,
            b: Vec<i32>,
            c: HashMap<&'static str, &'static str>,
        }

        let input = Type {
            a: "a".to_string(),
            b: vec![0, 1],
            c: {
                let mut map = HashMap::new();
                map.insert("So", "pretty");
                map
            },
        };

        let stdout = stdout();
        Sparkles::new(stdout.lock()).run(&input).unwrap();
    }

    #[rstest]
    #[case("", (0, 0), &[])]
    #[case("a", (0, 0), &[])]
    #[case("a", (1, 1), &['a'])]
    #[case("a", (2, 3), &[' ', ' ', 'a', ' ', ' ', ' '])]
    #[case("a", (3, 2), &[' ', 'a', ' ', ' ', ' ', ' '])]
    #[case("a", (3, 3), &[' ', ' ', ' ', ' ', 'a', ' ', ' ', ' ', ' '])]
    #[case("a\nb", (4, 3), &[' ', 'a', ' ', ' ', ' ', 'b', ' ', ' ', ' ', ' ', ' ', ' '])]
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
