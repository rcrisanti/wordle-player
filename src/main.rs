use colored::Colorize;
use game::Game;
use player::{strategies, Player};

mod game;
mod player;

fn main() {
    let player = Player::new(5, strategies::random);
    let mut game = Game::new(player, "audio", 6);
    game.play();

    // let word = vec!["h", "e", "l", "l", "o"];
    // println!(
    //     "{}{}{}",
    //     "hello"
    //         .to_ascii_uppercase()
    //         .on_truecolor(181, 159, 58)
    //         .truecolor(215, 218, 220)
    //         .bold(),
    //     "there"
    //         .to_ascii_uppercase()
    //         .on_truecolor(83, 141, 78)
    //         .truecolor(215, 218, 220)
    //         .bold(),
    //     "world"
    //         .to_ascii_uppercase()
    //         .on_truecolor(58, 58, 60)
    //         .truecolor(215, 218, 220)
    //         .bold()
    // );
}
