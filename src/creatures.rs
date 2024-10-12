use std::collections::HashSet;

use regex::Regex;

#[derive(Debug)]
pub enum Token {
    Name(String),
    Hint(String),
}
pub type Tokens = Vec<Token>;

pub fn fetch_greetings() -> Vec<String> {
    include_str!("greetings.txt")
        .lines()
        .map(|lines| lines.trim().to_string())
        .collect()
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

#[derive(Debug, Clone)]
pub struct Puzzle {
    pub creature: String,
    pub creature_length_hint: String,
    pub hint: String,
    pub naive_score: usize,
    pub unique_score: usize,
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
                    creature_length_hint: calculate_name_length_hint(name.clone()),
                    hint: hint_buffer.clone(),
                    naive_score: calculate_naive_score(name.clone()),
                    unique_score: calculate_unique_score(name.clone()),
                    frequency_score: calculate_frequency_score(name.clone()),
                });
                hint_buffer = String::from("");
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

fn calculate_name_length_hint(name: String) -> String {
    let x: Vec<String> = name
        .split_ascii_whitespace()
        .map(|s| s.trim().to_string())
        .collect();
    x.iter()
        .map(|word| {
            // For each creature name's word, find the number of whitespace on each side of the
            // creature name's length hint e.g. below is a word of len 4
            // MITE
            // ____
            // *4**
            let name_len_as_str = word.len().to_string();
            let num_of_asterisks_used = word.len() - name_len_as_str.len();
            let name_len_index = (num_of_asterisks_used as f64 / 2.0).floor() as usize;
            let left_whitespace = " ".repeat(name_len_index);
            let right_whitespace = " ".repeat(num_of_asterisks_used - name_len_index);
            let word_length_hint: String =
                format!("{}{}{}", left_whitespace, name_len_as_str, right_whitespace);
            word_length_hint
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn calculate_naive_score(s: String) -> usize {
    s.as_str().chars().filter(|c| !c.is_whitespace()).count()
}

fn calculate_unique_score(s: String) -> usize {
    let mut unique_chars = HashSet::new(); // Set to track unique characters
    let mut score = 0; // Initialize score to zero

    for c in s.chars() {
        if c.is_alphabetic() && !unique_chars.contains(&c) {
            // Check if alphabetic and not already counted
            unique_chars.insert(c); // Insert unique character
            score += 1; // Increment score for each unique character
        }
    }

    score
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
