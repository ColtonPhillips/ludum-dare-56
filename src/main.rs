mod copy;
mod creatures;
mod model;
mod paint;
mod puzzle;
mod tunes;

use creatures::*;

use model::*;
use paint::*;
use puzzle::*;
use rand::{seq::SliceRandom, Rng};

use std::{collections::HashSet, io, process};

fn main() -> io::Result<()> {
    if cfg!(feature = "audio") {
        tunes::play_bg_music();
    }

    let mut paint = Paint {
        intro: copy::SKIPPABLE_INTRO.to_string(),
        ..Default::default()
    };

    // Get this party started
    let selected_puzzles = fetch_selected_puzzles();
    let greetings = fetch_greetings();
    let mut game = Game::default();
    // if false {
    for puzzle in selected_puzzles.clone() {
        // Setup: refactor into method?
        game.puzzle = puzzle;
        game.hints_unlocked = 1;
        game.question = convert_name_to_guess_format(&game.puzzle.creature);
        game.letters_guessed = String::from("");
        game.rnd_greeting = greetings.choose(&mut rand::thread_rng()).unwrap().clone();
        // Get hints in random order, and allow user to unlock them as they play
        let mut rnd_hints = game.puzzle.hints.clone();
        rnd_hints.shuffle(&mut rand::thread_rng());
        game.rnd_hints = rnd_hints.clone();
        //
        loop {
            paint_state(&game);
            let input = get_user_input();
            update_state(&mut game, &input);
            match game.state {
                State::WinPuzzle() => break,
                _ => (),
            }
        }
    }
    // }

    // to be refactored
    for puzzle in selected_puzzles {
        game.hints_unlocked = 1;

        // Setup
        game.question = convert_name_to_guess_format(&puzzle.creature);
        game.letters_guessed = String::from("");
        game.rnd_greeting = greetings.choose(&mut rand::thread_rng()).unwrap().clone();

        // Get hints in random order, and allow user to unlock them as they play
        let mut rnd_hints = puzzle.hints.clone();
        rnd_hints.shuffle(&mut rand::thread_rng());

        loop {
            // update rnd_hints to make sure the user can see hint messaging from unlocked letters
            game.rnd_hint =
                reveal_guessed_letters(&rnd_hints, &game.letters_guessed, game.hints_unlocked);

            // Render the puzzle and question
            paint.status = format!(
            "{}: '{}'\n{}\n\nYou: 'Heyyy...'\n\nMy thoughts:\n{}\n\nHealth: {}, Cash:{}, Unused Letters:{}\nEnter a Letter...",
                game.question,
                game.rnd_greeting,
                puzzle.creature_length_hint,
                game.rnd_hint,
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
                        if game.cash < game.bisect_cost {
                            paint.answer_result = format!(
                                "You can't buy it. You need ${} to buy-sect the unused letters!",
                                game.bisect_cost
                            );
                        } else {
                            game.letters_guessed = bisect_guessable_letters(&puzzle, &game);
                            game.cash -= game.bisect_cost;
                            game.bisect_cost = ((game.bisect_cost as f64) * 1.15).floor() as usize;
                            paint.answer_result =
                                format!("You paid a friend to remove some of the options, but they want {} next time", game.bisect_cost);
                        }

                        // XXX copy paste coding necessary because we didnt make a guess but wanna update status
                        game.rnd_hint = reveal_guessed_letters(
                            &rnd_hints,
                            &game.letters_guessed,
                            game.hints_unlocked,
                        );
                        paint.status = format!(
                                "{}: '{}'\n{}\n\nYou: 'Heyyy...'\n\nMy thoughts:\n{}\n\nHealth: {}, Cash:{}, Unused Letters:{}\nEnter a Letter...",
                                    game.question,
                                    game.rnd_greeting,
                                    puzzle.creature_length_hint,
                                    game.rnd_hint,
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

fn update_state(game: &mut Game, input: &str) {
    match game.state {
        State::Introduction() => {
            game.state = State::PlayerInput();
        }
        State::WinPuzzle() => {
            game.state = State::PlayerInput();
        }
        State::PlayerInput() => {
            // update rnd_hints to make sure the user can see hint messaging from unlocked letters
            game.rnd_hint =
                reveal_guessed_letters(&game.rnd_hints, &game.letters_guessed, game.hints_unlocked);

            // // Render the puzzle and question
            // paint.status = format!(
            //     "{}: '{}'\n{}\n\nYou: 'Heyyy...'\n\nMy thoughts:\n{}\n\nHealth: {}, Cash:{}, Unused Letters:{}\nEnter a Letter...",
            //         game.question,
            //         game.rnd_greeting,
            //         puzzle.creature_length_hint,
            //         game.rnd_hint,
            //         game.health,
            //         game.cash,
            //         find_unused_letters(&game.letters_guessed),
            //     );

            let mut guess = String::from("");
            let mut exit_loop = false;
            {
                // This block handles user input
                while !exit_loop {
                    if input.contains("BUY") {
                        // this has a bug where you never remove the last unused letter on odd amounts
                        if game.cash < game.bisect_cost {
                            // paint.answer_result = format!(
                            // "You can't buy it. You need ${} to buy-sect the unused letters!",
                            // game.bisect_cost
                            // );
                        } else {
                            game.letters_guessed = bisect_guessable_letters(&game.puzzle, &game);
                            game.cash -= game.bisect_cost;
                            game.bisect_cost = ((game.bisect_cost as f64) * 1.15).floor() as usize;
                            // paint.answer_result =
                            // format!("You paid a friend to remove some of the options, but they want {} next time", game.bisect_cost);
                        }

                        // XXX copy paste coding necessary because we didnt make a guess but wanna update status
                        game.rnd_hint = reveal_guessed_letters(
                            &game.rnd_hints,
                            &game.letters_guessed,
                            game.hints_unlocked,
                        );
                        // paint.status = format!(
                        //             "{}: '{}'\n{}\n\nYou: 'Heyyy...'\n\nMy thoughts:\n{}\n\nHealth: {}, Cash:{}, Unused Letters:{}\nEnter a Letter...",
                        //                 game.question,
                        //                 game.rnd_greeting,
                        //                 puzzle.creature_length_hint,
                        //                 game.rnd_hint,
                        //                 game.health,
                        //                 game.cash,
                        //                 find_unused_letters(&game.letters_guessed),
                        //             );
                        // paint_terminal(&paint);
                    } else if input.contains("QUIT") {
                        process::exit(0);
                    } else if input == "".to_string() {
                        continue;
                    } else if game.letters_guessed.contains(input) {
                        // paint.answer_result =
                        // format!("You already guessed that letter, you silly billy goat Gus!");
                    } else if input.len() == 1 && input.chars().all(|c| c.is_alphabetic()) {
                        guess = input.to_string();
                        exit_loop = true;
                    } else {
                        // paint.answer_result = format!("Enter a single letter pls!");
                    }
                }
            }

            let is_correct_guess =
                game.puzzle.creature.contains(&guess) && !game.question.contains(&guess);
            // Add a letter that you guessed
            game.letters_guessed.push_str(&guess);
            // game.letters_guessed = sort_string_alphabetically(&game.letters_guessed);
            if is_correct_guess {
                // paint.answer_result = format!("{guess} was CORRECT!");
                // Update the question to show your progress
                game.question = update_question(&game.puzzle.creature, &game.question, &guess);
            } else {
                game.hints_unlocked += 1; // You got something wrong, but you learned something new!
                let psychic_damage = rand::thread_rng().gen_range(1..5);
                // paint.answer_result =
                // format!("{guess} was INCORRECT!\nGuest did {psychic_damage} psychic damage to your ego!");
                game.health -= psychic_damage;
            }

            // Check if you are a winner of the puzzle
            let is_winning_question = is_question_winning(&game.question);
            if is_winning_question {
                // You WON A PUZZLE, and solved the hangman
                // paint.answer_result = format!(
                //         "\n\nYou: Hi, {}!\n\n{}:{}\n\nHealth++;\nCash++;\n\nPress Enter to greet the next tiny creature",
                //         puzzle.creature, puzzle.creature, "Sup!"
                //     );
                game.health += 5;
                game.health = game.health.min(100);
                game.cash += game.puzzle.frequency_score;

                // paint_terminal_winning_game(&paint);
                wait_for_user_input();
                // paint.answer_result = "".to_string();
                // break;
                game.state = State::WinPuzzle()
            }

            if game.health < 1 {
                // paint.answer_result = format!("You lost the game, shitheel!");
                process::exit(0);
            }
        } // game.state = State::Introduction();
    }
}

fn get_user_input() -> String {
    // Get User input ( a single character)
    let mut input = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut input).unwrap();
    input = input.trim().to_string().to_uppercase();
    return input.clone();
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
