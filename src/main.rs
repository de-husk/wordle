use std::io;

//const WORD_LENGTH = 5;

fn main() {
    println!("Type word to play");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); // TODO: Handle err

    let correct = check_answer(input);

    println!("{:?}", correct);

    // TODO:
    // * Add 5 tries with an ultimate winning / losing condition
    // * Add basic UI
}

#[derive(Debug)]
enum Score {
    FullyCorrect, // Green
    CorrectChar,  // Yellow
    NotFound,     // Gray
}

#[derive(Debug)]
struct WordScore {
    letter_scores: Vec<Score>,
}

fn check_answer(guess: String) -> WordScore {
    // TODO: Stop hardcoding answer and use a random word from a dictionary of common words
    let answer = "perch";

    // TODO: Implement
    WordScore {
        letter_scores: vec![
            Score::FullyCorrect,
            Score::FullyCorrect,
            Score::FullyCorrect,
            Score::FullyCorrect,
            Score::FullyCorrect,
        ],
    }
}
