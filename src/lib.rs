use rand::distributions::{Bernoulli, Distribution};
use rand::rngs::SmallRng;
use rand::SeedableRng;
use std::fmt::{Debug, Display, Formatter};
use std::iter::{once, repeat};

#[derive(Debug)]
pub struct PrettierPrint<'a, T>
where
    T: Debug,
{
    inner: &'a T,
    seed: <SmallRng as SeedableRng>::Seed,
}

impl<'a, T> PrettierPrint<'a, T>
where
    T: Debug,
{
    /// Use this to initiate multiple instances with different seeds
    pub fn new_with_seed(inner: &'a T, seed: <SmallRng as SeedableRng>::Seed) -> Self {
        Self { inner, seed }
    }
}

impl<'a, T> From<&'a T> for PrettierPrint<'a, T>
where
    T: Debug,
{
    /// Should only be called once in code
    fn from(inner: &'a T) -> Self {
        let mut seed = <SmallRng as SeedableRng>::Seed::default();
        getrandom::getrandom(&mut seed).expect("could not get random bytes");
        PrettierPrint::new_with_seed(inner, seed)
    }
}

impl<'a, T> Display for PrettierPrint<'a, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut rng = Bernoulli::from_ratio(1, 8)
            .unwrap() // Can be unwrap_unchecked()
            .sample_iter(SmallRng::from_seed(self.seed.clone()));

        let debug_str = format!("{:#?}", self.inner);
        let width = debug_str
            .lines()
            .map(|s| s.len())
            .max()
            .map_or(0, |n| n + n / 10 + 2);

        let mut result: String = repeat(' ')
            .take(width)
            .map(|c| if rng.next().unwrap() { '⭐' } else { c })
            .chain(once('\n'))
            .collect();

        for line in debug_str.lines() {
            let leading_whitespace_count = line.chars().take_while(|c| c.is_whitespace()).count();
            let mut char_iter = line.chars();

            // Leading space
            result.extend(char_iter.by_ref().take(leading_whitespace_count).map(|c| {
                if rng.next().unwrap() {
                    '⭐'
                } else {
                    c
                }
            }));

            // Content
            result.extend(char_iter);

            // Trailing space
            result.extend(repeat(' ').take(width - line.len()).map(|c| {
                if rng.next().unwrap() {
                    '⭐'
                } else {
                    c
                }
            }));

            result.push('\n');
        }

        result.extend(
            repeat(' ')
                .take(width)
                .map(|c| if rng.next().unwrap() { '⭐' } else { c }),
        );
        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("{}", PrettierPrint::from(&vec!["ab", "cd"]));
    }
}
