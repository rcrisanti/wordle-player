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
    Win(u8),
    Loss,
    Continue(IntermediateLetterInfo),
}

impl Display for GuessResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Win(_) => write!(f, "Win! :)"),
            Self::Loss => write!(f, "Loss :("),
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
