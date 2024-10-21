#[derive(Debug)]
pub struct Game {
    pub question: String,
    pub letters_guessed: String,
    pub health: usize,
    pub cash: usize,
    pub hints_unlocked: usize,
    pub bisect_cost: usize,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            question: String::new(),
            letters_guessed: String::new(),
            health: 100,
            cash: 20,
            hints_unlocked: 0,
            bisect_cost: 15,
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
