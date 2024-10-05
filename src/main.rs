mod creatures;

use creatures::{
    convert_name_to_guess_format, fetch_puzzles, is_question_winning, parse_creatures,
    update_question,
};
use std::io;

struct Game {
    question: String,
    letters_guessed: String,
}

fn main() -> io::Result<()> {
    let tokens = parse_creatures();
    // println!("{tokens:?}");

    let puzzles = fetch_puzzles(tokens);
    // println!("{puzzles:#?}");

    let mut game: Game = Game {
        question: String::from(""),
        letters_guessed: String::from(""),
    };

    for puzzle in puzzles {
        // Setup
        game.question = convert_name_to_guess_format(&puzzle.creature);
        game.letters_guessed = String::from("");
        loop {
            // Render the puzzle
            println!("???: {}", game.question);
            println!("Letters Guessed: {}", game.letters_guessed);

            // Get User input and check if its a good guess
            let guess: String = get_user_character(&game.letters_guessed);

            let is_correct_guess =
                puzzle.creature.contains(&guess) && !game.question.contains(&guess);
            // Add a letter that you guessed
            game.letters_guessed.push_str(&guess); // sort this alpha
            if is_correct_guess {
                println!("Good Guess! {guess} is in there!");
                // Update the question to show your progress
                game.question = update_question(&puzzle.creature, &game.question, &guess);
            } else {
                println!("You lossed a life!!!");
            }

            // Check if you are a winner of the puzzle
            let is_winning_question = is_question_winning(&game.question);

            // You won a puzzle, and solved the hangman
            if is_winning_question {
                println!("Great job, the answer was indeed {}\n\n", puzzle.creature);
                break;
            }
        }
    }
    println!("You won the game!");
    Ok(())
}

fn get_user_character(letters_guessed: &str) -> String {
    let mut result = String::from("");
    let mut exit_loop = false;
    while !exit_loop {
        // Get User input ( a single character)
        let mut input = String::new();
        let stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut input).unwrap();
        input = input.trim().to_string().to_uppercase();

        if letters_guessed.contains(input.as_str()) {
            println!("You already guessed that letter, you silly billy goat Gus!");
        } else if input.len() == 1 && input.chars().all(|c| c.is_alphabetic()) {
            result = input.clone();
            exit_loop = true;
        } else {
            println!("Enter a single letter pls!");
        }
    }
    result
}
