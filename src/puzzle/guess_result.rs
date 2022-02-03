use colored::Colorize;
use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub struct IntermediateLetterInfo(pub Vec<LetterStatus>);

impl Display for IntermediateLetterInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0.len() {
            1 => write!(f, "{}", self.0.get(0).unwrap()),
            2 => write!(f, "{} {}", self.0.get(0).unwrap(), self.0.get(1).unwrap()),
            3 => write!(
                f,
                "{} {} {}",
                self.0.get(0).unwrap(),
                self.0.get(1).unwrap(),
                self.0.get(2).unwrap()
            ),
            4 => write!(
                f,
                "{} {} {} {}",
                self.0.get(0).unwrap(),
                self.0.get(1).unwrap(),
                self.0.get(2).unwrap(),
                self.0.get(3).unwrap()
            ),
            5 => write!(
                f,
                "{} {} {} {} {}",
                self.0.get(0).unwrap(),
                self.0.get(1).unwrap(),
                self.0.get(2).unwrap(),
                self.0.get(3).unwrap(),
                self.0.get(4).unwrap()
            ),
            6 => write!(
                f,
                "{} {} {} {} {} {}",
                self.0.get(0).unwrap(),
                self.0.get(1).unwrap(),
                self.0.get(2).unwrap(),
                self.0.get(3).unwrap(),
                self.0.get(4).unwrap(),
                self.0.get(5).unwrap()
            ),
            7 => write!(
                f,
                "{} {} {} {} {} {} {}",
                self.0.get(0).unwrap(),
                self.0.get(1).unwrap(),
                self.0.get(2).unwrap(),
                self.0.get(3).unwrap(),
                self.0.get(4).unwrap(),
                self.0.get(5).unwrap(),
                self.0.get(6).unwrap()
            ),
            8 => write!(
                f,
                "{} {} {} {} {} {} {} {}",
                self.0.get(0).unwrap(),
                self.0.get(1).unwrap(),
                self.0.get(2).unwrap(),
                self.0.get(3).unwrap(),
                self.0.get(4).unwrap(),
                self.0.get(5).unwrap(),
                self.0.get(6).unwrap(),
                self.0.get(7).unwrap()
            ),
            _ => write!(f, "{:?}", self.0),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum GuessResult {
    Win(IntermediateLetterInfo, u8),
    Loss(IntermediateLetterInfo),
    Continue(IntermediateLetterInfo),
}

impl Display for GuessResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Win(status, _) => write!(f, "{}\n\tWin! :)", status),
            Self::Loss(status) => write!(f, "{}\n\tLoss :(", status),
            Self::Continue(status) => write!(f, "{}", status),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LetterStatus {
    Correct(char),
    InDifferentPosition(char),
    NotInWord(char),
}

impl Display for LetterStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted = match &self {
            LetterStatus::Correct(c) => format!(" {} ", c).to_ascii_uppercase().on_green(),
            LetterStatus::InDifferentPosition(c) => {
                format!(" {} ", c).to_ascii_uppercase().on_yellow()
            }
            LetterStatus::NotInWord(c) => format!(" {} ", c).to_ascii_uppercase().on_bright_black(),
        }
        .bright_white()
        .bold();

        write!(f, "{}", formatted)
    }
}
