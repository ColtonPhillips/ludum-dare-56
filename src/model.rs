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
    PlayerInput(),
    WinPuzzle(),
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
            hints_unlocked: 0,
            bisect_cost: 15,
            rnd_greeting: String::new(),
            rnd_hints: vec![],
            rnd_hint: String::new(),
            result: String::new(),
            guess: String::new(),
        }
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
