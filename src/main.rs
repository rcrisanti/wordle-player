use clap::Parser;
use player::{strategies, Player};
use puzzle::Puzzle;

mod player;
mod puzzle;

#[derive(Parser)]
#[clap(author = "Ryan Crisanti", version, about, long_about = None)]
struct Args {
    answer: String,

    #[clap(short, long, default_value_t = 6)]
    n_turns: u8,

    #[clap(short, long, possible_values=["optimized", "random", "user"], default_value="optimized")]
    player: String,
}

fn main() {
    let args = Args::parse();
    let answer = args.answer.to_ascii_lowercase();

    let mut player = Player::new(
        answer.len(),
        match &args.player.as_str() {
            &"random" => strategies::random,
            &"user" => strategies::user_input,
            _ => strategies::word_letter_commonality,
        },
    );
    let mut puzzle = Puzzle::new(&mut player, &answer, args.n_turns);
    puzzle.solve();
}
