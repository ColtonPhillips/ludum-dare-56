use crate::model::*;
use crate::puzzle::*;

use rand::seq::SliceRandom;
use regex::Regex;

pub type Puzzles = Vec<Puzzle>;
pub fn fetch_selected_puzzles() -> Puzzles {
    let puzzles: Puzzles = fetch_puzzles();

    let num_buckets = 10;
    let chunk_size = (puzzles.len() + num_buckets - 1) / num_buckets;
    let puzzle_buckets: Vec<&[Puzzle]> = puzzles.chunks(chunk_size).collect();

    let mut selected_puzzles: Puzzles = Puzzles::new();

    for bucket in puzzle_buckets {
        let p = bucket.choose(&mut rand::thread_rng()).unwrap();
        selected_puzzles.push(p.clone());
    }
    return selected_puzzles.clone();
}

pub fn fetch_puzzles() -> Puzzles {
    let tokens = parse_creatures();
    let mut puzzles = Puzzles::new();
    let mut hint_buffer: Hints = vec![];
    for token in tokens {
        match token {
            Token::Hint(hint) => {
                hint_buffer = hint;
            }
            Token::Name(name) => {
                puzzles.push(Puzzle {
                    creature: name.clone().to_uppercase(),
                    creature_length_hint: calculate_name_length_hint(name.clone()),
                    hints: hint_buffer.clone(),
                    naive_score: calculate_naive_score(name.clone()),
                    unique_score: calculate_unique_score(name.clone()),
                    frequency_score: calculate_frequency_score(name.clone()),
                });
                hint_buffer = vec![];
            }
        }
    }
    // Sort puzzles by the weighted sum of unique_score, naive_score, and frequency_score
    let naive_weight = 0.5;
    let unique_weight = 0.5;
    let frequency_weight = 0.25;
    puzzles.sort_by(|a, b| {
        let a_total_score = (a.naive_score as f32 * naive_weight)
            + (a.unique_score as f32 * unique_weight)
            + (a.frequency_score as f32 * frequency_weight);

        let b_total_score = (b.naive_score as f32 * naive_weight)
            + (b.unique_score as f32 * unique_weight)
            + (b.frequency_score as f32 * frequency_weight);

        a_total_score.partial_cmp(&b_total_score).unwrap()
    });

    puzzles
}

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
                    let hint = caps.get(1).unwrap().as_str();
                    let hints: Hints = hint
                        .to_string()
                        .split("|")
                        .map(|s| s.trim().to_string())
                        .collect();

                    tokens.push(Token::Hint(hints.clone()));
                }
                // Then extract the line for the creature and trim whitespace
                let line = re.replace(line, "").to_string();
                tokens.push(Token::Name(String::from(line.trim())));
            }
        }
    }
    tokens
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

pub fn fetch_greetings() -> Vec<String> {
    include_str!("greetings.txt")
        .lines()
        .map(|lines| lines.trim().to_string())
        .collect()
}
