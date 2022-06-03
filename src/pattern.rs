use crate::{
    board::{Board, Coordinates, Move},
    constants::{BOARD_SIZE, DIRECTIONS},
    player::Player,
    rock::Rock,
};

#[repr(u8)]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum PatternRock {
    None,
    Player,
    Opponent,
}

#[repr(u8)]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Pattern {
    FiveInRow,
    KilledFive,
    LiveFour,
    KilledFour,
    DeadFour,
    KilledThree,
    LiveThree,
    CutThree,
    DeadThree,
    LiveTwo,
    DeadTwo,
}

#[derive(Default, Debug, Clone)]
pub struct PatternCount {
    pub five_in_row: u8,
    pub killed_five: u8,
    pub live_four: u8,
    pub killed_four: u8,
    pub dead_four: u8,
    pub killed_three: u8,
    pub live_three: u8,
    pub cut_three: u8,
    pub dead_three: u8,
    pub live_two: u8,
    pub dead_two: u8,
}

impl PatternCount {
    // Order by which to sort the generated moves
    // Gives priority to moves that save the game or end the game
    pub fn best_pattern(&self) -> u8 {
        if self.five_in_row > 0 {
            11
        } else if self.killed_five > 0 {
            10
        } else if self.live_four > 0 {
            9
        } else if self.killed_four > 0 {
            8
        } else if self.killed_three > 0 {
            7
        } else if self.dead_four > 0 {
            6
        } else if self.live_three > 0 {
            5
        } else if self.cut_three > 0 {
            4
        } else if self.dead_three > 0 {
            3
        } else if self.live_two > 0 {
            2
        } else if self.dead_two > 0 {
            1
        } else {
            0
        }
    }
}

pub struct Finder {
    patterns: Vec<(Vec<(i16, PatternRock)>, Pattern)>,
}

