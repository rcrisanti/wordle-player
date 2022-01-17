use std::collections::{HashMap, HashSet};

use self::guess_result::{GuessResult, IntermediateLetterInfo, LetterStatus};
use super::player::Player;

pub mod guess_result;

#[cfg(test)]
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
                    .enumerate()
                    .map(|(i, (a, g))| {
                        if a == g {
                            LetterStatus::Correct(g)
                        } else if self.answer.contains(g) {
                            // check for edge behavior when word has duplicate letters
                            if letter_occurance_in_word(i, &word)
                                > count_char_in_word(g, &self.answer)
                                || guess_letter_correct(
                                    &word,
                                    &self.answer,
                                    self.answer
                                        .chars()
                                        .position(|l| l == g)
                                        .expect("could not find character in answer"),
                                )
                            {
                                LetterStatus::NotInWord(g)
                            } else {
                                LetterStatus::InDifferentPosition(g)
                            }
                        } else {
                            LetterStatus::NotInWord(g)
                        }
                    })
                    .collect::<Vec<_>>(),
            ))
        }
    }
}

fn guess_letter_correct(guess: &str, answer: &str, idx: usize) -> bool {
    guess.to_ascii_lowercase().chars().nth(idx) == answer.to_ascii_lowercase().chars().nth(idx)
}

fn letter_occurance_in_word(idx: usize, word: &str) -> usize {
    let c = word
        .to_ascii_lowercase()
        .chars()
        .nth(idx)
        .expect("cannot get search character");

    word.to_ascii_lowercase()
        .chars()
        .enumerate()
        .filter_map(|(i, l)| if i <= idx && l == c { Some(true) } else { None })
        .count()
}

fn count_char_in_word(c: char, word: &str) -> usize {
    word.chars()
        .filter_map(|l| if l == c { Some(true) } else { None })
        .count()
}
