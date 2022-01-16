use super::game::LetterStatus;
use fancy_regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

pub mod strategies;

pub struct Player<T>
where
    T: Fn(HashSet<String>) -> String,
{
    state: Vec<Option<char>>,
    off_limit: HashSet<char>,
    must_include: HashMap<char, Vec<usize>>,
    strategy: T,
}

impl<T> Player<T>
where
    T: Fn(HashSet<String>) -> String,
{
    pub fn new(word_len: usize, strategy: T) -> Self {
        Player {
            state: vec![None; word_len],
            off_limit: HashSet::new(),
            must_include: HashMap::new(),
            strategy: strategy,
        }
    }

    pub fn from(
        state: Vec<Option<char>>,
        off_limit: HashSet<char>,
        must_include: HashMap<char, Vec<usize>>,
        strategy: T,
    ) -> Self {
        Player {
            state,
            off_limit,
            must_include,
            strategy,
        }
    }

    pub fn guess(&self) -> String {
        (self.strategy)(word_options(
            &self.state,
            &self.off_limit,
            &self.must_include,
        ))
    }

    // fn update_knowledge(&mut self, from: Vec<LetterStatus>) {
    //     for (i, letter_status) in from.iter().enumerate() {
    //         match letter_status {
    //             LetterStatus::Correct(l) => {
    //                 std::mem::replace(&mut self.state[i], Some(*l));
    //             }
    //             LetterStatus::InDifferentPosition(l) => {
    //                 let s = self.must_include.get(l);
    //                 match s {
    //                     Some(idxs) => {
    //                         idxs.push(i);
    //                     }
    //                     None => {
    //                         self.must_include.insert(*l, vec![i]);
    //                     }
    //                 }
    //             }
    //             LetterStatus::NotInWord(l) => {
    //                 self.off_limit.insert(*l);
    //             }
    //         }
    //     }
    // }
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
                .map(|c| format!("{}{}", c.to_ascii_lowercase(), c.to_ascii_uppercase()))
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn regex_first_guess() {
        let state = vec![None, None, None, None, None];
        let off_limit = HashSet::new();
        let must_include = HashMap::new();
        let regex_exp = build_regex_query(&state, &off_limit, &must_include);
        let re = Regex::new(&regex_exp).expect("regex expression failed to compile");

        assert!(re.is_match("alone").unwrap(), "did not match 'alone'");
        assert!(re.is_match("ALONE").unwrap(), "did not match 'ALONE'");
        assert!(re.is_match("aLoNE").unwrap(), "did not match 'aLoNE'");
        assert!(
            !re.is_match("aaaa").unwrap(),
            "matched a string that was too short"
        );
        assert!(
            !re.is_match("aaaaaa").unwrap(),
            "matched a string that was too long"
        );
    }

    #[test]
    fn regex_off_limits() {
        let state = vec![None, None, None, None, None];
        let off_limit = HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i']);
        let must_include = HashMap::new();
        let regex_exp = build_regex_query(&state, &off_limit, &must_include);
        let re = Regex::new(&regex_exp).expect("regex expression failed to compile");

        assert!(re.is_match("zzzzz").unwrap(), "did not match 'zzzzz'");
        assert!(
            !re.is_match("azzzz").unwrap(),
            "incorrectly matched 'azzzz'"
        );
        assert!(
            !re.is_match("zzzza").unwrap(),
            "incorrectly matched 'zzzza'"
        );
        assert!(
            !re.is_match("abcde").unwrap(),
            "incorrectly matched 'abcde'"
        );
        assert!(!re.is_match("z").unwrap(), "incorrectly matched 'z'");
        assert!(
            !re.is_match("zzzzzz").unwrap(),
            "incorrectly matched 'zzzzzz'"
        );
    }

    #[test]
    fn regex_must_include() {
        let state = vec![None, None, None, None, None];
        let off_limit = HashSet::new();
        let must_include = HashMap::from([('r', vec![]), ('y', vec![])]);
        let regex_exp = build_regex_query(&state, &off_limit, &must_include);
        let re = Regex::new(&regex_exp).expect("regex expression failed to compile");

        assert!(re.is_match("rusty").unwrap(), "did not match 'rusty'");
        assert!(re.is_match("weary").unwrap(), "did not match 'weary'");
        assert!(
            !re.is_match("sorts").unwrap(),
            "incorrectly matched 'sorts'"
        );
    }

    #[test]
    fn regex_heavy_restriction() {
        let state = vec![None, None, None, None, None];
        let off_limit = HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i']);
        let must_include = HashMap::from([('r', vec![]), ('y', vec![])]);
        let regex_exp = build_regex_query(&state, &off_limit, &must_include);
        let re = Regex::new(&regex_exp).expect("regex expression failed to compile");

        assert!(re.is_match("rusty").unwrap(), "did not match 'rusty'");
        assert!(
            !re.is_match("rusts").unwrap(),
            "incorrectly matched 'rusts'"
        );
        assert!(
            !re.is_match("rasty").unwrap(),
            "incorrectly matched 'rasty'"
        );
    }

    #[test]
    fn regex_known_letters() {
        let state = vec![None, Some('a'), Some('n'), None, Some('y')];
        let off_limit = HashSet::from(['l', 'o', 'e', 'd', 'r', 'i', 'o', 'd', 'l']);
        let must_include = HashMap::from([('a', vec![0, 2]), ('n', vec![3, 4])]);
        let regex_exp = build_regex_query(&state, &off_limit, &must_include);
        let re = Regex::new(&regex_exp).expect("regex expression failed to compile");

        assert!(
            !re.is_match("alone").unwrap(),
            "incorrectly matched 'alone'"
        );
        assert!(
            !re.is_match("drain").unwrap(),
            "incorrectly matched 'drain'"
        );
        assert!(re.is_match("wanky").unwrap(), "did not match 'wanky'");
        assert!(re.is_match("tangy").unwrap(), "did not match 'tangy'");
    }
}