impl Default for Finder {
    fn default() -> Self {
        let patterns: Vec<(Vec<(i16, PatternRock)>, Pattern)> = vec![
            // Five in a row
            // Only half of the patterns are required since it will check all directions
            // -- [1, 1, 1, 1, 1]
            (
                vec![
                    (1, PatternRock::Player),
                    (2, PatternRock::Player),
                    (3, PatternRock::Player),
                    (4, PatternRock::Player),
                ],
                Pattern::FiveInRow,
            ),
            (
                vec![
                    (-1, PatternRock::Player),
                    (1, PatternRock::Player),
                    (2, PatternRock::Player),
                    (3, PatternRock::Player),
                ],
                Pattern::FiveInRow,
            ),
            (
                vec![
                    (-2, PatternRock::Player),
                    (-1, PatternRock::Player),
                    (1, PatternRock::Player),
                    (2, PatternRock::Player),
                ],
                Pattern::FiveInRow,
            ),
            // -- [0, 1, 1, 1, 1]
            // -- [1, 1, 1, 1, 0]
            (
                vec![
                    (-1, PatternRock::None),
                    (1, PatternRock::Player),
                    (2, PatternRock::Player),
                    (3, PatternRock::Player),
                ],
                Pattern::LiveFour,
            ),
            (
                vec![
                    (-2, PatternRock::None),
                    (-1, PatternRock::Player),
                    (1, PatternRock::Player),
                    (2, PatternRock::Player),
                ],
                Pattern::LiveFour,
            ),
            // -- [2, 1, 1, 1, 1, 0]
            // -- [0, 1, 1, 1, 1, 2]
            (
                vec![
                    (-1, PatternRock::Opponent),
                    (1, PatternRock::Player),
                    (2, PatternRock::Player),
                    (3, PatternRock::Player),
                    (4, PatternRock::None),
                ],
                Pattern::LiveFour,
            ),
            (
                vec![
                    (-2, PatternRock::Opponent),
                    (-1, PatternRock::Player),
                    (1, PatternRock::Player),
                    (2, PatternRock::Player),
                    (3, PatternRock::None),
                ],
                Pattern::LiveFour,
            ),
            (
                vec![
                    (-3, PatternRock::Opponent),
                    (-2, PatternRock::Player),
                    (-1, PatternRock::Player),
                    (1, PatternRock::Player),
                    (2, PatternRock::None),
                ],
                Pattern::LiveFour,
            ),
            // -- [1, 0, 1, 1, 1]
            // -- [1, 1, 1, 0, 1]
            (
                vec![
                    (1, PatternRock::None),
                    (2, PatternRock::Player),
                    (3, PatternRock::Player),
                    (4, PatternRock::Player),
                ],
                Pattern::LiveFour,
            ),
            (
                vec![
                    (-2, PatternRock::Player),
                    (-1, PatternRock::None),
                    (1, PatternRock::Player),
                    (2, PatternRock::Player),
                ],
                Pattern::LiveFour,
            ),
            // -- [1, 1, 0, 1, 1]
            (
                vec![
                    (1, PatternRock::Player),
                    (2, PatternRock::None),
                    (3, PatternRock::Player),
                    (4, PatternRock::Player),
                ],
                Pattern::LiveFour,
            ),
            (
                vec![
                    (-1, PatternRock::Player),
                    (1, PatternRock::None),
                    (2, PatternRock::Player),
                    (3, PatternRock::Player),
                ],
                Pattern::LiveFour,
            ),
            // -- [0, 1, 1, 0, 1, 1]
            (
                vec![
                    (-1, PatternRock::None),
                    (1, PatternRock::Player),
                    (2, PatternRock::None),
                    (3, PatternRock::Player),
                    (4, PatternRock::Player),
                ],
                Pattern::LiveFour,
            ),
            (
                vec![
                    (-2, PatternRock::None),
                    (-1, PatternRock::Player),
                    (1, PatternRock::None),
                    (2, PatternRock::Player),
                    (3, PatternRock::Player),
                ],
                Pattern::LiveFour,
            ),
            (
                vec![
                    (-4, PatternRock::None),
                    (-3, PatternRock::Player),
                    (-2, PatternRock::Player),
                    (-1, PatternRock::None),
                    (1, PatternRock::Player),
                    (2, PatternRock::Player),
                ],
                Pattern::LiveFour,
            ),
            // -- [1, 1, 0, 1, 1, 0]
            (
                vec![
                    (1, PatternRock::Player),
                    (2, PatternRock::None),
                    (3, PatternRock::Player),
                    (4, PatternRock::Player),
                    (5, PatternRock::None),
                ],
                Pattern::LiveFour,
            ),
            (
                vec![
                    (-1, PatternRock::Player),
                    (1, PatternRock::None),
                    (2, PatternRock::Player),
                    (3, PatternRock::Player),
                    (4, PatternRock::None),
                ],
                Pattern::LiveFour,
            ),
            (
                vec![
                    (-3, PatternRock::Player),
                    (-2, PatternRock::Player),
                    (-1, PatternRock::None),
                    (1, PatternRock::Player),
                    (2, PatternRock::None),
                ],
                Pattern::LiveFour,
            ),
            (
                vec![
                    (-4, PatternRock::Player),
                    (-3, PatternRock::Player),
                    (-2, PatternRock::None),
                    (-1, PatternRock::Player),
                    (1, PatternRock::None),
                ],
                Pattern::LiveFour,
            ),
            // -- [2, 0, 1, 1, 1, 0, 2]
            (
                vec![
                    (-2, PatternRock::Opponent),
                    (-1, PatternRock::None),
                    (1, PatternRock::Player),
                    (2, PatternRock::Player),
                    (3, PatternRock::None),
                    (4, PatternRock::Opponent),
                ],
                Pattern::LiveThree,
            ),
            (
                vec![
                    (-3, PatternRock::Opponent),
                    (-2, PatternRock::None),
                    (-1, PatternRock::Player),
                    (1, PatternRock::Player),
                    (2, PatternRock::None),
                    (3, PatternRock::Opponent),
                ],
                Pattern::LiveThree,
            ),
            // -- [0, 1, 1, 1, 0]
            (
                vec![
                    (-1, PatternRock::None),
                    (1, PatternRock::Player),
                    (2, PatternRock::Player),
                    (3, PatternRock::None),
                ],
                Pattern::LiveThree,
            ),
            (
                vec![
                    (-2, PatternRock::None),
                    (-1, PatternRock::Player),
                    (1, PatternRock::Player),
                    (2, PatternRock::None),
                ],
                Pattern::LiveThree,
            ),
            // -- [1, 1, 1]
            (
                vec![(1, PatternRock::Player), (2, PatternRock::Player)],
                Pattern::LiveThree,
            ),
            (
                vec![(-1, PatternRock::Player), (1, PatternRock::Player)],
                Pattern::LiveThree,
            ),
            // -- [0, 1, 1, 1, 2]
            // -- [2, 1, 1, 1, 0]
            (
                vec![
                    (-1, PatternRock::None),
                    (1, PatternRock::Player),
                    (2, PatternRock::Player),
                    (3, PatternRock::Opponent),
                ],
                Pattern::LiveThree,
            ),
            (
                vec![
                    (-2, PatternRock::None),
                    (-1, PatternRock::Player),
                    (1, PatternRock::Player),
                    (2, PatternRock::Opponent),
                ],
                Pattern::LiveThree,
            ),
            (
                vec![
                    (-3, PatternRock::None),
                    (-2, PatternRock::Player),
                    (-1, PatternRock::Player),
                    (1, PatternRock::Opponent),
                ],
                Pattern::LiveThree,
            ),
            // -- [1, 0, 1, 0, 1]
            (
                vec![
                    (1, PatternRock::None),
                    (2, PatternRock::Player),
                    (3, PatternRock::None),
                    (4, PatternRock::Player),
                ],
                Pattern::LiveThree,
            ),
            (
                vec![
                    (-2, PatternRock::Player),
                    (-1, PatternRock::None),
                    (1, PatternRock::None),
                    (2, PatternRock::Player),
                ],
                Pattern::LiveThree,
            ),
            // -- [1, 0, 1, 1]
            // -- [1, 1, 0, 1]
            (
                vec![
                    (1, PatternRock::None),
                    (2, PatternRock::Player),
                    (3, PatternRock::Player),
                ],
                Pattern::LiveThree,
            ),
            (
                vec![
                    (-2, PatternRock::Player),
                    (-1, PatternRock::None),
                    (1, PatternRock::Player),
                ],
                Pattern::LiveThree,
            ),
            // -- [1, 0, 1, 1, 2]
            // -- [2, 1, 1, 0, 1]
            (
                vec![
                    (1, PatternRock::None),
                    (2, PatternRock::Player),
                    (3, PatternRock::Player),
                    (4, PatternRock::Opponent),
                ],
                Pattern::DeadThree,
            ),
            (
                vec![
                    (-2, PatternRock::Player),
                    (-1, PatternRock::None),
                    (1, PatternRock::Player),
                    (2, PatternRock::Opponent),
                ],
                Pattern::DeadThree,
            ),
            (
                vec![
                    (-3, PatternRock::Player),
                    (-2, PatternRock::None),
                    (-1, PatternRock::Player),
                    (1, PatternRock::Opponent),
                ],
                Pattern::DeadThree,
            ),
            // -- [2, 1, 0, 1, 1]
            (
                vec![
                    (-1, PatternRock::Opponent),
                    (1, PatternRock::None),
                    (2, PatternRock::Player),
                    (3, PatternRock::Player),
                ],
                Pattern::DeadThree,
            ),
            (
                vec![
                    (-3, PatternRock::Opponent),
                    (-2, PatternRock::Player),
                    (-1, PatternRock::None),
                    (1, PatternRock::Player),
                ],
                Pattern::DeadThree,
            ),
            (
                vec![
                    (-4, PatternRock::Opponent),
                    (-3, PatternRock::Player),
                    (-2, PatternRock::None),
                    (-1, PatternRock::Player),
                ],
                Pattern::DeadThree,
            ),
            // -- [1, 1, 0, 1, 2, 0]
            (
                vec![
                    (1, PatternRock::Player),
                    (2, PatternRock::None),
                    (3, PatternRock::Player),
                    (4, PatternRock::Opponent),
                    (5, PatternRock::None),
                ],
                Pattern::DeadThree,
            ),
            (
                vec![
                    (-1, PatternRock::Player),
                    (1, PatternRock::None),
                    (2, PatternRock::Player),
                    (3, PatternRock::Opponent),
                    (4, PatternRock::None),
                ],
                Pattern::DeadThree,
            ),
            (
                vec![
                    (-3, PatternRock::Player),
                    (-2, PatternRock::Player),
                    (-1, PatternRock::None),
                    (1, PatternRock::Opponent),
                    (2, PatternRock::None),
                ],
                Pattern::DeadThree,
            ),
            // -- [1, 0, 0, 1, 1]
            // -- [1, 1, 0, 0, 1]
            (
                vec![
                    (1, PatternRock::None),
                    (2, PatternRock::None),
                    (3, PatternRock::Player),
                    (4, PatternRock::Player),
                ],
                Pattern::DeadThree,
            ),
            (
                vec![
                    (-3, PatternRock::Player),
                    (-2, PatternRock::None),
                    (-1, PatternRock::None),
                    (1, PatternRock::Player),
                ],
                Pattern::DeadThree,
            ),
            // -- [0, 1, 1, 1, 2]
            // -- [2, 1, 1, 1, 0]
            (
                vec![
                    (-1, PatternRock::None),
                    (1, PatternRock::Player),
                    (2, PatternRock::Player),
                    (3, PatternRock::Opponent),
                ],
                Pattern::DeadThree,
            ),
            (
                vec![
                    (-2, PatternRock::None),
                    (-1, PatternRock::Player),
                    (1, PatternRock::Player),
                    (2, PatternRock::Opponent),
                ],
                Pattern::DeadThree,
            ),
            (
                vec![
                    (-3, PatternRock::None),
                    (-2, PatternRock::Player),
                    (-1, PatternRock::Player),
                    (1, PatternRock::Opponent),
                ],
                Pattern::DeadThree,
            ),
            // -- [1, 0, 0, 1, 2]
            // -- [2, 1, 0, 0, 1]
            (
                vec![
                    (1, PatternRock::None),
                    (2, PatternRock::None),
                    (3, PatternRock::Player),
                    (4, PatternRock::Opponent),
                ],
                Pattern::LiveTwo,
            ),
            // -- [2, 1, 0, 1]
            // -- [1, 0, 1, 2]
            (
                vec![
                    (-1, PatternRock::Opponent),
                    (1, PatternRock::None),
                    (2, PatternRock::Player),
                ],
                Pattern::LiveTwo,
            ),
            // -- [1, 0, 0, 1]
            (
                vec![
                    (1, PatternRock::None),
                    (2, PatternRock::None),
                    (3, PatternRock::Player),
                ],
                Pattern::LiveTwo,
            ),
            // -- [1, 0, 0, 0, 1]
            (
                vec![
                    (1, PatternRock::None),
                    (2, PatternRock::None),
                    (3, PatternRock::None),
                    (4, PatternRock::Player),
                ],
                Pattern::DeadTwo,
            ),
            // -- [1, 0, 1]
            (
                vec![(1, PatternRock::None), (2, PatternRock::Player)],
                Pattern::LiveTwo,
            ),
            // -- [1, 0, 0, 0, 1]
            (
                vec![
                    (1, PatternRock::None),
                    (2, PatternRock::None),
                    (3, PatternRock::None),
                    (4, PatternRock::Player),
                ],
                Pattern::DeadTwo,
            ),
            // -- [2, 1, 1]
            // -- [1, 1, 2]
            (
                vec![(-1, PatternRock::Opponent), (1, PatternRock::Player)],
                Pattern::DeadTwo,
            ),
            (
                vec![(-2, PatternRock::Opponent), (-1, PatternRock::Player)],
                Pattern::DeadTwo,
            ),
            // -- [1, 1]
            (vec![(1, PatternRock::Player)], Pattern::DeadTwo),
        ];

        Finder { patterns }
    }
}

