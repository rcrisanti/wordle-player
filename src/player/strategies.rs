use std::collections::HashSet;

mod letter_frequency;
mod random;
mod user_input;

pub use letter_frequency::LetterFrequencyStrategy;
pub use random::RandomStrategy;
pub use user_input::UserInputStrategy;

pub trait Strategy {
    fn best_word(&self, words: &HashSet<String>, current_state: &Vec<Option<char>>) -> String;
}
