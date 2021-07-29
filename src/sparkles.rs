use crate::game_of_life::{Board, Cell};
use crate::prettier_printer::Seed;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use std::fmt::Debug;
use std::io::{stdin, Read, StdinLock, StdoutLock, Write};
use std::iter::once;
use std::str::Chars;
use std::thread::sleep;
use std::time::Duration;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, color, cursor, terminal_size, AsyncReader};

pub struct Sparkles<'stdout> {
    rng: SmallRng,
    stdout: RawTerminal<StdoutLock<'stdout>>,
    stdin: AsyncReader,
}

impl<'stdout> Sparkles<'stdout> {
    pub fn new(stdout: StdoutLock<'stdout>, stdin: AsyncReader) -> Self {
        Self {
            rng: SmallRng::from_entropy(),
            stdout: stdout.into_raw_mode().unwrap(),
            stdin,
        }
    }

    pub fn new_with_seed(seed: Seed, stdout: StdoutLock<'stdout>, stdin: AsyncReader) -> Self {
        Self {
            rng: SmallRng::from_seed(seed),
            stdout: stdout.into_raw_mode().unwrap(),
            stdin,
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

        // let terminal_size = terminal_size().unwrap();
        let terminal_size = (50, 20);

        let s = format!("{:#?}", what);

        let mut board = Board::new(Seed::default(), terminal_size);
        while self.stdin.by_ref().bytes().next().is_none() {
            let mut debug_str =
                CenteredDebugString::new(&s, (terminal_size.0 as usize, terminal_size.1 as usize));

            for (i, cell) in board.cell_array().iter().enumerate() {
                match cell {
                    Cell::Dead => write!(self.stdout, "{}", color::Bg(color::Reset))?,
                    Cell::Live => write!(self.stdout, "{}", color::Bg(color::LightWhite))?,
                };
                write!(self.stdout, "{}", debug_str.next().unwrap())?;

                // Line break
                if i % terminal_size.0 as usize == terminal_size.0 as usize - 1 {
                    write!(
                        self.stdout,
                        "{}{}",
                        color::Bg(color::Reset),
                        cursor::Goto(0, i as u16 / terminal_size.0 + 1)
                    )?;
                }
                self.stdout.flush()?;
            }

            board.tick();

            sleep(Duration::from_millis(50));
        }

        write!(
            self.stdout,
            "{}{}",
            color::Bg(color::Reset),
            color::Fg(color::Reset)
        )
    }
}

struct CenteredDebugString<'chars> {
    char_iter: Chars<'chars>,
    top_margin_length: usize,
    left_margin_length: usize,
    terminal_size: (usize, usize),
    curr_index: usize,
    in_right_side: bool,
}

impl<'chars> CenteredDebugString<'chars> {
    fn new(s: &'chars str, terminal_size: (usize, usize)) -> Self {
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

    fn len(&self) -> usize {
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
    use termion::async_stdin;
    use termion::raw::IntoRawMode;

    #[test]
    fn test() {
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
        let stdin = stdin();
        let mut sparkles = Sparkles::new(stdout.lock(), async_stdin());
        sparkles.output(&input).unwrap();
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
