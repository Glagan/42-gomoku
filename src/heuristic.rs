use crate::{
    board::{Board, Move},
    constants::DIRECTIONS,
    patterns::{Category, PatternCount, PATTERNS},
    player::Player,
    rock::PlayerRock,
    rules::RuleSet,
};

pub struct Heuristic {
    patterns: Vec<(Vec<(i16, PlayerRock)>, Category)>,
}

impl Default for Heuristic {
    fn default() -> Self {
        Heuristic {
            // Convert the simple [0, 1, 2] patterns to use the PlayerRock enum
            patterns: PATTERNS
                .iter()
                .map(|(pattern, category)| {
                    (
                        pattern
                            .iter()
                            .map(|(mov, expected)| {
                                (
                                    *mov,
                                    if expected == &0 {
                                        PlayerRock::None
                                    } else if expected == &1 {
                                        PlayerRock::Player
                                    } else {
                                        PlayerRock::Opponent
                                    },
                                )
                            })
                            .collect::<Vec<(i16, PlayerRock)>>(),
                        *category,
                    )
                })
                .collect::<Vec<(Vec<(i16, PlayerRock)>, Category)>>(),
        }
    }
}

impl Heuristic {
    // Collect all patterns that the rock placed by the movement created, in all directions
    pub fn get_patterns_for_movement(
        &self,
        rules: &RuleSet,
        board: &Board,
        movement: &Move,
    ) -> Vec<Category> {
        let mut patterns: Vec<Category> = vec![];
        for direction in &DIRECTIONS {
            for (pattern, category) in self.patterns.iter() {
                if board.check_pattern(&movement.coordinates, direction, pattern, movement.player) {
                    // Check if it's a five in a row that it can't be captured
                    if category == &Category::FiveInRow && rules.game_ending_capture {
                        let is_under_capture = board.five_in_a_row_is_under_capture(
                            rules,
                            &movement.coordinates,
                            direction,
                            pattern,
                            movement.player,
                        );
                        if is_under_capture {
                            patterns.push(Category::CapturedFiveInRow);
                        } else {
                            patterns.push(Category::FiveInRow);
                        }
                    } else {
                        patterns.push(*category);
                    }
                    // Since patterns are sorted by their priority,
                    // -- if a pattern match it's the best one
                    // break; // next direction
                }
            }
        }
        patterns
    }

    pub fn count_movement_patterns(
        &self,
        rules: &RuleSet,
        board: &Board,
        movement: &Move,
        captures: u8,
    ) -> PatternCount {
        let patterns = self.get_patterns_for_movement(rules, board, movement);
        let mut pattern_count = PatternCount::from_patterns(&patterns);
        pattern_count.total_captures = if movement.player == Player::Black {
            board.black.captures
        } else {
            board.white.captures
        };
        pattern_count.inc_captures = captures;
        pattern_count
    }

    pub fn patterns_score(&self, patterns: &PatternCount) -> i32 {
        // Return maximum value for the best and worst patterns
        if patterns.total_captures >= 10 || patterns.five_in_row > 0 {
            return i32::max_value();
        } else if patterns.kill_four > 0 {
            return i32::max_value() - 1;
        }
        // Count good patterns that were created
        let mut score: i32 = 0;
        if patterns.open_four > 0 {
            score += 50000;
        }
        if patterns.inc_captures > 2 {
            score += 40000
        }
        if patterns.reduce_three > 0 {
            score += 40000;
        }
        if patterns.inc_captures > 0 {
            score += 30000;
        }
        if patterns.close_four > 0 {
            score += 30000;
        }
        if patterns.open_three > 0 {
            score += 20000;
        }
        if patterns.kill_three > 0 {
            score += 10000;
        }
        if patterns.blocked_capture > 0 {
            score += 10000;
        }
        // if patterns.captured_five_in_row > 0 {
        //     score += 5000;
        // }
        if patterns.close_three > 0 {
            score += 4000;
        }
        if patterns.open_two > 0 {
            score += 3000;
        }
        if patterns.reduce_two > 0 {
            score += 3000;
        }
        if patterns.close_two > 0 {
            score += 1000;
        }
        score
    }
}

lazy_static! {
    pub static ref HEURISTIC: Heuristic = Heuristic::default();
}
