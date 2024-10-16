mod creatures;

use creatures::{
    convert_name_to_guess_format, fetch_greetings, fetch_puzzles, is_question_winning,
    parse_creatures, update_question, Puzzle, Puzzles,
};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use rand::{seq::SliceRandom, Rng};
use rodio::{Decoder, OutputStream, Sink, Source};
use std::io::Cursor;
use std::{collections::HashSet, io, process, thread};

#[derive(Debug)]
struct Game {
    question: String,
    letters_guessed: String,
    health: usize,
    cash: usize,
    hints_unlocked: usize,
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
    if cfg!(feature = "audio") {
        just_play_some_tunes_bro();
    }

    let mut bisect_cost: usize = 15;
    let mut paint = Paint {
        intro: format!(
            "
This game was built for the Ludum Dare 56 Solo Jam in October 2024
Author: Colton Phillips

Tiny Creatures Support Group!
============================

You find yourself at a support group for SMOL CREATURES! 
Someone forgot to bring any nametags so it's your job to
guess the names of each attendant one LETTER at a time!
Don't make too many mistakes or people will think you are
a bit of a narcissist. 

RULES:
=====
Guess the creature's name ONE letter at a time.
Type 'BUY' to BUY-sect the unused guesses. Costs {bisect_cost}
Type 'QUIT' to leave at any time.

Press Enter to greet the first member
of the Tiny Creature Support Group (I hope they brought snacks!)
"
        )
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

    let greetings = fetch_greetings();

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
        cash: bisect_cost,
        hints_unlocked: 1,
    };

    for puzzle in selected_puzzles {
        game.hints_unlocked = 1;

        // Setup
        game.question = convert_name_to_guess_format(&puzzle.creature);
        game.letters_guessed = String::from("");
        let rnd_greeting = greetings.choose(&mut rand::thread_rng()).unwrap();

        // Get hints in random order, and allow user to unlock them as they play
        let mut rnd_hints = puzzle.hints.clone();
        rnd_hints.shuffle(&mut rand::thread_rng());

        loop {
            // update rnd_hints to make sure the user can see hint messaging from unlocked letters
            let mut rnd_hint =
                reveal_guessed_letters(&rnd_hints, &game.letters_guessed, game.hints_unlocked);

            // Render the puzzle and question
            paint.status = format!(
            "{}: '{}'\n{}\n\nYou: 'Heyyy...'\n\nMy thoughts:\n{}\n\nHealth: {}, Cash:{}, Unused Letters:{}\nEnter a Letter...",
                game.question,
                rnd_greeting,
                puzzle.creature_length_hint,
                rnd_hint,
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

                    if input.contains("BUY") {
                        // this has a bug where you never remove the last unused letter on odd amounts
                        if game.cash < bisect_cost {
                            paint.answer_result =
                                format!("You can't buy it. You need ${bisect_cost} to buy-sect the unused letters!");
                        } else {
                            game.letters_guessed = bisect_guessable_letters(&puzzle, &game);
                            game.cash -= bisect_cost;
                            bisect_cost = ((bisect_cost as f64) * 1.15).floor() as usize;
                            paint.answer_result =
                                format!("You paid a friend to remove some of the options, but they want {bisect_cost} next time");
                        }

                        // XXX copy paste coding necessary because we didnt make a guess but wanna update status
                        rnd_hint = reveal_guessed_letters(
                            &rnd_hints,
                            &game.letters_guessed,
                            game.hints_unlocked,
                        );
                        paint.status = format!(
                                "{}: '{}'\n{}\n\nYou: 'Heyyy...'\n\nMy thoughts:\n{}\n\nHealth: {}, Cash:{}, Unused Letters:{}\nEnter a Letter...",
                                    game.question,
                                    rnd_greeting,
                                    puzzle.creature_length_hint,
                                    rnd_hint,
                                    game.health,
                                    game.cash,
                                    find_unused_letters(&game.letters_guessed),
                                );
                        paint_terminal(&paint);
                    } else if input.contains("QUIT") {
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
                game.hints_unlocked += 1; // You got something wrong, but you learned something new!
                let psychic_damage = rand::thread_rng().gen_range(1..5);
                paint.answer_result =
                    format!("{guess} was INCORRECT!\nGuest did {psychic_damage} psychic damage to your ego!");
                game.health -= psychic_damage;
            }

            // Check if you are a winner of the puzzle
            let is_winning_question = is_question_winning(&game.question);
            if is_winning_question {
                // You WON A PUZZLE, and solved the hangman
                paint.answer_result = format!(
                    "\n\nYou: Hi, {}!\n\n{}:{}\n\nHealth++;\nCash++;\n\nPress Enter to greet the next tiny creature",
                    puzzle.creature, puzzle.creature, "Sup!"
                );
                game.health += 5;
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

// kinda shit
fn reveal_guessed_letters(input: &Vec<String>, guessed: &str, count_hints: usize) -> String {
    let mut inside_braces = false;
    let mut result = String::new();
    let mut good_old_fashioned_i = 1;
    for s in input {
        result.push_str("- ");
        for c in s.chars() {
            if c == '{' {
                inside_braces = true;
            } else if c == '}' {
                inside_braces = false;
            } else if inside_braces
                && c.is_alphabetic()
                && !guessed.contains(c.to_ascii_uppercase())
            {
                result.push('_'); // Replace unguessed letters inside curly braces
            } else {
                result.push(c); // Keep everything else as is
            }
        }
        result.push('\n');
        if good_old_fashioned_i >= count_hints {
            break;
        } else {
            good_old_fashioned_i += 1;
        }
    }

    result
}

fn bisect_guessable_letters(puzzle: &Puzzle, game: &Game) -> String {
    let all_letters: HashSet<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let creature_letters: HashSet<char> = puzzle.creature.chars().collect();
    let letters_guessed: HashSet<char> = game.letters_guessed.chars().collect();
    let mut letters_guessed_v = game.letters_guessed.clone();
    let all_letters_less_creature_name: HashSet<char> = all_letters
        .difference(&creature_letters)
        .cloned()
        .collect::<HashSet<char>>();
    let all_letter_less_creature_name_and_guessed_letters = all_letters_less_creature_name
        .difference(&letters_guessed)
        .cloned()
        .collect::<HashSet<char>>();

    let letters_vec: Vec<char> = all_letter_less_creature_name_and_guessed_letters
        .into_iter()
        .collect();

    let half_count = letters_vec.len() / 2;

    let mut rng = rand::thread_rng();

    let random_letters: Vec<char> = letters_vec
        .choose_multiple(&mut rng, half_count)
        .cloned()
        .collect();

    letters_guessed_v.extend(random_letters);
    letters_guessed_v
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
