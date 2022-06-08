use crate::{
    board::{Board, Move},
    constants::DIRECTIONS,
    macros::coord,
    patterns::{Category, PatternCount, PATTERNS},
    player::Player,
    rock::PlayerRock,
    rules::RuleSet,
};

pub struct Heuristic {
    pub patterns: Vec<(Vec<(i16, PlayerRock)>, Category)>,
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
        for (pattern, category) in self.patterns.iter() {
            for direction in &DIRECTIONS {
                if board.check_pattern(&movement.coordinates, direction, pattern, movement.player) {
                    if rules.game_ending_capture {
                        // Check if it's a five in a row that it can't be captured
                        if category == &Category::FiveInRow {
                            let under_capture = board.pattern_is_under_capture(
                                rules,
                                &movement.coordinates,
                                direction,
                                pattern,
                                movement.player,
                            );
                            if under_capture {
                                patterns.push(Category::CapturedFiveInRow);
                            } else {
                                patterns.push(Category::FiveInRow);
                            }
                        }
                        // Avoid creating four in a row that are already under capture
                        else if category == &Category::OpenFour
                            || category == &Category::CloseFour
                        {
                            let under_capture = board.pattern_is_under_capture(
                                rules,
                                &movement.coordinates,
                                direction,
                                pattern,
                                movement.player,
                            );
                            if under_capture {
                                patterns.push(Category::CloseThree);
                            } else {
                                patterns.push(*category);
                            }
                        }
                        // Upgrade blocked captures to check if it unblock an open four or a five in a row
                        else if category == &Category::BlockedCapture {
                            // Check if either of the [1] that are unblocked are five in a row
                            // TODO Check that no other rocks in the pattern are under capture
                            let unblocked = coord!(
                                movement.coordinates.x + pattern[1].0 * direction.0,
                                movement.coordinates.y + pattern[1].0 * direction.1
                            );
                            if board.rock_is_five_in_a_row(&unblocked, movement.player) {
                                patterns.push(Category::FiveInRow);
                                continue;
                            }
                            let unblocked = coord!(
                                movement.coordinates.x + 2 * pattern[1].0 * direction.0,
                                movement.coordinates.y + 2 * pattern[1].0 * direction.1
                            );
                            if board.rock_is_five_in_a_row(&unblocked, movement.player) {
                                patterns.push(Category::FiveInRow);
                                continue;
                            }
                            patterns.push(Category::BlockedCapture);
                        }
                        // Increase capture score that break an OpenFour or more to KillFour
                        else if category == &Category::CreateCapture {
                            // Check if the capture is legal first
                            let capture_from = coord!(
                                movement.coordinates.x + pattern[1].0 * direction.0,
                                movement.coordinates.y + pattern[1].0 * direction.1
                            );
                            if !board.is_move_legal(
                                rules,
                                &Move {
                                    player: movement.player,
                                    coordinates: capture_from,
                                },
                            ) {
                                continue;
                            }
                            // Check if the opponent pattern in the capture is an open four or more
                            let blocked = coord!(
                                movement.coordinates.x + pattern[1].0 * direction.0,
                                movement.coordinates.y + pattern[1].0 * direction.1
                            );
                            if board.rock_is_five_in_a_row(&blocked, movement.player.opponent()) {
                                patterns.push(Category::KillFour);
                                continue;
                            }
                            let blocked = coord!(
                                movement.coordinates.x + 2 * pattern[1].0 * direction.0,
                                movement.coordinates.y + 2 * pattern[1].0 * direction.1
                            );
                            if board.rock_is_five_in_a_row(&blocked, movement.player.opponent()) {
                                patterns.push(Category::KillFour);
                                continue;
                            }
                            patterns.push(Category::BlockedCapture);
                        }
                        // TODO downgrade KillFour that are under capture
                        else {
                            patterns.push(*category);
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
        } else if patterns.open_four > 0 {
            return i32::max_value() - 2;
        }
        // Count good patterns that were created
        let mut score: i32 = 0;
        if patterns.reduce_three > 0 {
            score += 100001;
        }
        if patterns.inc_captures > 2 {
            score += 40000
        }
        if patterns.kill_three > 0 {
            score += 50001;
        }
        if patterns.close_four > 0 {
            score += 40000;
        }
        if patterns.open_three > 0 {
            score += 20000;
        }
        if patterns.inc_captures > 0 {
            score += 10000;
        }
        if patterns.blocked_capture > 0 {
            score += 10000;
        }
        if patterns.captured_five_in_row > 0 {
            score += 5000;
        }
        if patterns.close_three > 0 {
            score += 4000;
        }
        if patterns.open_two > 0 {
            score += 500;
        }
        if patterns.reduce_two > 0 {
            score += 300;
        }
        if patterns.close_two > 0 {
            score += 100;
        }
        score
    }
}

lazy_static! {
    pub static ref HEURISTIC: Heuristic = Heuristic::default();
}
