use rand::distributions::{Bernoulli, Distribution};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::fmt::{Debug, Display, Formatter};
use std::iter::repeat;

pub type Seed = <SmallRng as SeedableRng>::Seed;

#[derive(Debug)]
pub struct PrettierPrint<'a, T>
where
    T: Debug,
{
    inner: &'a T,
    seed: Seed,
}

impl<'a, T> PrettierPrint<'a, T>
where
    T: Debug,
{
    /// Use this to initiate multiple instances with different seeds
    pub fn new_with_seed(inner: &'a T, seed: Seed) -> Self {
        Self { inner, seed }
    }
}

impl<'a, T> From<&'a T> for PrettierPrint<'a, T>
where
    T: Debug,
{
    /// Should only be called once in code
    fn from(inner: &'a T) -> Self {
        let mut seed = Seed::default();
        getrandom::getrandom(&mut seed).expect("could not get random bytes");
        PrettierPrint::new_with_seed(inner, seed)
    }
}

pub fn gen_seed(rng: &mut SmallRng) -> Seed {
    let mut seed = Seed::default();
    rng.fill(&mut seed);
    seed
}

impl<'a, T> Display for PrettierPrint<'a, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        const RAINBOW: char = '🌈';
        const STAR: char = '⭐';

        let mut rng = SmallRng::from_seed(self.seed.clone());
        let mut line_rng = Bernoulli::from_ratio(3, 5)
            .unwrap() // Can be unwrap_unchecked()
            .sample_iter(SmallRng::from_seed(gen_seed(&mut rng)));

        let debug_str = format!("{:#?}", self.inner);
        let width = debug_str
            .lines()
            .map(|s| s.len())
            .max()
            .map_or(0, |n| n + n / 10 + 2);

        let mut result = RAINBOW.to_string();
        result.extend(repeat(' ').take(width - 2));
        result.push(RAINBOW);
        result.push('\n');

        for line in debug_str.lines() {
            result.push(' ');

            let leading_space_count = line.bytes().take_while(|&b| b == b' ').count();

            // Leading space and content
            if leading_space_count > 0 && line_rng.next().unwrap() {
                let star_index = rng.gen_range(0..leading_space_count);
                result.extend(repeat(' ').take(star_index));
                result.push(STAR);
                result.extend(repeat(' ').take(leading_space_count - star_index - 1));

                result += line.split_at(leading_space_count).1;
            } else {
                result.push_str(line);
            }

            // Trailing stars
            if line_rng.next().unwrap() {
                let star_index = rng.gen_range(0..width - line.len());
                result.extend(repeat(' ').take(star_index));
                result.push(STAR);
            }

            // Remove extra spaces
            while result.ends_with(' ') {
                result.pop();
            }

            result.push('\n');
        }

        result.push(RAINBOW);
        result.extend(repeat(' ').take(width - 2));
        result.push(RAINBOW);
        result.push('\n');

        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test() {
        let seed = {
            let mut seed = Seed::default();
            seed[0] = 15;
            seed
        };
        {
            let result = PrettierPrint::new_with_seed(&0, seed).to_string();
            assert!(result.starts_with("🌈 🌈\n"));
            assert!(result.ends_with("🌈 🌈\n"));
            assert!(result.contains(' '));
        }
        {
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

            let result = PrettierPrint::new_with_seed(&input, seed).to_string();
            assert!(result.starts_with("🌈                         🌈\n"));
            assert!(result.ends_with("🌈                         🌈\n"));
            println!("{}", result);
        }
    }
}
