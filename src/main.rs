use game::Game;
use player::{strategies, Player};
use std::collections::{HashMap, HashSet};

mod game;
mod player;

fn main() {
    // let state = vec![None, None, None, None, None];
    // let off_limit = HashSet::from(['z', 'q']);
    // let must_include = HashMap::from([('a', vec![0, 2])]);

    let player = Player::new(5, strategies::random);
    let mut game = Game::new(player, "audio", 6);
    game.play();

    // println!("{}", player.guess());
}
