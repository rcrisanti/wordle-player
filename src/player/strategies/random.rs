use super::Strategy;
use rand::Rng;
use std::collections::HashSet;

pub struct RandomStrategy {}

impl RandomStrategy {
    pub fn new() -> Self {
        RandomStrategy {}
    }
}

impl Strategy for RandomStrategy {
    fn best_word(&self, words: &HashSet<String>, _current_state: &Vec<Option<char>>) -> String {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..words.len());
        match words.iter().collect::<Vec<_>>().get(index) {
            Some(word) => word.to_string(),
            None => "words".to_string(),
        }
    }
}
