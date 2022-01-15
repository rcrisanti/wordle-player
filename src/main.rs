use std::collections::{HashMap, HashSet};

use guess::Guess;

mod guess;

fn main() {
    let state = vec![None, None, None, None, None];
    let off_limit = HashSet::from(['z', 'q']);
    let must_include = HashMap::from([('a', vec![0, 2])]);
    // let off_limit = HashSet::new();
    // let must_include = HashMap::new();
    let guess = Guess::new(state, off_limit, must_include);

    println!("{}", guess.regex_query());

    let options = guess.word_options();

    println!("{}", options.len());
    println!("{:?}", options);
}
