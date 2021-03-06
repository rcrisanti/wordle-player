use super::*;
use crate::player::strategies;

#[test]
fn none_in_word() {
    let mut player = Player::new(3, strategies::random);
    let guess_results = vec![
        LetterStatus::NotInWord('a'),
        LetterStatus::NotInWord('b'),
        LetterStatus::NotInWord('c'),
    ];
    player.update_knowledge(guess_results);

    assert_eq!(player.state, vec![None; 3]);
    assert_eq!(player.must_include, HashMap::new());
    assert_eq!(player.off_limit, HashSet::from(['a', 'b', 'c']));
}

#[test]
fn wrong_order() {
    let mut player = Player::new(3, strategies::random);
    let guess_results = vec![
        LetterStatus::InDifferentPosition('b'),
        LetterStatus::InDifferentPosition('a'),
        LetterStatus::InDifferentPosition('c'),
    ];
    player.update_knowledge(guess_results);

    assert_eq!(player.state, vec![None; 3]);
    assert_eq!(
        player.must_include,
        HashMap::from([('b', vec![0]), ('a', vec![1]), ('c', vec![2])])
    );
    assert_eq!(player.off_limit, HashSet::new());
}

#[test]
fn all_correct() {
    let mut player = Player::new(3, strategies::random);
    let guess_results = vec![
        LetterStatus::Correct('c'),
        LetterStatus::Correct('a'),
        LetterStatus::Correct('b'),
    ];
    player.update_knowledge(guess_results);

    assert_eq!(player.state, vec![Some('c'), Some('a'), Some('b')]);
    assert_eq!(player.must_include, HashMap::new());
    assert_eq!(player.off_limit, HashSet::new());
}

#[test]
fn mix() {
    let mut player = Player::new(3, strategies::random);
    let guess_results = vec![
        LetterStatus::Correct('c'),
        LetterStatus::InDifferentPosition('b'),
        LetterStatus::NotInWord('x'),
    ];
    player.update_knowledge(guess_results);

    assert_eq!(player.state, vec![Some('c'), None, None]);
    assert_eq!(player.must_include, HashMap::from([('b', vec![1])]));
    assert_eq!(player.off_limit, HashSet::from(['x']));
}
