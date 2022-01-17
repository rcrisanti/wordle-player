use player::{strategies, Player};
use puzzle::Puzzle;

mod player;
mod puzzle;

fn main() {
    let player = Player::new(5, strategies::word_letter_commonality);
    let mut puzzle = Puzzle::new(player, "shire", 6);
    puzzle.solve();
}
