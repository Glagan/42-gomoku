use crate::{
    board::{Board, Coordinates, Move},
    constants::{BOARD_SIZE, DIRECTIONS},
    macros::coord,
    player::Player,
    rock::Rock,
};
use fixed_vec_deque::FixedVecDeque;

#[repr(u8)]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Pattern {
    FiveInRow = 0,
    LiveFour = 1,
    DeadFour = 2,
    LiveThree = 3,
    DeadThree = 4,
    LiveTwo = 5,
    DeadTwo = 6,
}

#[derive(Debug, Default)]
pub struct PatternCount {
    pub five_in_row: u8,
    pub live_four: u8,
    pub dead_four: u8,
    pub live_three: u8,
    pub dead_three: u8,
    pub live_two: u8,
    pub dead_two: u8,
}

pub struct Finder {
    patterns: Vec<(Vec<u8>, u8, Pattern)>,
}

impl Default for Finder {
    fn default() -> Self {
        let patterns: Vec<(Vec<u8>, u8, Pattern)> = vec![
            (vec![1, 1, 1, 1, 1], 5, Pattern::FiveInRow),
            // 2x1
            (vec![0, 1, 1, 1, 1], 5, Pattern::LiveFour),
            (vec![1, 1, 1, 1, 0], 5, Pattern::LiveFour),
            // 3x1
            (vec![2, 1, 1, 1, 1, 0], 6, Pattern::LiveFour),
            (vec![0, 1, 1, 1, 1, 2], 6, Pattern::LiveFour),
            // 4x1
            (vec![1, 0, 1, 1, 1], 5, Pattern::LiveFour),
            (vec![1, 1, 0, 1, 1], 5, Pattern::LiveFour),
            (vec![1, 1, 1, 0, 1], 5, Pattern::LiveFour),
            // 5x1
            // (vec![0, 1, 1, 0, 1, 1], 6, PatternCategory::LiveFour),
            // (vec![1, 1, 0, 1, 1, 0], 6, PatternCategory::LiveFour),
            // 6x2
            (vec![2, 0, 1, 1, 1, 0, 2], 7, Pattern::LiveThree),
            // 6x1
            (vec![0, 1, 1, 1, 0], 5, Pattern::LiveThree),
            // (vec![1, 1, 1], 3, PatternCategory::LiveThree),
            // 1x2
            (vec![0, 1, 1, 1, 2], 5, Pattern::LiveThree),
            (vec![2, 1, 1, 1, 0], 5, Pattern::LiveThree),
            // 5x2
            (vec![1, 0, 1, 0, 1], 5, Pattern::LiveThree),
            // 7x1
            (vec![1, 0, 1, 1], 4, Pattern::LiveThree),
            (vec![1, 1, 0, 1], 4, Pattern::LiveThree),
            // 2x2
            (vec![1, 0, 1, 1, 2], 5, Pattern::DeadThree),
            (vec![2, 1, 1, 0, 1], 5, Pattern::DeadThree),
            // 3x2
            (vec![2, 1, 0, 1, 1], 5, Pattern::DeadThree),
            (vec![1, 1, 0, 1, 2, 0], 6, Pattern::DeadThree),
            // 4x2
            (vec![1, 0, 0, 1, 1], 5, Pattern::DeadThree),
            (vec![1, 1, 0, 0, 1], 5, Pattern::DeadThree),
            // (vec![0, 1, 1, 1, 2], 5, PatternCategory::DeadThree),
            // (vec![2, 1, 1, 1, 0], 5, PatternCategory::DeadThree),
            // 5x3
            (vec![1, 0, 0, 1, 2], 5, Pattern::LiveTwo),
            (vec![2, 1, 0, 0, 1], 5, Pattern::LiveTwo),
            // 4x3
            (vec![1, 0, 1, 2], 4, Pattern::LiveTwo),
            (vec![2, 1, 0, 1], 4, Pattern::LiveTwo),
            // 2x3
            (vec![1, 0, 0, 1], 4, Pattern::LiveTwo),
            // 1x3
            (vec![1, 0, 1], 3, Pattern::LiveTwo),
            // 7x2
            (vec![1, 0, 0, 0, 1], 5, Pattern::DeadTwo),
            // 3x3
            (vec![1, 1, 2], 3, Pattern::DeadTwo),
            (vec![2, 1, 1], 3, Pattern::DeadTwo),
            // 6x3
            // (vec![1, 1], 2, PatternCategory::DeadTwo),
        ];

        Finder { patterns }
    }
}

impl Finder {
    pub fn rock_to_pattern_rock(board: &Board, coordinates: &Coordinates, player: Player) -> u8 {
        let rock = board.get(coordinates.x, coordinates.y);
        if rock == Rock::None {
            0
        } else if (rock == Rock::Black && player == Player::Black)
            || (rock == Rock::White && player == Player::White)
        {
            1
        } else {
            2
        }
    }

