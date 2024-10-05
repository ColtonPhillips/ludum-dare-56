mod creatures;

use creatures::parse_creatures;
use std::io;

fn main() -> io::Result<()> {
    let tokens = parse_creatures();
    print!("{tokens:?}");
    Ok(())
}