impl Finder {
    // TODO "PatternRock" board
    #[allow(clippy::manual_range_contains)]
    pub fn check_pattern(
        &self,
        board: &Board,
        player: Player,
        coordinates: &Coordinates,
        direction: &(i16, i16),
        pattern: &[(i16, PatternRock)],
    ) -> bool {
        for (key, value) in pattern {
            let (check_x, check_y) = (
                coordinates.x + direction.0 * key,
                coordinates.y + direction.1 * key,
            );
            if check_x < 0 || check_x >= BOARD_SIZE || check_y < 0 || check_y >= BOARD_SIZE {
                return false;
            }
            let rock = board.get(check_x, check_y);
            let rock = if rock == Rock::None {
                &PatternRock::None
            } else if (rock == Rock::Black && player == Player::Black)
                || (rock == Rock::White && player == Player::White)
            {
                &PatternRock::Player
            } else {
                &PatternRock::Opponent
            };
            if rock != value {
                return false;
            }
        }
        true
    }

    // For each rocks on the board check all 8 directions to count all patterns
    // -- in a sliding window of 6 around the rock
    pub fn get_patterns_for_movement(&self, board: &Board, movement: &Move) -> Vec<Pattern> {
        let mut patterns: Vec<Pattern> = vec![];
        for direction in &DIRECTIONS {
            let mut best_pattern_index: Option<usize> = None;
            let mut best_pattern_value: Option<Pattern> = None;
            for (index, (pattern, category)) in self.patterns.iter().enumerate() {
                if self.check_pattern(
                    board,
                    movement.player,
                    &movement.coordinates,
                    direction,
                    pattern,
                ) {
                    let has_no_best_pattern = best_pattern_index.is_none();

                    if has_no_best_pattern || best_pattern_index.unwrap() > index {
                        best_pattern_index = Some(index);
                        best_pattern_value = Some(*category);
                    }
                }
            }
            // Save the pattern if there was one
            if let Some(best_pattern) = best_pattern_value {
                patterns.push(best_pattern);
            }
        }
        patterns
    }

