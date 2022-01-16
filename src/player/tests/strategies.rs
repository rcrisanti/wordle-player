use std::collections::HashSet;

use crate::player::strategies;

#[test]
fn random_from_list_of_one() {
    let words = HashSet::from(["hello".to_string()]);
    let guess = strategies::random(&words);
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
    let guess = strategies::random(&words);
    assert!(words.contains(&guess));
}
