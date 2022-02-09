use statrs::statistics::{Data, Distribution, OrderStatistics};
use std::collections::HashSet;

use crate::{
    player::{
        strategies::{LetterFrequencyStrategy, RandomStrategy, Strategy},
        Player,
    },
    puzzle::{guess_result::GuessResult, Puzzle},
};

#[test]
fn random_from_list_of_one() {
    let words = HashSet::from(["hello".to_string()]);
    let strategy = RandomStrategy::new();
    let guess = strategy.best_word(&words, &vec![]);
    assert_eq!(guess, "hello".to_string());
}

#[test]
fn random_is_one_of_choices() {
    let words = HashSet::from([
        "i".to_string(),
        "will".to_string(),
        "master".to_string(),
        "wordle".to_string(),
    ]);
    let strategy = RandomStrategy::new();
    let guess = strategy.best_word(&words, &vec![]);
    assert!(words.contains(&guess));
}

#[test]
#[ignore]
fn heuristic_better_than_random() {
    let words = HashSet::from([
        "shire", "proxy", "point", "robot", "prick", "wince", "crimp", "knoll", "sugar", "whack",
        "mount", "perky", "could",
    ]); // actual wordle words

    let word_length = 5;
    let max_n_turns = 20;

    let mut random_turns = Data::new(
        words
            .iter()
            .map(|answer| {
                let mut random_player = Player::new(word_length, Box::new(RandomStrategy::new()));
                let mut puzzle = Puzzle::new(&mut random_player, &answer, max_n_turns);
                let solved = puzzle.solve();
                match solved {
                    Ok(res) => match res {
                        GuessResult::Win(_, n_turns) => n_turns as f64,
                        _ => panic!(
                            "random solver could not guess '{}' in {} turns",
                            answer, max_n_turns
                        ),
                    },
                    Err(err) => panic!("\"{}\" (random) {}", answer, err),
                }
            })
            .collect::<Vec<_>>(),
    );

    let mut heuristic_turns = Data::new(
        words
            .iter()
            .map(|answer| {
                let mut heuristic_player =
                    Player::new(word_length, Box::new(LetterFrequencyStrategy::new()));
                let mut puzzle = Puzzle::new(&mut heuristic_player, &answer, max_n_turns);
                let solved = puzzle.solve();
                match solved {
                    Ok(res) => match res {
                        GuessResult::Win(_, n_turns) => n_turns as f64,
                        _ => panic!(
                            "heuristic solver could not guess '{}' in {} turns",
                            answer, max_n_turns
                        ),
                    },
                    Err(err) => panic!("\"{}\" (heuristic) {}", answer, err),
                }
            })
            .collect::<Vec<_>>(),
    );

    // check mean, percentiles, etc. are better
    assert!(
        heuristic_turns
            .mean()
            .expect("could not calculate mean for heuristic")
            < random_turns
                .mean()
                .expect("could not calculate mean for random"),
        "Mean turns for heuristic ({}) not better than random ({})",
        heuristic_turns
            .mean()
            .expect("could not calculate mean for heuristic"),
        random_turns
            .mean()
            .expect("could not calculate mean for random"),
    );

    assert!(
        heuristic_turns.percentile(95) < random_turns.percentile(95),
        "95th percentile number of turns for heuristic ({}) not better than random ({})",
        heuristic_turns.percentile(95),
        random_turns.percentile(95)
    );
}
