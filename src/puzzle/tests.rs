use crate::player::strategies;

use super::*;

#[test]
fn correct_answer_wins() {
    let mut player = Player::new(4, strategies::user_input);
    let mut puzzle = Puzzle::new(&mut player, "easy", 5);

    assert!(matches!(
        puzzle.turn("easy".to_string()),
        GuessResult::Win { .. }
    ));
    assert!(matches!(
        puzzle.turn("EASY".to_string()),
        GuessResult::Win { .. }
    ));
    assert!(matches!(
        puzzle.turn("eAsY".to_string()),
        GuessResult::Win { .. }
    ));
}

#[test]
fn out_of_turns_loses() {
    let mut player = Player::new(4, strategies::user_input);
    let mut puzzle = Puzzle::new(&mut player, "easy", 1);

    let expected_result = GuessResult::Loss(IntermediateLetterInfo(vec![
        LetterStatus::NotInWord('h'),
        LetterStatus::Correct('a'),
        LetterStatus::NotInWord('r'),
        LetterStatus::NotInWord('d'),
    ]));

    assert_eq!(puzzle.turn("hard".to_string()), expected_result);
}

#[test]
fn intermediate_info_correct() {
    let mut player = Player::new(4, strategies::user_input);
    let mut puzzle = Puzzle::new(&mut player, "easy", 5);

    assert_eq!(
        puzzle.turn("east".to_string()),
        GuessResult::Continue(IntermediateLetterInfo(vec![
            LetterStatus::Correct('e'),
            LetterStatus::Correct('a'),
            LetterStatus::Correct('s'),
            LetterStatus::NotInWord('t')
        ]))
    );
}

#[test]
fn intermediate_info_correct_answer_duplicate_letters_first_right() {
    let mut player = Player::new(5, strategies::user_input);
    let mut puzzle = Puzzle::new(&mut player, "sweet", 5);

    assert_eq!(
        puzzle.turn("swept".to_string()),
        GuessResult::Continue(IntermediateLetterInfo(vec![
            LetterStatus::Correct('s'),
            LetterStatus::Correct('w'),
            LetterStatus::Correct('e'),
            LetterStatus::NotInWord('p'),
            LetterStatus::Correct('t')
        ]))
    );
}

#[test]
fn inter_info_correct_answer_duplicate_letters_first_wrong() {
    let mut player = Player::new(5, strategies::user_input);
    let mut puzzle = Puzzle::new(&mut player, "aalii", 5);

    assert_eq!(
        puzzle.turn("allis".to_string()),
        GuessResult::Continue(IntermediateLetterInfo(vec![
            LetterStatus::Correct('a'),
            LetterStatus::NotInWord('l'),
            LetterStatus::Correct('l'),
            LetterStatus::Correct('i'),
            LetterStatus::NotInWord('s')
        ]))
    );
}

#[test]
fn inter_info_correct_guess_duplicate_letters() {
    let mut player = Player::new(5, strategies::user_input);
    let mut puzzle = Puzzle::new(&mut player, "swept", 5);

    assert_eq!(
        puzzle.turn("sweet".to_string()),
        GuessResult::Continue(IntermediateLetterInfo(vec![
            LetterStatus::Correct('s'),
            LetterStatus::Correct('w'),
            LetterStatus::Correct('e'),
            LetterStatus::NotInWord('e'),
            LetterStatus::Correct('t')
        ]))
    );
}

#[test]
fn count_char_in_word_single() {
    assert_eq!(count_char_in_word('a', "maybe"), 1);
}

#[test]
fn count_char_in_word_none() {
    assert_eq!(count_char_in_word('a', "nope"), 0);
}

#[test]
fn count_char_in_word_many() {
    assert_eq!(count_char_in_word('e', "tennessee"), 4);
}

#[test]
fn char_occurance_in_word_single() {
    assert_eq!(letter_occurance_in_word(1, "maybe"), 1);
}

#[test]
fn char_occurance_in_word_many_first() {
    assert_eq!(letter_occurance_in_word(1, "tennessee"), 1);
}

#[test]
fn char_occurance_in_word_many_mid() {
    assert_eq!(letter_occurance_in_word(7, "tennessee"), 3);
}
