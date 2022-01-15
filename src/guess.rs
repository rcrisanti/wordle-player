use fancy_regex::Regex;
use rand::Rng;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

#[cfg(test)]
mod test;

pub struct Guess {
    state: Vec<Option<char>>,
    off_limit: HashSet<char>,
    must_include: HashMap<char, Vec<usize>>,
}

// constructor
impl Guess {
    pub fn new(
        state: Vec<Option<char>>,
        off_limit: HashSet<char>,
        must_include: HashMap<char, Vec<usize>>,
    ) -> Self {
        Guess {
            state,
            off_limit,
            must_include,
        }
    }
}

// create a guess
impl Guess {
    pub fn regex_query(&self) -> String {
        // Negative lookahead to apply to every character (characters you know are not included)
        let neg = match self.off_limit.len() {
            0 => None,
            _ => Some(
                self.off_limit
                    .iter()
                    .map(|c| format!("{}{}", c.to_ascii_lowercase(), c.to_ascii_uppercase()))
                    .collect::<String>(),
            ),
        };
        let neg_lookahead = match neg {
            Some(q) => format!("(?![{}])", q),
            None => "".to_string(),
        };

        // Postive lookaheads to apply to whole word (characters you know are included somewhere but don't know where)
        let pos_lookahead = match self.must_include.len() {
            0 => "".to_string(),
            _ => self
                .must_include
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
        let match_expr: String = self
            .state
            .iter()
            .enumerate()
            .map(|(i, c)| match c {
                Some(l) => format!("[{}|{}]", l.to_ascii_lowercase(), l.to_ascii_uppercase()),
                None => {
                    let local_non_chars = self
                        .must_include
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

    pub fn word_options(&self) -> Vec<String> {
        // for now using a static file
        let word_db = File::open("word-database.txt").expect("could not open word database file");
        let lines = BufReader::new(word_db).lines();

        let re = Regex::new(&self.regex_query()).expect("regex expression failed to compile");

        lines
            .filter_map(|line| {
                if let Ok(word) = line {
                    if re.is_match(&word).unwrap() {
                        return Some(word);
                    }
                }
                None
            })
            .collect::<Vec<_>>()
    }

    // pub fn guess(&self) -> &str {
    //     // for now, pick a random available word
    //     let options = self.word_options();
    //     let mut rng = rand::thread_rng();
    //     let index = rng.gen_range(0..options.len());
    //     options[index]
    // }
}
