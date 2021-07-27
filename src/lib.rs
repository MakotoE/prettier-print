use rand::distributions::{Bernoulli, Distribution};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::fmt::{Debug, Display, Formatter};
use std::iter::repeat;

pub type Seed = <SmallRng as SeedableRng>::Seed;

#[derive(Debug, Clone)]
pub struct PrettierPrinter {
    rng: SmallRng,
}

impl PrettierPrinter {
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

    pub fn gen_seed(rng: &mut SmallRng) -> Seed {
        let mut seed = Seed::default();
        rng.fill(&mut seed);
        seed
    }

    pub fn print<'a, T>(&mut self, inner: &'a T) -> PrettierPrintDisplayer<'a, T> {
        PrettierPrintDisplayer {
            seed: PrettierPrinter::gen_seed(&mut self.rng),
            inner,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrettierPrintDisplayer<'a, T> {
    seed: Seed,
    inner: &'a T,
}

impl<T> PrettierPrintDisplayer<'_, T> {
    fn output(seed: Seed, debug_str: &str, f: &mut Formatter<'_>) -> std::fmt::Result {
        const RAINBOW: char = '🌈';
        const STAR: char = '⭐';
        const COLORED_STAR: char = '🌟';

        let mut rng = SmallRng::from_seed(seed.clone());
        let mut line_rng = Bernoulli::from_ratio(3, 5)
            .unwrap() // Can be unwrap_unchecked()
            .sample_iter(SmallRng::from_seed(PrettierPrinter::gen_seed(&mut rng)));

        let mut star_rng = Bernoulli::from_ratio(1, 6)
            .unwrap() // Can be unwrap_unchecked()
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
                let star_index = rng.gen_range(0..leading_space_count);
                result.extend(repeat(' ').take(star_index));

                if star_rng.next().unwrap() {
                    debug_assert!(result.ends_with(' '));
                    result.pop().unwrap(); // Compensate for wider width of emoji
                    result.push(COLORED_STAR);
                } else {
                    result.push(STAR);
                }
                result.extend(repeat(' ').take(leading_space_count - star_index - 1));

                result += line.split_at(leading_space_count).1;
            } else {
                result.push_str(line);
            }

            // Trailing stars
            if line_rng.next().unwrap() {
                let star_index = rng.gen_range(0..width - line.len());
                result.extend(repeat(' ').take(star_index));
                result.push(if star_rng.next().unwrap() {
                    COLORED_STAR
                } else {
                    STAR
                });
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

impl<T> Display for PrettierPrintDisplayer<'_, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        PrettierPrintDisplayer::<T>::output(self.seed, &format!("{:#?}", self.inner), f)
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
            seed[0] = 170; // Good seed for example
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
            // println!("{}", result);
        }
    }
}
