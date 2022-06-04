use crate::{
    board::{Board, Move},
    constants::DIRECTIONS,
    rock::PlayerRock,
    rules::RuleSet,
};

// * Static patterns

pub const FREE_THREE_DIRECT_PATTERN: [(i16, PlayerRock); 4] = [
    (-1, PlayerRock::None),
    (1, PlayerRock::Player),
    (2, PlayerRock::Player),
    (3, PlayerRock::None),
];

pub const FREE_THREE_DIRECT_CENTER_PATTERN: [(i16, PlayerRock); 4] = [
    (-2, PlayerRock::None),
    (-1, PlayerRock::Player),
    (1, PlayerRock::Player),
    (2, PlayerRock::None),
];

pub const FREE_THREE_SECONDARY_PATTERN: [(i16, PlayerRock); 5] = [
    (-1, PlayerRock::None),
    (1, PlayerRock::None),
    (2, PlayerRock::Player),
    (3, PlayerRock::Player),
    (4, PlayerRock::None),
];

pub const FREE_THREE_SECONDARY_CENTER_PATTERN: [(i16, PlayerRock); 5] = [
    (-2, PlayerRock::None),
    (-1, PlayerRock::Player),
    (1, PlayerRock::None),
    (2, PlayerRock::Player),
    (3, PlayerRock::None),
];

pub const FIVE_PATTERNS: [&[(i16, PlayerRock); 4]; 3] = [
    &[
        (1, PlayerRock::Player),
        (2, PlayerRock::Player),
        (3, PlayerRock::Player),
        (4, PlayerRock::Player),
    ],
    &[
        (-1, PlayerRock::Player),
        (1, PlayerRock::Player),
        (2, PlayerRock::Player),
        (3, PlayerRock::Player),
    ],
    &[
        (-2, PlayerRock::Player),
        (-1, PlayerRock::Player),
        (1, PlayerRock::Player),
        (2, PlayerRock::Player),
    ],
];

pub const UNDER_CAPTURE_PATTERNS: [[(i16, PlayerRock); 3]; 2] = [
    [
        (-1, PlayerRock::None),
        (1, PlayerRock::Player),
        (2, PlayerRock::Opponent),
    ],
    [
        (-1, PlayerRock::Opponent),
        (1, PlayerRock::Player),
        (2, PlayerRock::None),
    ],
];

pub const RECURSIVE_CAPTURE_PATTERN: &[(i16, PlayerRock); 3] = &[
    (-1, PlayerRock::Opponent),
    (1, PlayerRock::Player),
    (2, PlayerRock::Opponent),
];

pub const CAPTURE_PATTERN: &[(i16, PlayerRock); 3] = &[
    (1, PlayerRock::Opponent),
    (2, PlayerRock::Opponent),
    (3, PlayerRock::Player),
];

#[repr(u8)]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Category {
    FiveInRow,
    CapturedFiveInRow,
    KilledFive,
    LiveFour,
    KilledFour,
    DeadFour,
    KilledThree,
    BlockedCapture,
    LiveThree,
    CutThree,
    DeadThree,
    LiveTwo,
    DeadTwo,
}

#[derive(Default, Debug, Clone)]
pub struct PatternCount {
    pub five_in_row: u8,
    pub captured_five_in_row: u8,
    pub killed_five: u8,
    pub live_four: u8,
    pub killed_four: u8,
    pub dead_four: u8,
    pub killed_three: u8,
    pub blocked_capture: u8,
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
            13
        } else if self.killed_five > 0 {
            12
        } else if self.live_four > 0 {
            11
        } else if self.killed_four > 0 {
            10
        } else if self.killed_three > 0 {
            9
        } else if self.blocked_capture > 0 {
            8
        } else if self.dead_four > 0 {
            7
        } else if self.live_three > 0 {
            6
        } else if self.cut_three > 0 {
            5
        } else if self.captured_five_in_row > 0 {
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
    patterns: Vec<(Vec<(i16, PlayerRock)>, Category)>,
}

