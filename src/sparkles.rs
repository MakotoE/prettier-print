use crate::game_of_life::{Board, Cell};
use crate::prettier_printer::Seed;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use std::fmt::Debug;
use std::io::{stdout, Stdout, StdoutLock, Write};
use termion::raw::IntoRawMode;
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
        write!(self.stdout, "{}", clear::All)?;
        write!(self.stdout, "{}", cursor::Goto(1, 1))?;

        let debug_str = format!("{:?}", what);

        let mut board = Board::new(Seed::default(), terminal_size().unwrap());
        loop {
            for cell in board.cell_array() {
                match cell {
                    Cell::Dead => write!(self.stdout, "{} ", color::Bg(color::Reset))?,
                    Cell::Live => write!(self.stdout, "{} ", color::Bg(color::LightWhite))?,
                };
            }
            board.tick();
        }
        write!(self.stdout, "{} ", color::Bg(color::Reset))
    }
}

#[test]
fn test() {
    let stdout = stdout();
    let mut stdout = stdout.lock();
    let mut sparkles = Sparkles::new(stdout);
    sparkles.output(&0);
}
