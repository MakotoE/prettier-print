use rand::distributions::{Bernoulli, Distribution};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use rand_distr::WeightedAliasIndex;
use std::fmt::{Debug, Display, Formatter};
use std::iter::repeat;

pub type Seed = <SmallRng as SeedableRng>::Seed;

/// Outputs a prettier-printed version of the `Debug` string of a variable.
#[derive(Debug, Clone)]
pub struct PrettierPrinter {
    rng: SmallRng,
}

impl PrettierPrinter {
    /// Instantiates `PrettierPrinter` with given seed. See also `default()`.
    pub fn new_with_seed(seed: Seed) -> Self {
        Self {
            rng: SmallRng::from_seed(seed),
        }
    }

    /// Generates a `Seed` from given `SmallRng`.
    pub fn gen_seed(rng: &mut SmallRng) -> Seed {
        let mut seed = Seed::default();
        rng.fill(&mut seed);
        seed
    }

    /// Pass your variable to this.
    pub fn print<'a, T>(&mut self, inner: &'a T) -> PrettierPrintDisplayer<'a, T> {
        PrettierPrintDisplayer {
            seed: PrettierPrinter::gen_seed(&mut self.rng),
            inner,
        }
    }
}

impl Default for PrettierPrinter {
    /// Use this if you want to keep things simple. Calls `getrandom()` to get seed.
    fn default() -> Self {
        Self {
            rng: SmallRng::from_entropy(),
        }
    }
}

/// Implements `Display` to output the prettier-printed debug string.
#[derive(Debug, Clone)]
pub struct PrettierPrintDisplayer<'a, T> {
    seed: Seed,
    inner: &'a T,
}

impl<T> PrettierPrintDisplayer<'_, T> {
    pub fn output(seed: Seed, debug_str: &str) -> String {
        const RAINBOW: char = '🌈';
        const STARS: &[char] = &['⭐', '🌟', '☀'];
        let weights: Vec<u8> = vec![15, 3, 1];

        let mut rng = SmallRng::from_seed(seed);
        let mut line_rng = Bernoulli::from_ratio(3, 5)
            .unwrap() // Can be unwrap_unchecked() when API is stabilized
            .sample_iter(SmallRng::from_seed(PrettierPrinter::gen_seed(&mut rng)));

        let mut star_rng = WeightedAliasIndex::new(weights.to_vec())
            .unwrap()
            .sample_iter(SmallRng::from_seed(PrettierPrinter::gen_seed(&mut rng)));

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
                // Add star to line
                let star_index = rng.gen_range(0..leading_space_count);
                result.extend(repeat(' ').take(star_index));

                result.push(STARS[star_rng.next().unwrap()]);
                result.extend(repeat(' ').take(leading_space_count - star_index - 1));

                result += line.split_at(leading_space_count).1;
            } else {
                // No star
                result.push_str(line);
            }

            // Trailing stars
            if line_rng.next().unwrap() {
                let star_index = rng.gen_range(0..width - line.len());
                result.extend(repeat(' ').take(star_index));
                result.push(STARS[star_rng.next().unwrap()]);
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
        result
    }
}

impl<T> Display for PrettierPrintDisplayer<'_, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            PrettierPrintDisplayer::<T>::output(self.seed, &format!("{:#?}", self.inner))
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn prettier_printer() {
        let seed = {
            let mut seed = Seed::default();
            seed[0] = 180; // Good seed for example
            seed
        };
        {
            let result = PrettierPrinter::new_with_seed(seed).print(&0).to_string();
            assert!(result.starts_with("🌈 🌈\n"));
            assert!(result.ends_with("🌈 🌈\n"));
            assert!(result.contains(' '));
        }
        {
            #[derive(Debug, Clone)]
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

            let displayer = PrettierPrinter::new_with_seed(seed).print(&input);

            let result = displayer.to_string();
            assert!(result.starts_with("🌈                         🌈\n"));
            assert!(result.ends_with("🌈                         🌈\n"));
            // Check if cloned Displayer outputs the same string
            assert_eq!(result, displayer.clone().to_string());

            println!("{:#?}", &input);
            println!("{}", result);
        }
    }
}
