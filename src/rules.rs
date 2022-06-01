#[derive(Debug, Copy, Clone)]
pub struct RuleSet {
    pub capture: bool,
    pub game_ending_capture: bool,
    pub no_double_three: bool,
}

impl Default for RuleSet {
    fn default() -> RuleSet {
        RuleSet {
            capture: true,
            game_ending_capture: true,
            no_double_three: true,
        }
    }
}
