pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

pub enum Algorithm {
    Minimax,
    MinimaxAlphaBeta,
    Negamax,
}

pub struct Options {
    pub difficulty: Difficulty,
    pub algorithm: Algorithm,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            difficulty: Difficulty::Medium,
            algorithm: Algorithm::Negamax,
        }
    }
}
