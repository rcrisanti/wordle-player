use colored::Colorize;
use std::{collections::HashSet, fmt::Display, num::IntErrorKind};

use super::player::Player;

pub struct Game<T>
where
    T: Fn(&HashSet<String>) -> String,
{
    player: Player<T>,
    answer: &'static str,
    n_turns: u8,
    completed_turns: u8,
}

impl<T> Game<T>
where
    T: Fn(&HashSet<String>) -> String,
{
    pub fn new(player: Player<T>, answer: &'static str, n_turns: u8) -> Self {
        Game {
            player,
            answer,
            n_turns,
            completed_turns: 0,
        }
    }

    pub fn play(&mut self) {
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

struct IntermediateLetterInfo(Vec<LetterStatus>);

// impl Display for IntermediateLetterInfo {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let fmt_places = vec!["{}".to_string(); self.0.len()].join("");
//     }
// }

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
            Self::Continue(status) => write!(f, "{:?}", status.0),
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
        match &self {
            LetterStatus::Correct(l) => write!(f, "{}", l.to_string().white().on_green()),
            LetterStatus::InDifferentPosition(l) => {
                write!(f, "{}", l.to_string().white().on_yellow())
            }
            LetterStatus::NotInWord(l) => write!(f, "{}", l.to_string().white().on_bright_black()),
        }
    }
}