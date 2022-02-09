use colored::Colorize;
use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub struct IntermediateLetterInfo(pub Vec<LetterStatus>);

impl Display for IntermediateLetterInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|l| { format!("{}", l) })
                .collect::<String>()
        )
    }
}

#[derive(PartialEq, Debug)]
pub enum GuessResult {
    Win(IntermediateLetterInfo, u8),
    Loss(IntermediateLetterInfo, String),
    Continue(IntermediateLetterInfo),
}

impl Display for GuessResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Win(status, turns) => {
                write!(f, "{status}\nCongrats! You won in {turns} turns :)")
            }
            Self::Loss(status, answer) => write!(
                f,
                "{status}\nSo close! You couldn't quite guess \"{answer}\" :("
            ),
            Self::Continue(status) => write!(f, "{status}"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LetterStatus {
    Correct(char),
    Misplaced(char),
    Incorrect(char),
}

impl Display for LetterStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted = match &self {
            LetterStatus::Correct(c) => format!(" {} ", c).to_ascii_uppercase().on_green(),
            LetterStatus::Misplaced(c) => format!(" {} ", c).to_ascii_uppercase().on_yellow(),
            LetterStatus::Incorrect(c) => format!(" {} ", c).to_ascii_uppercase().on_bright_black(),
        }
        .bright_white()
        .bold();

        write!(f, "{}", formatted)
    }
}
