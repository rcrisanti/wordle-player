use rand::Rng;
use std::cmp::Ordering::Equal;
use std::collections::{HashMap, HashSet};
use std::io;

pub fn user_input(
    words: &HashSet<String>,
    _turn_perc: f32,
    _current_state: &Vec<Option<char>>,
    _word_frequencies: &HashMap<String, f32>,
) -> String {
    let mut guess = String::new();
    println!("Please input your guess:");
    io::stdin()
        .read_line(&mut guess)
        .expect("could not read line")
        .to_string();

    guess = guess.trim().to_ascii_lowercase();

    if words.contains(&guess) {
        guess
    } else {
        println!(
            "Your guess '{}' does not exist in our dictionary. Try again!",
            guess
        );
        user_input(words, _turn_perc, _current_state, _word_frequencies)
    }
}

pub fn random(
    words: &HashSet<String>,
    _turn_perc: f32,
    _current_state: &Vec<Option<char>>,
    _word_frequencies: &HashMap<String, f32>,
) -> String {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..words.len());
    match words.iter().collect::<Vec<_>>().get(index) {
        Some(word) => word.to_string(),
        None => "words".to_string(),
    }
}

pub fn word_letter_commonality(
    words: &HashSet<String>,
    turn_perc: f32,
    current_state: &Vec<Option<char>>,
    word_frequencies: &HashMap<String, f32>,
) -> String {
    let lambda_min = 0.2;
    let lambda = lambda_min + (1. - lambda_min) * turn_perc;

    let letter_frequencies_bytes = include_bytes!("../../letter-frequencies.txt");
    let file = String::from_utf8_lossy(letter_frequencies_bytes);
    let letter_freqs = file
        .split("\n")
        .map(|line| {
            let (letter, freq) = line.split_at(line.find(',').expect("did not finc comma in line"));
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
            let heuristic_a = word_letter_commonality_heuristic(
                a,
                lambda,
                &current_state,
                &letter_freqs,
                word_frequencies.get(*a).unwrap_or(&0.5),
            );
            let heuristic_b = word_letter_commonality_heuristic(
                b,
                lambda,
                &current_state,
                &letter_freqs,
                word_frequencies.get(*b).unwrap_or(&0.5),
            );
            heuristic_a.partial_cmp(&heuristic_b).unwrap_or(Equal)
        })
        .expect("could not find a best word")
        .to_string()
}

fn word_letter_commonality_heuristic(
    word: &String,
    lambda: f32,
    current_state: &Vec<Option<char>>,
    letter_frequencies: &HashMap<char, f32>,
    word_frequency: &f32,
) -> f32 {
    let letters_term: f32 = word
        .to_ascii_lowercase()
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
        .sum();
    letters_term + lambda * word_frequency
}
