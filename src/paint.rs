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

    println!(
        "{}\n\n{}\n{}\n",
        &paint.intro, &paint.status, &paint.answer_result
    );
}

pub fn paint_terminal_winning_game(paint: &Paint) {
    let _ = io::stdout().execute(Hide);
    let _ = io::stdout().execute(MoveTo(0, 0)); // Move to the top-left corner
    let _ = io::stdout().execute(Clear(ClearType::All));
    let _ = io::stdout().execute(Show);

    println!("{}", &paint.answer_result);
}
