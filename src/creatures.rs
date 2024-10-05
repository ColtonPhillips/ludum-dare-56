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
    pub creature: String,
    pub hint: String,
    pub naive_score: usize,
    pub frequency_score: usize,
}
pub type Puzzles = Vec<Puzzle>;

pub fn fetch_puzzles(tokens: Tokens) -> Puzzles {
    let mut puzzles = Puzzles::new();
    let mut hint_buffer = String::from("");
    for token in tokens {
        match token {
            Token::Hint(hint) => {
                hint_buffer = hint;
            }
            Token::Name(name) => {
                puzzles.push(Puzzle {
                    creature: name.clone().to_uppercase(),
                    hint: hint_buffer.clone(),
                    naive_score: calculate_naive_score(name.clone()),
                    frequency_score: calculate_frequency_score(name.clone()),
                });
                hint_buffer = String::from("");
            }
        }
    }
    puzzles
}

fn calculate_naive_score(s: String) -> usize {
    s.as_str().chars().filter(|c| !c.is_whitespace()).count()
}

// Based on 'you know what' game
fn calculate_frequency_score(word: String) -> usize {
    fn char_score(c: char) -> usize {
        match c.to_ascii_uppercase() {
            // Convert to uppercase for consistency
            'A' | 'E' | 'I' | 'L' | 'N' | 'O' | 'R' | 'S' | 'T' | 'U' => 1,
            'D' | 'G' => 2,
            'B' | 'C' | 'M' | 'P' => 3,
            'F' | 'H' | 'V' | 'W' | 'Y' => 4,
            'K' => 5,
            'J' | 'X' => 8,
            'Q' | 'Z' => 10,
            _ => 0, // Any non-alphabet character scores 0
        }
    }

    // Calculate the total score for the given word
    word.as_str().chars().map(char_score).sum()
}

pub fn convert_name_to_guess_format(creature: &str) -> String {
    let guess = creature
        .chars()
        .map(|c| if c.is_alphabetic() { '_' } else { c })
        .collect();
    guess
}

pub fn update_question(creature: &str, question: &str, guess: &str) -> String {
    let mut result: Vec<char> = question.chars().collect();
    for (i, c) in creature.chars().enumerate() {
        if c == guess.chars().next().unwrap() {
            result[i] = c;
        }
    }
    result.into_iter().collect()
}

pub fn is_question_winning(question: &str) -> bool {
    !question.contains("_")
}
