use std::collections::HashSet;

pub fn calculate_name_length_hint(name: String) -> String {
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
            let num_of_whitespace_used = word.len() - name_len_as_str.len();
            let name_len_index = (num_of_whitespace_used as f64 / 2.0).floor() as usize;
            let left_whitespace = " ".repeat(name_len_index);
            let right_whitespace = " ".repeat(num_of_whitespace_used - name_len_index);
            let word_length_hint: String =
                format!("{}{}{}", left_whitespace, name_len_as_str, right_whitespace);
            word_length_hint
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn calculate_naive_score(s: String) -> usize {
    s.as_str().chars().filter(|c| !c.is_whitespace()).count()
}

pub fn calculate_unique_score(s: String) -> usize {
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
pub fn calculate_frequency_score(word: String) -> usize {
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

pub fn is_question_winning(question: &str) -> bool {
    !question.contains("_")
}
