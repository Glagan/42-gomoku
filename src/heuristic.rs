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
                    if rules.capture {
                        // Increase capture score that break an OpenFour or more to KillFour
                        // -- if the capture rule is enabled
                        if category == &Category::CreateCapture {
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
                            if board.rock_is_four_or_more(&blocked, movement.player.opponent()) > 0
                            {
                                patterns.push(Category::KillFour);
                                continue;
                            }
                            let blocked = coord!(
                                movement.coordinates.x + 2 * pattern[1].0 * direction.0,
                                movement.coordinates.y + 2 * pattern[1].0 * direction.1
                            );
                            if board.rock_is_four_or_more(&blocked, movement.player.opponent()) > 0
                            {
                                patterns.push(Category::KillFour);
                                continue;
                            }
                            // Upgrade CreateCapture to CloseFour if captures >= 8
                            // -- it's a forced move for the opponent
                            let captures = if movement.player == Player::White {
                                board.white.captures
                            } else {
                                board.black.captures
                            };
                            if captures >= 8 {
                                patterns.push(Category::CloseFour);
                                continue;
                            }
                            patterns.push(Category::CreateCapture);
                        }
                        // Upgrade blocked captures to check if it unblock an open four or a five in a row
                        else if category == &Category::BlockedCapture {
                            // Check if the opponent has already 8 or more captures
                            // -- Update to KillFour since ignoring it result in a loss
                            let captures = if movement.player == Player::White {
                                board.black.captures
                            } else {
                                board.white.captures
                            };
                            if captures >= 8 {
                                // println!(
                                //     "movement on {}x{} for {:#?} upgraded to KillFour\n{}",
                                //     movement.coordinates.x,
                                //     movement.coordinates.y,
                                //     movement.player,
                                //     board
                                // );
                                patterns.push(Category::KillFour);
                                continue;
                            }
                            // Check if either of the [1] that are unblocked are five in a row
                            // ? Check that no other rocks in the pattern are under capture
                            // First rock
                            let unblocked = coord!(
                                movement.coordinates.x + pattern[1].0 * direction.0,
                                movement.coordinates.y + pattern[1].0 * direction.1
                            );
                            let is_four_or_more =
                                board.rock_is_four_or_more(&unblocked, movement.player);
                            if is_four_or_more > 0 {
                                patterns.push(if is_four_or_more == 2 {
                                    Category::FiveInRow
                                } else {
                                    Category::OpenFour
                                });
                                continue;
                            }
                            // Second rock
                            let unblocked = coord!(
                                movement.coordinates.x + 2 * pattern[1].0 * direction.0,
                                movement.coordinates.y + 2 * pattern[1].0 * direction.1
                            );
                            let is_four_or_more =
                                board.rock_is_four_or_more(&unblocked, movement.player);
                            if is_four_or_more > 0 {
                                patterns.push(if is_four_or_more == 2 {
                                    Category::FiveInRow
                                } else {
                                    Category::OpenFour
                                });
                                continue;
                            }
                            patterns.push(Category::BlockedCapture);
                        }

                        // Specific upgrades and downgrades for the game ending capture rule
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
                            // ? Downgrade KillFour that are under capture
                            // ? -- Not necessary since OpenFour under capture
                            // ? -- are seen by the opponent and can be uncaptured
                            else {
                                patterns.push(*category);
                            }
                        } else {
                            patterns.push(*category);
                        }
                    } else {
                        patterns.push(*category);
                    }
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
        if movement.player == Player::Black {
            pattern_count.total_captures = board.black.captures;
            pattern_count.opponent_captures = board.white.captures;
        } else {
            pattern_count.total_captures = board.white.captures;
            pattern_count.opponent_captures = board.black.captures;
        }
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
        } else if patterns.reveal_capture > 0 && patterns.opponent_captures >= 8 {
            return 0;
        }
        // Count good patterns that were created
        let mut score: i32 = 0;
        if patterns.reduce_three > 0 {
            score += 50001;
        }
        if patterns.inc_captures > 2 {
            let total_bonus: i32 =
                (patterns.total_captures.max(patterns.inc_captures) - patterns.inc_captures).into();
            score += 20000 * patterns.inc_captures as i32 + total_bonus * 10000;
        }
        if patterns.close_four > 0 {
            score += 40000;
        }
        if patterns.kill_three > 0 {
            score += 30001;
        }
        if patterns.open_three > 0 {
            score += 30000;
        }
        if patterns.inc_captures > 0 {
            let total_bonus =
                (patterns.total_captures.max(patterns.inc_captures) - patterns.inc_captures) as i32;
            score += 10000 * patterns.inc_captures as i32 + total_bonus * 10000;
        }
        if patterns.blocked_capture > 0 || patterns.created_captures > 0 {
            score += 15001;
        }
        if patterns.captured_five_in_row > 0 {
            score += 5000;
        }
        if patterns.reveal_capture > 0 {
            score = i32::max(score - 5000, 0);
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

    pub fn movement_score(
        &self,
        rules: &RuleSet,
        board: &Board,
        movement: &Move,
        captures: u8,
    ) -> i32 {
        let patterns = self.count_movement_patterns(rules, board, movement, captures);
        self.patterns_score(&patterns)
    }
}

lazy_static! {
    pub static ref HEURISTIC: Heuristic = Heuristic::default();
}
