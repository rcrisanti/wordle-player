use std::cmp::Ordering::Equal;
use std::collections::HashMap;

use super::Strategy;

pub struct LetterFrequencyStrategy {}

impl LetterFrequencyStrategy {
    pub fn new() -> Self {
        LetterFrequencyStrategy {}
    }
}

impl LetterFrequencyStrategy {
    fn heuristic(
        &self,
        word: &String,
        current_state: &Vec<Option<char>>,
        letter_frequencies: &HashMap<char, f32>,
    ) -> f32 {
        word.to_ascii_lowercase()
            .chars()
            .map(|c| {
                if current_state.contains(&Some(c)) {
                    0f32
                } else {
                    let letter_freq = letter_frequencies.get(&c.to_ascii_lowercase());
                    let n_appearances = word
                        .to_ascii_lowercase()
                        .chars()
                        .filter(|l| l.to_owned() == c)
                        .count() as f32;
                    if let Some(lf) = letter_freq {
                        *lf / n_appearances
                    } else {
                        println!("warning: do not have letter frequency value for '{}'", c);
                        0.5 / n_appearances
                    }
                }
            })
            .sum()
    }
}

impl Strategy for LetterFrequencyStrategy {
    fn best_word(
        &self,
        words: &std::collections::HashSet<String>,
        current_state: &Vec<Option<char>>,
    ) -> String {
        let letter_frequencies_bytes = include_bytes!("../../../letter-frequencies.txt");
        let file = String::from_utf8_lossy(letter_frequencies_bytes);
        let letter_freqs = file
            .split("\n")
            .map(|line| {
                let (letter, freq) =
                    line.split_at(line.find(',').expect("did not finc comma in line"));
                (
                    letter
                        .to_ascii_lowercase()
                        .chars()
                        .next()
                        .expect("not at least 1 character in first column of letter freqs"),
                    freq[1..]
                        .parse::<f32>()
                        .expect("could not parse letter freq"),
                )
            })
            .collect::<HashMap<_, _>>();

        words
            .iter()
            .max_by(|a, b| {
                let heuristic_a = self.heuristic(a, &current_state, &letter_freqs);
                let heuristic_b = self.heuristic(b, &current_state, &letter_freqs);
                heuristic_a.partial_cmp(&heuristic_b).unwrap_or(Equal)
            })
            .expect("could not find a best word")
            .to_string()
    }
}
