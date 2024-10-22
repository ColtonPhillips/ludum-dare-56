use crate::copy::*;
use crate::find_unused_letters;
use crate::model::*;

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io;

pub fn paint_terminal(paint: &Paint) {
    let _ = io::stdout().execute(Hide);
    let _ = io::stdout().execute(MoveTo(0, 0)); // Move to the top-left corner
    let _ = io::stdout().execute(Clear(ClearType::All));
    let _ = io::stdout().execute(Show);
    println!("{}", &paint.status);
}

pub fn paint_terminal_winning_game(paint: &Paint) {
    let _ = io::stdout().execute(Hide);
    let _ = io::stdout().execute(MoveTo(0, 0)); // Move to the top-left corner
    let _ = io::stdout().execute(Clear(ClearType::All));
    let _ = io::stdout().execute(Show);

    println!("{}", &paint.answer_result);
}

pub fn paint_state(game: &Game) {
    let _ = io::stdout().execute(Hide);
    let _ = io::stdout().execute(MoveTo(0, 0)); // Move to the top-left corner
    let _ = io::stdout().execute(Clear(ClearType::All));
    let _ = io::stdout().execute(Show);

    match game.state {
        State::Introduction() => {
            paint_introduction(game);
        }
        State::WinPuzzle() => {
            paint_win_puzzle(game);
        }

        State::PlayerInput() => {
            paint_player_input(game);
        }
    }
}

fn paint_player_input(game: &Game) {
    println!(
        "{}: '{}'
        {}
        
        You: 'Heyyy...'
        
        My thoughts:
        {}
        
        Health: {}, Cash:{}, Unused Letters:{}
        Enter a Letter...",
        game.question,
        game.rnd_greeting,
        game.puzzle.creature_length_hint,
        game.rnd_hint,
        game.health,
        game.cash,
        find_unused_letters(&game.letters_guessed),
    );
}

fn paint_introduction(game: &Game) {
    println!("{}", &SKIPPABLE_INTRO);
}

fn paint_win_puzzle(game: &Game) {
    println!("Good job!");
}
