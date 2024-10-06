mod creatures;

use creatures::{
    convert_name_to_guess_format, fetch_puzzles, is_question_winning, parse_creatures,
    update_question, Puzzle, Puzzles,
};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use rand::seq::SliceRandom;
use rodio::{Decoder, OutputStream, Sink, Source};
use std::io::Cursor;
use std::{collections::HashSet, io, process, thread};

struct Game {
    question: String,
    letters_guessed: String,
    health: usize,
    cash: usize,
}

// fn select_random_puzzle(puzzles: &Vec<Puzzle>) -> Option<&Puzzle> {
//     if puzzles.is_empty() {
//         return None; // If the Vec is empty, return None
//     }

//     let mut rng = rand::thread_rng(); // Create a random number generator
//     let random_index = rng.gen_range(0..puzzles.len()); // Generate a random index within the vector bounds
//     Some(&puzzles[random_index]) // Return the puzzle at the random index
// }

struct Paint {
    intro: String,
    status: String,
    answer_result: String,
}

impl Default for Paint {
    fn default() -> Self {
        Paint {
            intro: String::new(),
            status: String::new(),
            answer_result: String::new(),
        }
    }
}

fn just_play_some_tunes_bro() {
    // Spawn a new thread to play the music
    thread::spawn(|| {
        // Create an output stream
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        // Create a sink to manage audio playback
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Embed the audio file as bytes
        let music_data = include_bytes!("music.mp3");
        let cursor = Cursor::new(music_data);

        // Create a decoder from the embedded data
        let source = Decoder::new(cursor).unwrap();

        // Loop the audio indefinitely using the `repeat_infinite` method
        let repeated_source = source.repeat_infinite();

        // Play the audio asynchronously
        sink.append(repeated_source);

        // Detach the sink so that it plays the audio independently
        sink.detach();

        // Keep the thread alive to allow the music to continue playing
        loop {
            thread::park(); // Park the thread to keep it alive
        }
    });
}

fn main() -> io::Result<()> {
    // just_play_some_tunes_bro();

    let mut paint = Paint {
        intro: "
This game was built for the Ludum Dare 56 Solo Jam in October 2024
Author: Colton Phillips

Tiny Creatures Support Group!
============================

You find yourself at a support group for SMOL CREATURES! 
Someone forgot to bring any nametags so it's your job to
guess the names of each attendant one LETTER at a time!
Don't make too many mistakes or people will think you are
a bit of a narcicist. 

RULES:
=====
Guess the creature's name ONE letter at a time. 
Type 'QUIT' to leave at any time.

Press Enter to greet the first member
of the Tiny Creature Support Group (I hope they snacks!)
"
        .to_string(),
        ..Default::default()
    };

    // Force user to sit and read the story
    paint_terminal(&paint);
    wait_for_user_input();
    paint.intro = "".to_string();

    // Get this party started
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
        cash: 0,
    };

    for puzzle in selected_puzzles {
        // Setup
        game.question = convert_name_to_guess_format(&puzzle.creature);
        game.letters_guessed = String::from("");
        loop {
            // Render the puzzle and question

            paint.status = format!(
            "{}: '{}'\n\nYou: 'Heyyy...'\n\nMy thoughts: {}\n\nHealth: {}, Cash:{}, Unused Letters:{}\nEnter a Letter...",
                game.question,
                "Hey man",
                puzzle.hint,
                game.health,
                game.cash,
                find_unused_letters(&game.letters_guessed),
            );

            let mut guess = String::from("");
            let mut exit_loop = false;
            {
                // This block handles user input
                while !exit_loop {
                    paint_terminal(&paint);
                    // Get User input ( a single character)
                    let mut input = String::new();
                    let stdin = io::stdin(); // We get `Stdin` here.
                    stdin.read_line(&mut input).unwrap();
                    input = input.trim().to_string().to_uppercase();

                    if input.contains("QUIT") {
                        process::exit(0);
                    } else if input == "".to_string() {
                        continue;
                    } else if game.letters_guessed.contains(input.as_str()) {
                        paint.answer_result =
                            format!("You already guessed that letter, you silly billy goat Gus!");
                    } else if input.len() == 1 && input.chars().all(|c| c.is_alphabetic()) {
                        guess = input.clone();
                        exit_loop = true;
                    } else {
                        paint.answer_result = format!("Enter a single letter pls!");
                    }
                }
            }

            let is_correct_guess =
                puzzle.creature.contains(&guess) && !game.question.contains(&guess);
            // Add a letter that you guessed
            game.letters_guessed.push_str(&guess);
            // game.letters_guessed = sort_string_alphabetically(&game.letters_guessed);
            if is_correct_guess {
                paint.answer_result = format!("{guess} was CORRECT!");
                // Update the question to show your progress
                game.question = update_question(&puzzle.creature, &game.question, &guess);
            } else {
                paint.answer_result = format!("{guess} was INCORRECT!");
                game.health -= 1;
            }

            // Check if you are a winner of the puzzle
            let is_winning_question = is_question_winning(&game.question);
            if is_winning_question {
                // You WON A PUZZLE, and solved the hangman
                paint.answer_result = format!(
                    "\n\nYou: Hi, {}!\n\n{}:{}\n\nHealth++;\nCash++;\n\nPress Enter to greet the next tiny creature",
                    puzzle.creature, puzzle.creature, "Sup!"
                );
                game.health += 3;
                game.health = game.health.min(100);
                game.cash += puzzle.frequency_score;

                paint_terminal_winning_game(&paint);
                wait_for_user_input();
                paint.answer_result = "".to_string();
                break;
            }

            if game.health < 1 {
                paint.answer_result = format!("You lost the game, shitheel!");
                process::exit(0);
            }
        }
    }
    println!("You won the game! Press ENTER to QUIT\nI didnt make the game loop yet, sorry :P");
    let mut input = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut input).unwrap();
    Ok(())
}

fn wait_for_user_input() {
    // Poll stdin, to let user (force them) to read the story
    let mut input = String::new();
    let stdin = io::stdin();
    // We get `Stdin` here.
    stdin.read_line(&mut input).unwrap();
}

// fn sort_string_alphabetically(s: &str) -> String {
//     let mut chars: Vec<char> = s.chars().collect(); // Convert string to vector of characters
//     chars.sort(); // Sort the characters
//     chars.into_iter().collect() // Collect the sorted characters back into a string
// }

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

fn paint_terminal(paint: &Paint) {
    let _ = io::stdout().execute(Hide);
    let _ = io::stdout().execute(MoveTo(0, 0)); // Move to the top-left corner
    let _ = io::stdout().execute(Clear(ClearType::All));
    let _ = io::stdout().execute(Show);

    println!(
        "{}\n\n{}\n{}\n",
        &paint.intro, &paint.status, &paint.answer_result
    );
}

fn paint_terminal_winning_game(paint: &Paint) {
    let _ = io::stdout().execute(Hide);
    let _ = io::stdout().execute(MoveTo(0, 0)); // Move to the top-left corner
    let _ = io::stdout().execute(Clear(ClearType::All));
    let _ = io::stdout().execute(Show);

    println!("{}", &paint.answer_result);
}
