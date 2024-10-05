mod creatures;

use creatures::{
    convert_name_to_guess_format, fetch_puzzles, is_question_winning, parse_creatures,
    update_question, Puzzle, Puzzles,
};
use rand::seq::SliceRandom;
use std::{collections::HashSet, io, process};

struct Game {
    question: String,
    letters_guessed: String,
    health: usize,
}

// fn select_random_puzzle(puzzles: &Vec<Puzzle>) -> Option<&Puzzle> {
//     if puzzles.is_empty() {
//         return None; // If the Vec is empty, return None
//     }

//     let mut rng = rand::thread_rng(); // Create a random number generator
//     let random_index = rng.gen_range(0..puzzles.len()); // Generate a random index within the vector bounds
//     Some(&puzzles[random_index]) // Return the puzzle at the random index
// }

fn main() -> io::Result<()> {
    println!("Little Creatures!\nType 'QUIT' at any time to leave the game");
    let tokens = parse_creatures();
    // println!("{tokens:?}");

    let puzzles = fetch_puzzles(tokens);
    // println!("{puzzles:#?}");
    let num_buckets = 10;
    let chunk_size = (puzzles.len() + num_buckets - 1) / num_buckets;
    let puzzle_buckets: Vec<&[Puzzle]> = puzzles.chunks(chunk_size).collect();

    let mut selected_puzzles: Puzzles = Puzzles::new();

    for bucket in puzzle_buckets {
        let p = bucket.choose(&mut rand::thread_rng()).unwrap();
        selected_puzzles.push(p.clone());
    }

    let mut game: Game = Game {
        question: String::from(""),
        letters_guessed: String::from(""),
        health: 100,
    };

    for puzzle in selected_puzzles {
        // Setup
        game.question = convert_name_to_guess_format(&puzzle.creature);
        game.letters_guessed = String::from("");
        loop {
            let h_st = match puzzle.hint.as_str() {
                "" => "".to_string(),                             // Return an empty String
                _ => "Hint: ".to_string() + puzzle.hint.as_str(), // Convert "Hint" to a String, then concatenate
            };
            // Render the puzzle
            println!("???: {} {}", game.question, h_st);
            // println!("ans: {}", puzzle.creature);
            println!(
                "Health: {} / Letters Remaining: {}",
                game.health,
                find_unused_letters(&game.letters_guessed)
            );

            // Get User input and check if its a good guess
            let guess: String = get_user_character(&game.letters_guessed);

            let is_correct_guess =
                puzzle.creature.contains(&guess) && !game.question.contains(&guess);
            // Add a letter that you guessed
            game.letters_guessed.push_str(&guess);
            // game.letters_guessed = sort_string_alphabetically(&game.letters_guessed);
            if is_correct_guess {
                println!("{guess} was CORRECT!");
                // Update the question to show your progress
                game.question = update_question(&puzzle.creature, &game.question, &guess);
            } else {
                game.health -= 1;
            }

            // Check if you are a winner of the puzzle
            let is_winning_question = is_question_winning(&game.question);

            // You won a puzzle, and solved the hangman
            if is_winning_question {
                println!("Great job, the answer was indeed {}\n\n", puzzle.creature);
                game.health += 3;
                game.health = game.health.min(100);
                break;
            }

            if game.health < 1 {
                panic!("You lost the game, shitheel!");
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

        if input.contains("QUIT") {
            process::exit(0);
        }
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

fn sort_string_alphabetically(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect(); // Convert string to vector of characters
    chars.sort(); // Sort the characters
    chars.into_iter().collect() // Collect the sorted characters back into a string
}

fn find_unused_letters(used: &str) -> String {
    // Create a set of all capitalized letters A-Z
    let all_letters: HashSet<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();

    // Create a set of the used letters from the input string
    let used_letters: HashSet<char> = used.chars().collect();

    // Find the unused letters by subtracting used from all
    let unused_letters: HashSet<char> = all_letters.difference(&used_letters).cloned().collect();

    // Sort the remaining unused letters and collect them into a String
    let mut unused_vec: Vec<char> = unused_letters.into_iter().collect();
    unused_vec.sort();

    unused_vec.iter().collect() // Collect the sorted characters back into a string
}
