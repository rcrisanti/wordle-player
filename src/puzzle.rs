use colored::Colorize;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use super::player::Player;

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

pub struct IntermediateLetterInfo(Vec<LetterStatus>);

impl Display for IntermediateLetterInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.len() == 5 {
            write!(
                f,
                "{} {} {} {} {}",
                self.0.get(0).unwrap(),
                self.0.get(1).unwrap(),
                self.0.get(2).unwrap(),
                self.0.get(3).unwrap(),
                self.0.get(4).unwrap()
            )
        } else {
            write!(f, "{:?}", self.0)
        }
    }
}

pub enum GuessResult {
    Win,
    Loss,
    Continue(IntermediateLetterInfo),
}

impl Display for GuessResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Win => write!(f, "Win! :)"),
            Self::Loss => write!(f, "Loss :("),
            Self::Continue(status) => write!(f, "{}", status),
        }
    }
}

#[derive(Debug)]
pub enum LetterStatus {
    Correct(char),
    InDifferentPosition(char),
    NotInWord(char),
}

impl Display for LetterStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted = match &self {
            LetterStatus::Correct(c) => {
                c.to_string().to_ascii_uppercase().on_truecolor(83, 141, 78)
            }
            LetterStatus::InDifferentPosition(c) => c
                .to_string()
                .to_ascii_uppercase()
                .on_truecolor(181, 159, 58),
            LetterStatus::NotInWord(c) => {
                c.to_string().to_ascii_uppercase().on_truecolor(58, 58, 60)
            }
        }
        .truecolor(215, 218, 220)
        .bold();

        write!(f, "{}", formatted)
    }
}
