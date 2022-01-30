use super::puzzle::guess_result::LetterStatus;
use crate::errors::ImpossiblePuzzleError;
use fancy_regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

pub mod strategies;

#[cfg(test)]
mod tests;

pub struct Player<T>
where
    T: Fn(&HashSet<String>, f32, &Vec<Option<char>>, &HashMap<String, f32>) -> String,
{
    state: Vec<Option<char>>,
    off_limit: HashSet<char>,
    must_include: HashMap<char, Vec<usize>>,
    strategy: T,
    n_turns: Option<u8>,
    completed_turns: u8,
}

impl<T> Player<T>
where
    T: Fn(&HashSet<String>, f32, &Vec<Option<char>>, &HashMap<String, f32>) -> String,
{
    pub fn new(word_len: usize, strategy: T) -> Self {
        Player {
            state: vec![None; word_len],
            off_limit: HashSet::new(),
            must_include: HashMap::new(),
            strategy,
            n_turns: None,
            completed_turns: 0,
        }
    }

    // pub fn from(
    //     state: Vec<Option<char>>,
    //     off_limit: HashSet<char>,
    //     must_include: HashMap<char, Vec<usize>>,
    //     strategy: T,
    //     n_turns: Option<u8>,
    //     completed_turns: u8,
    // ) -> Self {
    //     Player {
    //         state,
    //         off_limit,
    //         must_include,
    //         strategy,
    //         n_turns,
    //         completed_turns,
    //     }
    // }

    pub fn guess(&mut self) -> Result<String, ImpossiblePuzzleError> {
        self.completed_turns += 1;
        let turn_perc = match self.n_turns {
            Some(n) => self.completed_turns as f32 / n as f32,
            None => 0.5,
        };

        let words = word_options(&self.state, &self.off_limit, &self.must_include);

        match words.len() {
            0 => Err(ImpossiblePuzzleError {}),
            _ => Ok((self.strategy)(
                &words,
                turn_perc,
                &self.state,
                &HashMap::new(),
            )),
        }
    }

    pub fn set_puzzle_rules(&mut self, n_turns: u8) {
        self.n_turns = Some(n_turns);
    }

    pub fn update_knowledge(&mut self, guess_results: Vec<LetterStatus>) {
        for (i, letter_status) in guess_results.iter().enumerate() {
            match letter_status {
                LetterStatus::Correct(l) => {
                    let _ = std::mem::replace(&mut self.state[i], Some(*l));
                }
                LetterStatus::InDifferentPosition(l) => {
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
                LetterStatus::NotInWord(l) => {
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
    // for now using a static file
    let word_db = File::open("word-database.txt").expect("could not open word database file");
    let lines = BufReader::new(word_db).lines();

    let regex_query = build_regex_query(state, off_limit, must_include);
    let re = Regex::new(&regex_query).expect("regex expression failed to compile");

    lines
        .filter_map(|line| {
            if let Ok(word) = line {
                if re.is_match(&word).unwrap() {
                    return Some(word);
                }
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
