use std::io;

//const WORD_LENGTH = 5;

fn main() {
    let mut attempt_count = 0;
    let max_guesses = 6;
    let mut correct = false;

    while attempt_count < max_guesses && !correct {
        println!("Type word to play: {}", attempt_count);

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap(); // TODO: Handle err
        correct = check_answer(input);

        attempt_count += 1;
    }

    if attempt_count >= max_guesses {
        println!("You lose :(");
    } else {
        println!("You win!");
    }

    // TODO:
    // * Add basic UI
}

fn check_answer(s: String) -> bool {
    let answer = "heart".to_string();
    let s = s.trim();

    return answer == s;
}
