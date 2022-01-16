use rand::Rng;
use std::collections::HashSet;
use std::io;

pub fn user_input(words: &HashSet<String>) -> String {
    let mut guess = String::new();
    println!("Please input your guess:");
    io::stdin()
        .read_line(&mut guess)
        .expect("could not read line")
        .to_string();

    guess = guess.trim().to_ascii_lowercase();

    if words.contains(&guess) {
        guess
    } else {
        println!(
            "Your guess '{}' does not exist in our dictionary. Try again!",
            guess
        );
        user_input(words)
    }
}

pub fn random(words: &HashSet<String>) -> String {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..words.len());
    match words.iter().collect::<Vec<_>>().get(index) {
        Some(word) => word.to_string(),
        None => "words".to_string(),
    }
}