    pub fn count_movement_patterns(&self, board: &Board, movement: &Move) -> PatternCount {
        let mut pattern_count = PatternCount::default();
        let patterns = self.get_patterns_for_movement(board, movement);
        for pattern in patterns {
            if pattern == Pattern::FiveInRow {
                pattern_count.five_in_row += 1;
            } else if pattern == Pattern::KilledFive {
                pattern_count.killed_five += 1;
            } else if pattern == Pattern::LiveFour {
                pattern_count.live_four += 1;
            } else if pattern == Pattern::KilledFour {
                pattern_count.killed_four += 1;
            } else if pattern == Pattern::DeadFour {
                pattern_count.dead_four += 1;
            } else if pattern == Pattern::KilledThree {
                pattern_count.killed_three += 1;
            } else if pattern == Pattern::CutThree {
                pattern_count.cut_three += 1;
            } else if pattern == Pattern::LiveThree {
                pattern_count.live_three += 1;
            } else if pattern == Pattern::DeadThree {
                pattern_count.dead_three += 1;
            } else if pattern == Pattern::LiveTwo {
                pattern_count.live_two += 1;
            } else {
                pattern_count.dead_two += 1;
            }
        }
        pattern_count
    }

    pub fn patterns_score(&self, patterns: &PatternCount) -> i32 {
        let mut score: i32 = 0;
        if patterns.five_in_row > 0 {
            score += 100000;
        }
        if patterns.killed_five > 0 {
            score += 99999;
        }
        if patterns.killed_four > 0 {
            score += 75000;
        }
        if patterns.killed_three > 0 {
            score += 60000;
        }
        if patterns.live_four > 0 {
            score += 50000;
        }
        if patterns.cut_three > 0 {
            score += 25000;
        }
        if patterns.live_three > 0 {
            score += 15000;
        }
        if patterns.live_three > 0 {
            score += 2000;
        }
        if patterns.dead_four > 0 {
            score += patterns.dead_four as i32 * 50;
        }
        if patterns.live_two > 0 {
            score += 200;
        }
        score
    }
}

lazy_static! {
    pub static ref PATTERN_FINDER: Finder = Finder::default();
}
