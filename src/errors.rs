use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct ImpossiblePuzzleError {}

impl Display for ImpossiblePuzzleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "puzzle is impossible (no words exist in dictionary that match current knowledge state)")
    }
}

impl Error for ImpossiblePuzzleError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
