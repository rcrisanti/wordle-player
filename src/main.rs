use player::{strategies, Player};
use puzzle::Puzzle;

mod player;
mod puzzle;

fn main() {
    let mut player = Player::new(5, strategies::word_letter_commonality);
    let mut puzzle = Puzzle::new(&mut player, "shire", 6);
    puzzle.solve();
}
