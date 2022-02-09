use super::puzzle::guess_result::LetterStatus;
use crate::errors::ImpossiblePuzzleError;
use fancy_regex::Regex;
use std::collections::{HashMap, HashSet};
use strategies::Strategy;

pub mod strategies;

#[cfg(test)]
mod tests;

pub struct Player {
    state: Vec<Option<char>>,
    off_limit: HashSet<char>,
    must_include: HashMap<char, Vec<usize>>,
    strategy: Box<dyn Strategy>,
}

impl Player {
    pub fn new(word_len: usize, strategy: Box<dyn Strategy>) -> Self {
        Player {
            state: vec![None; word_len],
            off_limit: HashSet::new(),
            must_include: HashMap::new(),
            strategy,
        }
    }

    pub fn guess(&mut self) -> Result<String, ImpossiblePuzzleError> {
        let words = word_options(&self.state, &self.off_limit, &self.must_include);

        match words.len() {
            0 => Err(ImpossiblePuzzleError {}),
            _ => Ok(self.strategy.best_word(&words, &self.state)),
        }
    }

    pub fn update_knowledge(&mut self, guess_results: Vec<LetterStatus>) {
        for (i, letter_status) in guess_results.iter().enumerate() {
            match letter_status {
                LetterStatus::Correct(l) => {
                    let _ = std::mem::replace(&mut self.state[i], Some(*l));
                }
                LetterStatus::Misplaced(l) => {
                    let l_not_pos = self.must_include.get_mut(l);
                    match l_not_pos {
                        Some(idxs) => {
                            idxs.push(i);
                        }
                        None => {
                            self.must_include.insert(*l, vec![i]);
                        }
                    }
                }
                LetterStatus::Incorrect(l) => {
                    self.off_limit.insert(*l);
                }
            }
        }
    }
}

fn word_options(
    state: &Vec<Option<char>>,
    off_limit: &HashSet<char>,
    must_include: &HashMap<char, Vec<usize>>,
) -> HashSet<String> {
    // for now using a static file (read this way so that file is included in compiled executable)
    let words_bytes = include_bytes!("../word-database.txt");
    let file = String::from_utf8_lossy(words_bytes);
    let lines = file.split("\n").map(|w| w.to_string());

    let regex_query = build_regex_query(state, off_limit, must_include);
    let re = Regex::new(&regex_query).expect("regex expression failed to compile");

    lines
        .filter_map(|word| {
            if re.is_match(&word).unwrap() {
                return Some(word);
            }
            None
        })
        .collect::<HashSet<_>>()
}

fn build_regex_query(
    state: &Vec<Option<char>>,
    off_limit: &HashSet<char>,
    must_include: &HashMap<char, Vec<usize>>,
) -> String {
    // Negative lookahead to apply to every character (characters you know are not included)
    let neg = match off_limit.len() {
        0 => None,
        _ => Some(
            off_limit
                .iter()
                .map(|c| {
                    if must_include.contains_key(&c) {
                        "".to_string()
                    } else {
                        format!("{}{}", c.to_ascii_lowercase(), c.to_ascii_uppercase())
                    }
                })
                .collect::<String>(),
        ),
    };
    let neg_lookahead = match neg {
        Some(q) => format!("(?![{}])", q),
        None => "".to_string(),
    };

    // Postive lookaheads to apply to whole word (characters you know are included somewhere but don't know where)
    let pos_lookahead = match must_include.len() {
        0 => "".to_string(),
        _ => must_include
            .keys()
            .map(|c| {
                format!(
                    "(?=.*[{}|{}])",
                    c.to_ascii_lowercase(),
                    c.to_ascii_uppercase()
                )
            })
            .collect::<String>(),
    };

    // No build up the whole search expression
    let match_expr: String = state
        .iter()
        .enumerate()
        .map(|(i, c)| match c {
            Some(l) => format!("[{}|{}]", l.to_ascii_lowercase(), l.to_ascii_uppercase()),
            None => {
                let local_non_chars = must_include
                    .iter()
                    .filter_map(|(l, idxs)| {
                        if idxs.contains(&i) {
                            Some(format!(
                                "{}{}",
                                l.to_ascii_lowercase(),
                                l.to_ascii_uppercase()
                            ))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                let local_neg_lookahead = match local_non_chars.len() {
                    0 => "".to_string(),
                    _ => format!("(?![{}])", local_non_chars.join("")),
                };

                format!("{}{}[A-Z|a-z]", neg_lookahead, local_neg_lookahead)
            }
        })
        .collect();

    format!("^{}({})$", pos_lookahead, match_expr)
}
