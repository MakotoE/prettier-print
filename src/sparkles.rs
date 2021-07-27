use crate::prettier_printer::Seed;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Sparkles {
    rng: SmallRng,
}

impl Sparkles {
    pub fn new() -> Self {
        Self {
            rng: SmallRng::from_entropy(),
        }
    }

    pub fn new_with_seed(seed: Seed) -> Self {
        Self {
            rng: SmallRng::from_seed(seed),
        }
    }

    pub fn output<T>(what: &T)
    where
        T: Debug,
    {
    }
}
