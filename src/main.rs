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

    // Get this party started
    let selected_puzzles = fetch_selected_puzzles();
    let mut game = Game::default();
    for puzzle in selected_puzzles.clone() {
        game.puzzle = puzzle;
        game.setup_a_puzzle_meh();
        game.update_puzzle_meh();

        // Paint game state once before taking user input, so we can see the game state before it all starts up
        paint_state(&game);
        loop {
            let input = update_input(&game);
            update_state(&mut game, &input);
            game.update_puzzle_meh();
            paint_state(&game);
            match game.state {
                State::NextPuzzle() => break,
                State::QuitGame() => process::exit(0),
                _ => (),
            }
        }
    }

    println!("You won the game! Press ENTER to QUIT\nI didnt make the game loop yet, sorry :P");
    let mut input = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut input).unwrap();
    Ok(())
}

fn update_input(game: &Game) -> String {
    match game.state {
        State::SetupAPuzzle() => String::new(),
        State::NextPuzzle() => String::new(),
        _ => get_user_input(),
    }
}

fn update_state(game: &mut Game, input: &str) {
    match game.state {
        State::QuitGame() => {}
        State::Introduction() => {
            game.state = State::Ruleset();
            game.setup_a_puzzle_meh(); // hack: ask colton
            game.update_puzzle_meh();
        }
        State::Ruleset() => game.state = State::PlayerInput(),
        State::SetupAPuzzle() => {
            game.setup_a_puzzle_meh();
            game.state = State::PlayerInput();
        }
        State::WinPuzzle() => {
            game.hints_unlocked = 1;
            game.update_puzzle_meh();
            game.result = "".to_string();
            game.state = State::NextPuzzle();
        }
        State::NextPuzzle() => game.state = State::PlayerInput(),
        State::PlayerInput() => {
            match input {
                "" => {}
                _ if input.contains("QUIT") => game.state = State::QuitGame(),
                _ if input.contains("HELP") => game.state = State::Ruleset(),
                _ if input.contains("BUY") => {
                    if game.cash < game.bisect_cost {
                        game.result = format!(
                            "You can't buy it. You need ${} to buy-sect the unused letters!",
                            game.bisect_cost
                        )
                    } else {
                        game.letters_guessed = bisect_guessable_letters(&game.puzzle, &game);
                        game.cash -= game.bisect_cost;
                        game.bisect_cost = ((game.bisect_cost as f64) * 1.15).floor() as usize;
                        game.result =
                                format!("You paid a friend to remove some of the options, but they want {} next time", game.bisect_cost);
                    }
                }
                _ if game.letters_guessed.contains(input) => {
                    game.result =
                        format!("You already guessed that letter, you silly billy goat Gus!");
                }
                _ if input.len() == 1 && input.chars().all(|c| c.is_alphabetic()) => {
                    let guess = input.to_string();
                    // Add the letter you guessed
                    game.letters_guessed.push_str(&guess);
                    // game.letters_guessed = sort_string_alphabetically(&game.letters_guessed);

                    // Check if the guess is in the creature's name
                    let is_correct_guess =
                        game.puzzle.creature.contains(&guess) && !game.question.contains(&guess);

                    if is_correct_guess {
                        // Add a letter that you guessed
                        game.result = format!("{guess} was CORRECT!");

                        game.cash += 1;
                        // Update the question to show your progress
                        game.question =
                            update_question(&game.puzzle.creature, &game.question, &guess);
                    } else {
                        game.hints_unlocked += 1; // You got something wrong, but you learned something new!
                        let psychic_damage = rand::thread_rng().gen_range(1..5);
                        game.result = format!("{guess} was INCORRECT!\nGuest did {psychic_damage} psychic damage to your ego!");
                        game.health -= psychic_damage;
                    }

                    // Check if you are a winner of the puzzle
                    let is_winning_question = is_question_winning(&game.question);
                    if is_winning_question {
                        // You WON A PUZZLE, and solved the hangman

                        game.health += 5;
                        game.health = game.health.min(100);
                        game.cash += 5 + game.puzzle.frequency_score;
                        game.state = State::WinPuzzle();
                    }

                    if game.health < 1 {
                        game.state = State::QuitGame();
                        // paint.answer_result = format!("You lost the game, shitheel!");
                        // process::exit(0);
                    }
                }
                _ => {
                    game.result = "".to_string();
                }
            }
        }
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
