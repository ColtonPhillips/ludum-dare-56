use regex::Regex;

#[derive(Debug)]
pub enum Token {
    Name(String),
    Hint(String),
}
pub type Tokens = Vec<Token>;

pub fn parse_creatures() -> Tokens {
    let creatures = include_str!("creatures.txt");
    let mut tokens: Tokens = Tokens::new();
    for line in creatures.lines() {
        match line {
            "" => {} // split off empty lines in a dumb way
            _ => {
                // Extract a hint! if one exists, and tokenize it
                let re = Regex::new(r#""([^"]+)""#).unwrap();
                if let Some(caps) = re.captures(line) {
                    let h = caps.get(1).unwrap().as_str();
                    tokens.push(Token::Hint(h.to_string()));
                }
                // Then extract the line for the creature and trim whitespace
                let line = re.replace(line, "").to_string();
                tokens.push(Token::Name(String::from(line.trim())));
            }
        }
    }
    tokens
}

#[derive(Debug)]
pub struct Puzzle {
    creature: String,
    hint: String,
}
pub type Puzzles = Vec<Puzzle>;

pub fn fetch_puzzles(tokens: Tokens) -> Puzzles {
    let mut puzzles = Puzzles::new();
    let mut h = String::from("");
    for token in tokens {
        match token {
            Token::Hint(s) => {
                h = s;
            }
            Token::Name(s) => {
                puzzles.push(Puzzle {
                    creature: s,
                    hint: h.clone(),
                });
                h = String::from("");
            }
        }
    }
    puzzles
}