    pub fn best_pattern_for_rock(
        &self,
        board: &Board,
        coordinates: &Coordinates,
    ) -> Option<Pattern> {
        let mut best_pattern: Option<Pattern> = None;
        let (x, y) = (coordinates.x, coordinates.y);
        let rock = board.get(x, y);
        if rock == Rock::None {
            let player = if rock == Rock::Black {
                Player::Black
            } else {
                Player::White
            };
            let mut best_pattern_index: Option<usize> = None;
            // Sliding window of 7 (patterns length)
            let mut buf = FixedVecDeque::<[u8; 7]>::new();
            for (dir_x, dir_y) in DIRECTIONS {
                // Initialize to -7 so the first 7 elements
                // -- can be set and the last one is the initial rock
                let mut length = 0;
                // from [x x x x x x x] ? ? ? ? ? ?  I ? ? ? ? ? ?
                // to    x x x x x x x  ? ? ? ? ? ? [I ? ? ? ? ? ?]
                let mut mov_x = dir_x * -7;
                let mut mov_y = dir_y * -7;
                for _ in 0..13 {
                    let new_coords = coord!(x + mov_x, y + mov_y);
                    // Check Board boundaries
                    if new_coords.x >= 0
                        && new_coords.y >= 0
                        && new_coords.x < BOARD_SIZE
                        && new_coords.y < BOARD_SIZE
                    {
                        *buf.push_back() = if new_coords.x == x && new_coords.y == y {
                            1
                        } else {
                            Finder::rock_to_pattern_rock(board, &new_coords, player)
                        };
                        length += 1;
                        if length >= 7 && buf.iter().filter(|rock| *rock == &1).count() >= 2 {
                            let has_best_pattern = best_pattern_index.is_some();
                            let has_no_best_pattern = best_pattern_index.is_none();
                            if let Some((index, (_, _, category))) =
                                self.patterns.iter().enumerate().find(
                                    |&(index, (pattern, length, _))| {
                                        if has_best_pattern && best_pattern_index.unwrap() < index {
                                            return false;
                                        }
                                        let mut i: u8 = 0;
                                        for value in &buf {
                                            if *value == pattern[i as usize] {
                                                i += 1;
                                                if i == *length {
                                                    return true;
                                                }
                                            } else {
                                                i = 0;
                                            }
                                        }
                                        i == *length
                                    },
                                )
                            {
                                // println!("Found pattern {:#?} in {:#?}", category, buf);
                                if has_no_best_pattern || best_pattern_index.unwrap() > index {
                                    best_pattern_index = Some(index);
                                    best_pattern = Some(*category);
                                    if category == &Pattern::FiveInRow {
                                        return best_pattern;
                                    }
                                }
                            }
                        }
                    }
                    mov_x += dir_x;
                    mov_y += dir_y;
                }
            }
        }
        best_pattern
    }

    // For each rocks on the board check all 8 directions to count all patterns
    // -- in a sliding window of 6 around the rock
    pub fn get_patterns_for_movement(&self, board: &Board, movement: &Move) -> Vec<Pattern> {
        let mut patterns: Vec<Pattern> = vec![];
        let player = movement.player;
        // Sliding window of 6 (patterns length)
        let mut buf = FixedVecDeque::<[u8; 7]>::new();
        // Iterate trough each rocks on the board
        let (x, y) = (movement.coordinates.x, movement.coordinates.y);
        for (dir_x, dir_y) in DIRECTIONS {
            // Initialize to -7 so the first 7 elements
            // -- can be set and the last one is the initial rock
            let mut length = 0;
            let mut best_pattern_index: Option<usize> = None;
            let mut best_pattern_value: Option<Pattern> = None;
            // from [x x x x x x x] ? ? ? ? ? ?  I ? ? ? ? ? ?
            // to    x x x x x x x  ? ? ? ? ? ? [I ? ? ? ? ? ?]
            let mut mov_x = dir_x * -7;
            let mut mov_y = dir_y * -7;
            for _ in 0..13 {
                let new_coords = coord!(x + mov_x, y + mov_y);
                // Check Board boundaries
                if new_coords.x >= 0
                    && new_coords.y >= 0
                    && new_coords.x < BOARD_SIZE
                    && new_coords.y < BOARD_SIZE
                {
                    *buf.push_back() = Finder::rock_to_pattern_rock(board, &new_coords, player);
                    length += 1;
                    if length >= 7 && buf.iter().filter(|rock| *rock == &1).count() >= 2 {
                        let has_best_pattern = best_pattern_index.is_some();
                        let has_no_best_pattern = best_pattern_index.is_none();
                        if let Some((index, (_, _, category))) =
                            self.patterns.iter().enumerate().find(
                                |&(index, (pattern, length, _))| {
                                    if has_best_pattern && best_pattern_index.unwrap() < index {
                                        return false;
                                    }
                                    let mut i: u8 = 0;
                                    for value in &buf {
                                        if *value == pattern[i as usize] {
                                            i += 1;
                                            if i == *length {
                                                return true;
                                            }
                                        } else {
                                            i = 0;
                                        }
                                    }
                                    i == *length
                                },
                            )
                        {
                            if has_no_best_pattern || best_pattern_index.unwrap() > index {
                                best_pattern_index = Some(index);
                                best_pattern_value = Some(*category);
                            }
                        }
                    }
                }
                mov_x += dir_x;
                mov_y += dir_y;
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
            } else if pattern == Pattern::LiveFour {
                pattern_count.live_four += 1;
            } else if pattern == Pattern::DeadFour {
                pattern_count.dead_four += 1;
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

    // TODO
    pub fn movement_patterns_score(&self, patterns: &PatternCount) -> i32 {
        let mut score: i32 = 0;
        if patterns.five_in_row > 0 {
            score += 100000;
        }
        // if other_patterns.dead_four > 0 {
        //     score += 50000;
        // }
        if patterns.live_four > 0 {
            score += 25000;
        }
        if patterns.live_three >= 1 {
            score += 15000;
        }
        // if other_patterns.dead_three >= 1 {
        //     score += 8000;
        // }
        /* if patterns.live_three + other_patterns.dead_three >= 2 {
            score += 5000;
        } else */
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

    pub fn movement_score(&self, board: &Board, movement: &Move) -> i32 {
        let patterns = self.count_movement_patterns(board, movement);
        self.movement_patterns_score(&patterns)
    }
}

lazy_static! {
    pub static ref PATTERN_FINDER: Finder = Finder::default();
}
