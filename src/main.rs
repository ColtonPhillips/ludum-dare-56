mod creatures;

use creatures::{fetch_puzzles, parse_creatures};
use std::io;

fn main() -> io::Result<()> {
    let tokens = parse_creatures();
    println!("{tokens:?}");

    let puzzles = fetch_puzzles(tokens);
    println!("{puzzles:#?}");
    Ok(())
}
