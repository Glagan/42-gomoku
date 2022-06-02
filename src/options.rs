pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

pub struct Options {
    pub difficulty: Difficulty,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            difficulty: Difficulty::Medium,
        }
    }
}