impl Default for Finder {
    fn default() -> Self {
        let patterns: Vec<(Vec<(i16, u8)>, Category)> = vec![
            // Five in a row
            // Only half of the patterns are required since it will check all directions
            // -- [1, 1, 1, 1, 1]
            (vec![(1, 1), (2, 1), (3, 1), (4, 1)], Category::FiveInRow),
            (vec![(-1, 1), (1, 1), (2, 1), (3, 1)], Category::FiveInRow),
            (vec![(-2, 1), (-1, 1), (1, 1), (2, 1)], Category::FiveInRow),
            // -- [1, 2, 2, 2, 2, 1]
            (
                vec![(1, 2), (2, 2), (3, 2), (4, 2), (5, 1)],
                Category::KilledFive,
            ),
            (
                vec![(-5, 1), (-4, 2), (-3, 2), (-2, 2), (-1, 2)],
                Category::KilledFive,
            ),
            // -- [2, 1, 2, 2, 2]
            // -- [2, 2, 2, 1, 2]
            (vec![(-1, 2), (1, 2), (2, 2), (3, 2)], Category::KilledFive),
            // -- [2, 2, 1, 2, 2]
            (vec![(-2, 2), (-1, 2), (1, 2), (2, 2)], Category::KilledFive),
            // -- [1, 2, 2, 2, 2]
            // -- [2, 2, 2, 2, 1]
            (vec![(1, 2), (2, 2), (3, 2), (4, 2)], Category::KilledFive),
            // -- [1, 2, 2, 2, 1]
            // -- [1, 2, 2, 2, 1]
            (vec![(1, 2), (2, 2), (3, 2), (4, 1)], Category::KilledFive),
            // -- [1, 2, 2, 2]
            // -- [2, 2, 2, 1]
            (vec![(1, 2), (2, 2), (3, 2)], Category::KilledFour),
            // -- [0, 1, 1, 0, 1, 1]
            (
                vec![(-1, 0), (1, 1), (2, 0), (3, 1), (4, 1)],
                Category::LiveFour,
            ),
            (
                vec![(-2, 0), (-1, 1), (1, 0), (2, 1), (3, 1)],
                Category::LiveFour,
            ),
            (
                vec![(-4, 0), (-3, 1), (-2, 1), (-1, 0), (1, 1), (2, 1)],
                Category::LiveFour,
            ),
            // -- [1, 1, 0, 1, 1, 0]
            (
                vec![(1, 1), (2, 0), (3, 1), (4, 1), (5, 0)],
                Category::LiveFour,
            ),
            (
                vec![(-1, 1), (1, 0), (2, 1), (3, 1), (4, 0)],
                Category::LiveFour,
            ),
            (
                vec![(-3, 1), (-2, 1), (-1, 0), (1, 1), (2, 0)],
                Category::LiveFour,
            ),
            (
                vec![(-4, 1), (-3, 1), (-2, 0), (-1, 1), (1, 0)],
                Category::LiveFour,
            ),
            // -- [0, 1, 1, 1, 1]
            // -- [1, 1, 1, 1, 0]
            (vec![(-1, 0), (1, 1), (2, 1), (3, 1)], Category::LiveFour),
            (vec![(-2, 0), (-1, 1), (1, 1), (2, 1)], Category::LiveFour),
            // -- [2, 1, 1, 1, 1, 0]
            // -- [0, 1, 1, 1, 1, 2]
            (
                vec![(-1, 2), (1, 1), (2, 1), (3, 1), (4, 0)],
                Category::LiveFour,
            ),
            (
                vec![(-2, 2), (-1, 1), (1, 1), (2, 1), (3, 0)],
                Category::LiveFour,
            ),
            (
                vec![(-3, 2), (-2, 1), (-1, 1), (1, 1), (2, 0)],
                Category::LiveFour,
            ),
            // -- [1, 0, 1, 1, 1]
            // -- [1, 1, 1, 0, 1]
            (vec![(1, 0), (2, 1), (3, 1), (4, 1)], Category::LiveFour),
            (vec![(-2, 1), (-1, 0), (1, 1), (2, 1)], Category::LiveFour),
            // -- [1, 1, 0, 1, 1]
            (vec![(1, 1), (2, 0), (3, 1), (4, 1)], Category::LiveFour),
            (vec![(-1, 1), (1, 0), (2, 1), (3, 1)], Category::LiveFour),
            // -- [2, 1, 2, 2]
            // -- [2, 2, 1, 2]
            (vec![(-1, 2), (1, 2), (2, 2)], Category::KilledFour),
            (vec![(-2, 2), (-1, 2), (1, 2)], Category::KilledFour),
            // -- [2, 0, 1, 1, 1, 0, 2]
            (
                vec![(-2, 2), (-1, 0), (1, 1), (2, 1), (3, 0), (4, 2)],
                Category::LiveThree,
            ),
            (
                vec![(-3, 2), (-2, 0), (-1, 1), (1, 1), (2, 0), (3, 2)],
                Category::LiveThree,
            ),
            // -- [2, 1, 1, 1]
            // -- [1, 1, 1, 2]
            (vec![(1, 1), (2, 1), (3, 2)], Category::BlockedCapture),
            // -- [0, 1, 1, 1, 0]
            (vec![(-1, 0), (1, 1), (2, 1), (3, 0)], Category::LiveThree),
            (vec![(-2, 0), (-1, 1), (1, 1), (2, 0)], Category::LiveThree),
            // -- [1, 1, 1]
            (vec![(1, 1), (2, 1)], Category::LiveThree),
            (vec![(-1, 1), (1, 1)], Category::LiveThree),
            // -- [1, 0, 1, 0, 1]
            (vec![(1, 0), (2, 1), (3, 0), (4, 1)], Category::LiveThree),
            (vec![(-2, 1), (-1, 0), (1, 0), (2, 1)], Category::LiveThree),
            // -- [1, 0, 1, 1]
            // -- [1, 1, 0, 1]
            (vec![(1, 0), (2, 1), (3, 1)], Category::LiveThree),
            (vec![(-2, 1), (-1, 0), (1, 1)], Category::LiveThree),
            // -- [1, 0, 0, 1, 1]
            // -- [1, 1, 0, 0, 1]
            (vec![(1, 0), (2, 0), (3, 1), (4, 1)], Category::LiveThree),
            (vec![(-3, 1), (-2, 0), (-1, 0), (1, 1)], Category::LiveThree),
            // -- [1, 0, 1, 1, 2]
            // -- [2, 1, 1, 0, 1]
            (vec![(1, 0), (2, 1), (3, 1), (4, 2)], Category::DeadThree),
            (vec![(-2, 1), (-1, 0), (1, 1), (2, 2)], Category::DeadThree),
            (vec![(-3, 1), (-2, 0), (-1, 1), (1, 2)], Category::DeadThree),
            // -- [2, 1, 0, 1, 1]
            (vec![(-1, 2), (1, 0), (2, 1), (3, 1)], Category::DeadThree),
            (vec![(-3, 2), (-2, 1), (-1, 0), (1, 1)], Category::DeadThree),
            (
                vec![(-4, 2), (-3, 1), (-2, 0), (-1, 1)],
                Category::DeadThree,
            ),
            // -- [1, 1, 0, 1, 2, 0]
            (
                vec![(1, 1), (2, 0), (3, 1), (4, 2), (5, 0)],
                Category::DeadThree,
            ),
            (
                vec![(-1, 1), (1, 0), (2, 1), (3, 2), (4, 0)],
                Category::DeadThree,
            ),
            (
                vec![(-3, 1), (-2, 1), (-1, 0), (1, 2), (2, 0)],
                Category::DeadThree,
            ),
            // -- [0, 1, 1, 1, 2]
            // -- [2, 1, 1, 1, 0]
            (vec![(-1, 0), (1, 1), (2, 1), (3, 2)], Category::DeadThree),
            (vec![(-2, 0), (-1, 1), (1, 1), (2, 2)], Category::DeadThree),
            (vec![(-3, 0), (-2, 1), (-1, 1), (1, 2)], Category::DeadThree),
            // -- [1, 0, 0, 1, 2]
            // -- [2, 1, 0, 0, 1]
            // (vec![(1, 0), (2, 0), (3, 1), (4, 2)], Pattern::LiveTwo),
            // (vec![(-1, 2), (1, 0), (2, 0), (3, 1)], Pattern::LiveTwo),
            // -- [2, 1, 0, 1]
            // -- [1, 0, 1, 2]
            // (vec![(-1, 2), (1, 0), (2, 1)], Pattern::LiveTwo),
            // (vec![(1, 0), (2, 1), (3, 2)], Pattern::LiveTwo),
            // -- [1, 0, 0, 1]
            // (vec![(1, 0), (2, 0), (3, 1)], Pattern::LiveTwo),
            // -- [0, 1, 1, 0]
            (vec![(-1, 0), (1, 1), (2, 0)], Category::LiveTwo),
            // -- [1, 0, 0, 0, 1]
            (vec![(1, 0), (2, 0), (3, 0), (4, 1)], Category::DeadTwo),
            // -- [1, 0, 1]
            // (vec![(1, 0), (2, 1)], Pattern::LiveTwo),
            // -- [1, 0, 0, 0, 1]
            // (vec![(1, 0), (2, 0), (3, 0), (4, 1)], Pattern::DeadTwo),
            // -- [0, 1, 1, 2]
            // -- [2, 1, 1, 0]
            (vec![(-1, 0), (1, 1), (2, 1)], Category::DeadTwo),
            (vec![(-1, 2), (1, 1), (2, 0)], Category::DeadTwo),
            // -- [2, 1, 1]
            // -- [1, 1, 2]
            // (vec![(-1, 2), (1, 1)], Pattern::DeadTwo),
            // (vec![(-2, 2), (-1, 1)], Pattern::DeadTwo),
            // -- [1, 1]
            (vec![(1, 1)], Category::DeadTwo),
        ];

        Finder {
            patterns: patterns
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

impl Finder {
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
    ) -> PatternCount {
        let mut pattern_count = PatternCount::default();
        let patterns = self.get_patterns_for_movement(rules, board, movement);
        for pattern in patterns {
            if pattern == Category::FiveInRow {
                pattern_count.five_in_row += 1;
            } else if pattern == Category::KilledFive {
                pattern_count.killed_five += 1;
            } else if pattern == Category::LiveFour {
                pattern_count.live_four += 1;
            } else if pattern == Category::KilledFour {
                pattern_count.killed_four += 1;
            } else if pattern == Category::DeadFour {
                pattern_count.dead_four += 1;
            } else if pattern == Category::BlockedCapture {
                pattern_count.blocked_capture += 1;
            } else if pattern == Category::KilledThree {
                pattern_count.killed_three += 1;
            } else if pattern == Category::CutThree {
                pattern_count.cut_three += 1;
            } else if pattern == Category::LiveThree {
                pattern_count.live_three += 1;
            } else if pattern == Category::CapturedFiveInRow {
                pattern_count.captured_five_in_row += 1;
            } else if pattern == Category::DeadThree {
                pattern_count.dead_three += 1;
            } else if pattern == Category::LiveTwo {
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
        if patterns.blocked_capture > 0 {
            score += 70000;
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
        if patterns.captured_five_in_row > 0 {
            score += 20000;
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
