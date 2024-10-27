use crate::copy::*;
use crate::find_unused_letters;
use crate::model::*;

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io;

pub fn paint_state(game: &Game) {
    let _ = io::stdout().execute(Hide);
    let _ = io::stdout().execute(MoveTo(0, 0)); // Move to the top-left corner
    let _ = io::stdout().execute(Clear(ClearType::All));
    let _ = io::stdout().execute(Show);

    match game.state {
        State::Introduction() => {
            paint_introduction(game);
        }
        State::Ruleset() => {
            paint_ruleset(game);
        }
        State::WinPuzzle() => {
            paint_win_puzzle(game);
        }
        State::PlayerInput() => {
            paint_player_input(game);
        }
        _ => {}
    }
}

fn paint_player_input(game: &Game) {
    println!(
        "A {} looks up at you eagerly:

  {}: '{}'
  {}

  You: 'Heyyy...'

Your errant thoughts:
{}
Health: {}, Cash:{}, Unused Letters:{}
Enter Letter, 'BUY', 'HELP', or 'QUIT':
{}",
        game.question,
        game.question,
        game.rnd_greeting,
        game.puzzle.creature_length_hint,
        game.rnd_hint,
        game.health,
        game.cash,
        find_unused_letters(&game.letters_guessed),
        game.result
    );
}

fn paint_introduction(_game: &Game) {
    println!("{}", &SKIPPABLE_INTRO);
}

fn paint_ruleset(game: &Game) {
    println!(
        "   You find yourself at a support group
   for SMALL CREATURES with {} bucks
   in your pocket. Happy members line 
   up eagerly to be greeted by YOU. 
      
RULES:
  Remember the name of each creature(s):
      
    (e.g. 'PUPPIES' 
    has 3 'P's, 1 'U', and 1 'Y')
    (e.g. 'TWEETY'
    has 2 'T's, 2 'E's, 1 'W', and 1 'Y')
      
  Looking at each creature will give you
  errant thoughts (e.g. \"This guy is cool!\")
      
  Don't make too many mistakes or people
  will think that you're a bit of a narcissist.
    
Controls: 
  'L' (Any Letter) Guess a letter of their name
  'BUY' For ${}, remove some unguessed letters
  'HELP' - Open this help menu  
  'QUIT' to leave at any time.
  
Press Enter to proceed",
        game.cash, game.bisect_cost
    );
}

fn paint_win_puzzle(game: &Game) {
    // TODO: hook into random greetings again.
    println!(
        "\n\nYou: Hi, {}!\n\n{}:{}\n\nHealth++;\nCash++;\n\n Press Enter to continue",
        game.puzzle.creature, game.puzzle.creature, "Sup!"
    );
}
