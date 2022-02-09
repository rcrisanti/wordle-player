use std::io;

use super::Strategy;

pub struct UserInputStrategy {}

impl UserInputStrategy {
    pub fn new() -> Self {
        UserInputStrategy {}
    }
}

impl Strategy for UserInputStrategy {
    fn best_word(
        &self,
        words: &std::collections::HashSet<String>,
        _current_state: &Vec<Option<char>>,
    ) -> String {
        let mut guess = String::new();
        println!("Please input your guess:");
        io::stdin()
            .read_line(&mut guess)
            .expect("could not read line")
            .to_string();

        guess = guess.trim().to_ascii_lowercase();

        if words.contains(&guess) {
            guess
        } else {
            println!(
                "Your guess '{}' does not exist in our dictionary. Try again!",
                guess
            );
            self.best_word(words, _current_state)
        }
    }
}
