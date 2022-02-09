use clap::Parser;
use player::strategies::{LetterFrequencyStrategy, RandomStrategy, UserInputStrategy};
use player::Player;
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
        possible_values = ["letter-freq", "random", "user"], 
        default_value = "letter-freq", 
        help = "Which player is solving the puzzle", 
        long_help = concat!(
            "The letter-freq strategy uses a heuristic based on the frequency of each ",
            "letter in the English language to pick the best guess. The random ",
            "strategy picks a random word from the bag of valid words. The user ",
            "strategy allows you to play!"
        )
    )]
    strategy: String,

    #[clap(short, long, help = "Whether to allow guesses that violate current knowledge state")]
    easy_mode: bool
}

fn main() {
    let args = Args::parse();
    let answer = args.answer.to_ascii_lowercase();

    let mut player = Player::new(
        answer.len(),
        match &args.strategy.as_str() {
            &"random" => Box::new(RandomStrategy::new()),
            &"user" => Box::new(UserInputStrategy::new()),
            _ => Box::new(LetterFrequencyStrategy::new()),
        },
        !args.easy_mode
    );
    let mut puzzle = Puzzle::new(&mut player, &answer, args.n_turns);
    let solved = puzzle.solve();

    if let Err(err) = solved {
        eprintln!("\"{}\" {}", answer, err);
    }
}
