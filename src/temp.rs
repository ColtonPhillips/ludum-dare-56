//  // update rnd_hints to make sure the user can see hint messaging from unlocked letters
//             game.rnd_hint =
//                 reveal_guessed_letters(&game.rnd_hints, &game.letters_guessed, game.hints_unlocked);

// let mut exit_loop = false;
// {
// This block handles user input
// while !exit_loop {
// if input.contains("BUY") {
//     // this has a bug where you never remove the last unused letter on odd amounts
//     if game.cash < game.bisect_cost {
//         // paint.answer_result = format!(
//         // "You can't buy it. You need ${} to buy-sect the unused letters!",
//         // game.bisect_cost
//         // );
//     } else {
//         game.letters_guessed = bisect_guessable_letters(&game.puzzle, &game);
//         game.cash -= game.bisect_cost;
//         game.bisect_cost = ((game.bisect_cost as f64) * 1.15).floor() as usize;
//         // paint.answer_result =
//         // format!("You paid a friend to remove some of the options, but they want {} next time", game.bisect_cost);
//     }

//     // XXX copy paste coding necessary because we didnt make a guess but wanna update status
//     game.rnd_hint = reveal_guessed_letters(
//         &game.rnd_hints,
//         &game.letters_guessed,
//         game.hints_unlocked,
//     );
//     // paint.status = format!(
//     //             "{}: '{}'\n{}\n\nYou: 'Heyyy...'\n\nMy thoughts:\n{}\n\nHealth: {}, Cash:{}, Unused Letters:{}\nEnter a Letter...",
//     //                 game.question,
//     //                 game.rnd_greeting,
//     //                 puzzle.creature_length_hint,
//     //                 game.rnd_hint,
//     //                 game.health,
//     //                 game.cash,
//     //                 find_unused_letters(&game.letters_guessed),
//     //             );
//     // paint_terminal(&paint);

// let is_correct_guess =
//     game.puzzle.creature.contains(&guess) && !game.question.contains(&guess);
// // Add a letter that you guessed
// game.letters_guessed.push_str(&guess);
// // game.letters_guessed = sort_string_alphabetically(&game.letters_guessed);
// if is_correct_guess {
//     // paint.answer_result = format!("{guess} was CORRECT!");
//     // Update the question to show your progress
//     game.question = update_question(&game.puzzle.creature, &game.question, &guess);
// } else {
//     game.hints_unlocked += 1; // You got something wrong, but you learned something new!
//     let psychic_damage = rand::thread_rng().gen_range(1..5);
//     // paint.answer_result =
//     // format!("{guess} was INCORRECT!\nGuest did {psychic_damage} psychic damage to your ego!");
//     game.health -= psychic_damage;
// }

// // Check if you are a winner of the puzzle
// let is_winning_question = is_question_winning(&game.question);
// if is_winning_question {
//     // You WON A PUZZLE, and solved the hangman
//     // paint.answer_result = format!(
//     //         "\n\nYou: Hi, {}!\n\n{}:{}\n\nHealth++;\nCash++;\n\nPress Enter to greet the next tiny creature",
//     //         puzzle.creature, puzzle.creature, "Sup!"
//     //     );
//     game.health += 5;
//     game.health = game.health.min(100);
//     game.cash += game.puzzle.frequency_score;

//     // paint_terminal_winning_game(&paint);
//     wait_for_user_input();
//     // paint.answer_result = "".to_string();
//     // break;
//     game.state = State::WinPuzzle()
// }

// if game.health < 1 {
//     // paint.answer_result = format!("You lost the game, shitheel!");
//     process::exit(0);
// }
// game.result = format!(
//     "\n\nYou: Hi, {}!\n\n{}:{}\n\nHealth++;\nCash++;\n\nPress Enter to greet the next tiny creature",
//     game.puzzle.creature, game.puzzle.creature, "Sup!"
// );
