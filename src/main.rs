use clap::Parser;
use player::{strategies, Player};
use puzzle::Puzzle;

mod player;
mod puzzle;
mod errors;

#[derive(Parser)]
#[clap(author = "Ryan Crisanti", version, about, long_about = None)]
struct Args {
    #[clap(help = "The answer to the Wordle puzzle")]
    answer: String,

    #[clap(
        short,
        long,
        default_value_t = 6,
        help = "The number of turns given to solve the puzzle"
    )]
    n_turns: u8,

    #[clap(
        short, 
        long, 
        possible_values = ["optimized", "random", "user"], 
        default_value = "optimized", 
        help = "Which player is solving the puzzle", 
        long_help = concat!(
            "The optimized player uses a custom heuristic to pick the best guess. The ",
            "random player picks a random word from the bag of valid words. The user ",
            "player allows you to play!"
        )
    )]
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
    let solved = puzzle.solve();

    if let Err(err) = solved {
        eprintln!("\"{}\" {}", answer, err);
    }
}
