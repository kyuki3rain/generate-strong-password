use rand::{
    distributions::WeightedIndex, prelude::Distribution, rngs::ThreadRng, seq::SliceRandom,
};

use crate::character_set::CharacterSet;

static UPPERCASE_SETS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
static LOWERCASE_SETS: &str = "abcdefghijklmnopqrstuvwxyz";
static NUMERIC_SETS: &str = "0123456789";

pub struct PasswordGenerator {
    length: usize,
    character_sets: Vec<CharacterSet>,
    weights: WeightedIndex<usize>,
    includes: Vec<bool>,
}

impl PasswordGenerator {
    pub fn new(length: usize, weights: Vec<usize>, symbol_sets: &str) -> Self {
        let character_sets: Vec<CharacterSet> = vec![
            CharacterSet::new(UPPERCASE_SETS),
            CharacterSet::new(LOWERCASE_SETS),
            CharacterSet::new(NUMERIC_SETS),
            CharacterSet::new(symbol_sets),
        ];

        let includes = weights.iter().map(|&x| x != 0).collect::<Vec<bool>>();
        let weights = WeightedIndex::new(weights).unwrap();

        Self {
            length,
            character_sets,
            weights,
            includes,
        }
    }

    pub fn gen(&self, rng: &mut ThreadRng) -> Result<String, String> {
        let mut password = self.gen_minimum_password(rng);

        if self.length < password.len() {
            return Err(format!(
                "Password length must be at least {} (one character per enabled type)",
                password.len()
            ));
        }

        for _ in password.len()..self.length {
            password.push(self.choose(rng));
        }

        password.shuffle(rng);

        Ok(password.into_iter().collect::<String>())
    }

    fn choose(&self, rng: &mut ThreadRng) -> char {
        self.character_sets[self.weights.sample(rng)].choose(rng)
    }

    fn gen_minimum_password(&self, rng: &mut ThreadRng) -> Vec<char> {
        let mut password = Vec::new();

        for (i, character_set) in self.character_sets.iter().enumerate() {
            if self.includes[i] {
                password.push(character_set.choose(rng));
            }
        }

        password
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let password_generator = PasswordGenerator::new(10, vec![1, 1, 1, 1], "!@#$%^&*()");

        assert_eq!(password_generator.length, 10);
        assert_eq!(password_generator.includes, vec![true, true, true, true]);
    }

    #[test]
    fn test_gen() {
        let mut rng = rand::thread_rng();
        let password_generator = PasswordGenerator::new(10, vec![1, 1, 1, 1], "!@#$%^&*()");

        assert_eq!(password_generator.gen(&mut rng).unwrap().len(), 10);
    }

    #[test]
    fn test_gen_error_when_length_too_short() {
        let mut rng = rand::thread_rng();
        let password_generator = PasswordGenerator::new(3, vec![1, 1, 1, 1], "!@#$%^&*()");

        assert!(password_generator.gen(&mut rng).is_err());
    }
}
