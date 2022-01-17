use std::collections::{HashMap, HashSet};

use self::guess_result::{GuessResult, IntermediateLetterInfo, LetterStatus};
use super::player::Player;

pub mod guess_result;
mod tests;

pub struct Puzzle<'a, T>
where
    T: Fn(&HashSet<String>, f32, &Vec<Option<char>>, &HashMap<String, f32>) -> String,
{
    player: &'a mut Player<T>,
    answer: &'a str,
    n_turns: u8,
    completed_turns: u8,
}

impl<'a, T> Puzzle<'a, T>
where
    T: Fn(&HashSet<String>, f32, &Vec<Option<char>>, &HashMap<String, f32>) -> String,
{
    pub fn new(player: &'a mut Player<T>, answer: &'a str, n_turns: u8) -> Self {
        player.set_puzzle_rules(n_turns);
        Puzzle {
            player,
            answer,
            n_turns,
            completed_turns: 0,
        }
    }

    pub fn solve(&mut self) {
        loop {
            let guess = self.player.guess();
            println!("Turn {}: guess '{}'", self.completed_turns + 1, guess);
            let turn_res = self.turn(guess);
            println!("\t{}", turn_res);
            if let GuessResult::Continue(new_info) = turn_res {
                self.player.update_knowledge(new_info.0);
            } else {
                break;
            }
        }
    }

    fn turn(&mut self, word: String) -> GuessResult {
        self.completed_turns += 1;

        if word.to_ascii_lowercase() == self.answer.to_ascii_lowercase() {
            GuessResult::Win
        } else if self.completed_turns >= self.n_turns {
            GuessResult::Loss
        } else {
            GuessResult::Continue(IntermediateLetterInfo(
                self.answer
                    .chars()
                    .zip(word.chars())
                    .map(|(a, g)| {
                        // LetterStatus::Correct
                        if a == g {
                            LetterStatus::Correct(g)
                        } else if self.answer.contains(g) {
                            LetterStatus::InDifferentPosition(g)
                        } else {
                            LetterStatus::NotInWord(g)
                        }
                    })
                    .collect::<Vec<_>>(),
            ))
        }
    }
}
