use rand::seq::SliceRandom;

#[derive(Debug)]
pub enum Token {
    Name(String),
    Hint(Vec<String>),
}
pub type Tokens = Vec<Token>;
pub type Hints = Vec<String>;

#[derive(Debug, Clone)]
pub struct Puzzle {
    pub creature: String,
    pub creature_length_hint: String,
    pub hints: Hints,
    pub naive_score: usize,
    pub unique_score: usize,
    pub frequency_score: usize,
}
impl Default for Puzzle {
    fn default() -> Self {
        Puzzle {
            creature: "".to_string(),
            creature_length_hint: "".to_string(),
            hints: vec![],
            naive_score: 0,
            unique_score: 0,
            frequency_score: 0,
        }
    }
}
#[derive(Debug, PartialEq)]

pub enum State {
    Introduction(),
    Ruleset(),
    SetupAPuzzle(),
    PlayerInput(),
    WinPuzzle(),
    NextPuzzle(),
    QuitGame(),
}

#[derive(Debug)]
pub struct Game {
    pub puzzle: Puzzle,
    pub state: State,
    pub question: String,
    pub letters_guessed: String,
    pub health: usize,
    pub cash: usize,
    pub hints_unlocked: usize,
    pub bisect_cost: usize,
    pub rnd_greeting: String,
    pub rnd_hints: Vec<String>,
    pub rnd_hint: String,
    pub result: String,
    pub guess: String,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            puzzle: Puzzle::default(),
            state: State::Introduction(),
            question: String::new(),
            letters_guessed: String::new(),
            health: 100,
            cash: 20,
            hints_unlocked: 1,
            bisect_cost: 15,
            rnd_greeting: String::new(),
            rnd_hints: vec![],
            rnd_hint: String::new(),
            result: String::new(),
            guess: String::new(),
        }
    }
}

impl Game {
    // Really not in love with whats going on here so I added "meh"
    pub fn update_puzzle_meh(&mut self) {
        self.guess = "".to_string();
        self.rnd_hint = self.reveal_guessed_letters(
            &self.rnd_hints,
            &self.letters_guessed,
            self.hints_unlocked,
        );
    }

    pub fn setup_a_puzzle_meh(&mut self) {
        self.hints_unlocked = 1;

        self.question = self.convert_name_to_guess_format(&self.puzzle.creature);

        self.letters_guessed = String::from("");
        let greetings = self.fetch_greetings();

        self.rnd_greeting = greetings.choose(&mut rand::thread_rng()).unwrap().clone();
        // Get hints in random order, and allow user to unlock them as they play
        let mut rnd_hints = self.puzzle.hints.clone();
        rnd_hints.shuffle(&mut rand::thread_rng());
        self.rnd_hints = rnd_hints.clone();

        self.update_puzzle_meh();
    }

    fn convert_name_to_guess_format(&self, creature: &str) -> String {
        let guess = creature
            .chars()
            .map(|c| if c.is_alphabetic() { '_' } else { c })
            .collect();
        guess
    }

    fn fetch_greetings(&self) -> Vec<String> {
        include_str!("greetings.txt")
            .lines()
            .map(|lines| lines.trim().to_string())
            .collect()
    }

    // kinda shit
    fn reveal_guessed_letters(
        &self,
        input: &Vec<String>,
        guessed: &str,
        count_hints: usize,
    ) -> String {
        let mut inside_braces = false;
        let mut result = String::new();
        let mut good_old_fashioned_i = 1;
        for s in input {
            result.push_str("- ");
            for c in s.chars() {
                if c == '{' {
                    inside_braces = true;
                } else if c == '}' {
                    inside_braces = false;
                } else if inside_braces
                    && c.is_alphabetic()
                    && !guessed.contains(c.to_ascii_uppercase())
                {
                    result.push('_'); // Replace unguessed letters inside curly braces
                } else {
                    result.push(c); // Keep everything else as is
                }
            }
            result.push('\n');
            if good_old_fashioned_i >= count_hints {
                break;
            } else {
                good_old_fashioned_i += 1;
            }
        }

        result
    }
}

pub struct Paint {
    pub intro: String,
    pub status: String,
    pub answer_result: String,
}

impl Default for Paint {
    fn default() -> Self {
        Paint {
            intro: String::new(),
            status: String::new(),
            answer_result: String::new(),
        }
    }
}
