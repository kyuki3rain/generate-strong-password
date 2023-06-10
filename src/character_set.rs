use std::collections::HashSet;

use rand::{rngs::ThreadRng, seq::SliceRandom};

pub struct CharacterSet(Vec<char>);

impl CharacterSet {
    pub fn new(str: &str) -> Self {
        let mut character_set = Self(str.chars().collect::<Vec<char>>());

        character_set.remove_duplicates();
        character_set.sort();

        character_set
    }

    pub fn choose(&self, rng: &mut ThreadRng) -> char {
        *self.0.choose(rng).unwrap()
    }

    fn sort(&mut self) {
        self.0.sort();
    }

    fn remove_duplicates(&mut self) {
        let mut unique_chars: Vec<char> = Vec::new();
        let mut seen_chars: HashSet<char> = HashSet::new();

        for c in &self.0 {
            if !seen_chars.contains(c) {
                seen_chars.insert(*c);
                unique_chars.push(*c);
            }
        }

        self.0 = unique_chars;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let character_set = CharacterSet::new("abcabcabc");

        assert_eq!(character_set.0, vec!['a', 'b', 'c']);
    }

    #[test]
    fn test_choose() {
        let mut rng = rand::thread_rng();
        let character_set = CharacterSet::new("abc");

        assert!(character_set.0.contains(&character_set.choose(&mut rng)));
    }

    #[test]
    fn test_sort() {
        let mut character_set = CharacterSet::new("cba");

        character_set.sort();

        assert_eq!(character_set.0, vec!['a', 'b', 'c']);
    }

    #[test]
    fn test_remove_duplicates() {
        let mut character_set = CharacterSet::new("abcabcabc");

        character_set.remove_duplicates();

        assert_eq!(character_set.0, vec!['a', 'b', 'c']);
    }
}
