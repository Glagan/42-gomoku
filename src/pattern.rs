use crate::{
    board::{Board, Pawn, BOARD_SIZE, DIRECTIONS},
    player::Player,
};
use fixed_vec_deque::FixedVecDeque;

#[repr(u8)]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum PatternCategory {
    FiveInRow = 0,
    LiveFour = 1,
    DeadFour = 2,
    LiveThree = 3,
    DeadThree = 4,
    LiveTwo = 5,
    DeadTwo = 6,
}

pub struct PatternCount {
    pub five_in_row: usize,
    pub live_four: usize,
    pub dead_four: usize,
    pub live_three: usize,
    pub dead_three: usize,
    pub live_two: usize,
    pub dead_two: usize,
}

impl Default for PatternCount {
    fn default() -> PatternCount {
        PatternCount {
            five_in_row: 0,
            live_four: 0,
            dead_four: 0,
            live_three: 0,
            dead_three: 0,
            live_two: 0,
            dead_two: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pattern {
    pub pieces: Vec<usize>,
    pub category: PatternCategory,
}

pub struct Finder {
    patterns: Vec<(Vec<u8>, u8, PatternCategory)>,
}

impl Default for Finder {
    fn default() -> Self {
        let mut patterns: Vec<(Vec<u8>, u8, PatternCategory)> = vec![];

        patterns.push((vec![1, 1, 1, 1, 1], 5, PatternCategory::FiveInRow));
        // 2x1
        patterns.push((vec![0, 1, 1, 1, 1], 5, PatternCategory::LiveFour));
        patterns.push((vec![1, 1, 1, 1, 0], 5, PatternCategory::LiveFour));
        // 3x1
        patterns.push((vec![2, 1, 1, 1, 1, 0], 6, PatternCategory::LiveFour));
        patterns.push((vec![0, 1, 1, 1, 1, 2], 6, PatternCategory::LiveFour));
        // 4x1
        patterns.push((vec![1, 0, 1, 1, 1], 5, PatternCategory::LiveFour));
        patterns.push((vec![1, 1, 0, 1, 1], 5, PatternCategory::LiveFour));
        patterns.push((vec![1, 1, 1, 0, 1], 5, PatternCategory::LiveFour));
        // 5x1
        // patterns.push((vec![0, 1, 1, 0, 1, 1], 6, PatternCategory::LiveFour));
        // patterns.push((vec![1, 1, 0, 1, 1, 0], 6, PatternCategory::LiveFour));
        // 6x2
        patterns.push((vec![2, 0, 1, 1, 1, 0, 2], 7, PatternCategory::LiveThree));
        // 6x1
        patterns.push((vec![0, 1, 1, 1, 0], 5, PatternCategory::LiveThree));
        // patterns.push((vec![1, 1, 1], 3, PatternCategory::LiveThree));
        // 1x2
        patterns.push((vec![0, 1, 1, 1, 2], 5, PatternCategory::LiveThree));
        patterns.push((vec![2, 1, 1, 1, 0], 5, PatternCategory::LiveThree));
        // 5x2
        patterns.push((vec![1, 0, 1, 0, 1], 5, PatternCategory::LiveThree));
        // 7x1
        patterns.push((vec![1, 0, 1, 1], 4, PatternCategory::LiveThree));
        patterns.push((vec![1, 1, 0, 1], 4, PatternCategory::LiveThree));
        // 2x2
        patterns.push((vec![1, 0, 1, 1, 2], 5, PatternCategory::DeadThree));
        patterns.push((vec![2, 1, 1, 0, 1], 5, PatternCategory::DeadThree));
        // 3x2
        patterns.push((vec![2, 1, 0, 1, 1], 5, PatternCategory::DeadThree));
        patterns.push((vec![1, 1, 0, 1, 2, 0], 6, PatternCategory::DeadThree));
        // 4x2
        patterns.push((vec![1, 0, 0, 1, 1], 5, PatternCategory::DeadThree));
        patterns.push((vec![1, 1, 0, 0, 1], 5, PatternCategory::DeadThree));
        // patterns.push((vec![0, 1, 1, 1, 2], 5, PatternCategory::DeadThree));
        // patterns.push((vec![2, 1, 1, 1, 0], 5, PatternCategory::DeadThree));
        // 5x3
        patterns.push((vec![1, 0, 0, 1, 2], 5, PatternCategory::LiveTwo));
        patterns.push((vec![2, 1, 0, 0, 1], 5, PatternCategory::LiveTwo));
        // 4x3
        patterns.push((vec![1, 0, 1, 2], 4, PatternCategory::LiveTwo));
        patterns.push((vec![2, 1, 0, 1], 4, PatternCategory::LiveTwo));
        // 2x3
        patterns.push((vec![1, 0, 0, 1], 4, PatternCategory::LiveTwo));
        // 1x3
        patterns.push((vec![1, 0, 1], 3, PatternCategory::LiveTwo));
        // 7x2
        patterns.push((vec![1, 0, 0, 0, 1], 5, PatternCategory::DeadTwo));
        // 3x3
        patterns.push((vec![1, 1, 2], 3, PatternCategory::DeadTwo));
        patterns.push((vec![2, 1, 1], 3, PatternCategory::DeadTwo));
        // 6x3
        // patterns.push((vec![1, 1], 2, PatternCategory::DeadTwo));

        Finder { patterns }
    }
}

impl Finder {
    pub fn pawn_to_pattern_pawn(board: &Board, x: usize, y: usize, player: &Player) -> u8 {
        if let Some(pawn) = board.get(x, y) {
            if pawn == Pawn::None {
                0
            } else if (pawn == Pawn::Black && *player == Player::Black)
                || (pawn == Pawn::White && *player == Player::White)
            {
                1
            } else {
                2
            }
        } else {
            0
        }
    }

    pub fn best_pattern_for_rock(
        &self,
        board: &Board,
        rock_index: usize,
    ) -> Option<PatternCategory> {
        let mut best_pattern: Option<PatternCategory> = None;
        let (x, y) = Board::index_to_coordinates(rock_index);
        let rock = board.get(x, y);
        if let Some(rock) = rock {
            if rock == Pawn::None {
                let player = if rock == Pawn::Black {
                    &Player::Black
                } else {
                    &Player::White
                };
                let (x, y): (i16, i16) = (x.try_into().unwrap(), y.try_into().unwrap());
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
                        let (new_x, new_y) = (x + mov_x, y + mov_y);
                        // Check Board boundaries
                        if new_x >= 0
                            && new_y >= 0
                            && (new_x as usize) < BOARD_SIZE
                            && (new_y as usize) < BOARD_SIZE
                        {
                            *buf.push_back() = if new_x == x && new_y == y {
                                1
                            } else {
                                Finder::pawn_to_pattern_pawn(
                                    board,
                                    new_x as usize,
                                    new_y as usize,
                                    player,
                                )
                            };
                            length += 1;
                            if length >= 7 && buf.iter().filter(|pawn| *pawn == &1).count() >= 2 {
                                let has_best_pattern = best_pattern_index.is_some();
                                let has_no_best_pattern = best_pattern_index.is_none();
                                if let Some((index, (_, _, category))) = self
                                    .patterns
                                    .iter()
                                    .enumerate()
                                    .find(|&(index, (pattern, length, _))| {
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
                                    })
                                {
                                    // println!("Found pattern {:#?} in {:#?}", category, buf);
                                    if has_no_best_pattern || best_pattern_index.unwrap() > index {
                                        best_pattern_index = Some(index);
                                        best_pattern = Some(*category);
                                        if category == &PatternCategory::FiveInRow {
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
        }
        best_pattern
    }

    // For each rocks on the board check all 8 directions to count all patterns
    // -- in a sliding window of 6 around the rock
    pub fn get_patterns(&self, board: &Board, player: &Player) -> Vec<Pattern> {
        let mut patterns: Vec<Pattern> = vec![];
        // Sliding window of 6 (patterns length)
        let mut buf = FixedVecDeque::<[u8; 7]>::new();
        // Iterate trough each rocks on the board
        for existing_pawn in board.all_rocks.iter() {
            let (x, y) = Board::index_to_coordinates(*existing_pawn);
            let (x, y): (i16, i16) = (x.try_into().unwrap(), y.try_into().unwrap());
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
                    let (new_x, new_y) = (x + mov_x, y + mov_y);
                    // Check Board boundaries
                    if new_x >= 0
                        && new_y >= 0
                        && (new_x as usize) < BOARD_SIZE
                        && (new_y as usize) < BOARD_SIZE
                    {
                        *buf.push_back() = Finder::pawn_to_pattern_pawn(
                            board,
                            new_x as usize,
                            new_y as usize,
                            player,
                        );
                        length += 1;
                        if length >= 7 && buf.iter().filter(|pawn| *pawn == &1).count() >= 2 {
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
                                    best_pattern_value = Some(Pattern {
                                        pieces: vec![
                                            // TODO
                                            // Board::coordinates_to_index(x - 4, y),
                                            // Board::coordinates_to_index(x - 3, y),
                                            // Board::coordinates_to_index(x - 2, y),
                                            // Board::coordinates_to_index(x - 1, y),
                                            // Board::coordinates_to_index(x - 0, y),
                                        ],
                                        category: *category,
                                    });
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
        }
        patterns
    }

    pub fn count_patterns(&self, board: &Board, player: &Player) -> PatternCount {
        let mut pattern_count = PatternCount::default();
        let patterns = self.get_patterns(board, player);
        for pattern in patterns.iter() {
            if pattern.category == PatternCategory::FiveInRow {
                pattern_count.five_in_row += 1;
            } else if pattern.category == PatternCategory::LiveFour {
                pattern_count.live_four += 1;
            } else if pattern.category == PatternCategory::DeadFour {
                pattern_count.dead_four += 1;
            } else if pattern.category == PatternCategory::LiveThree {
                pattern_count.live_three += 1;
            } else if pattern.category == PatternCategory::DeadThree {
                pattern_count.dead_three += 1;
            } else if pattern.category == PatternCategory::LiveTwo {
                pattern_count.live_two += 1;
            } else {
                pattern_count.dead_two += 1;
            }
        }
        pattern_count
    }

    // TODO
    pub fn patterns_score(
        &self,
        self_patterns: &PatternCount,
        other_patterns: &PatternCount,
    ) -> i64 {
        let mut score: i64 = 0;
        if self_patterns.five_in_row > 0 {
            score += 100000;
        }
        if other_patterns.dead_four > 0 {
            score += 50000;
        }
        if self_patterns.live_four > 0 {
            score += 25000;
        }
        if self_patterns.live_three >= 1 {
            score += 15000;
        }
        if other_patterns.dead_three >= 1 {
            score += 8000;
        }
        if self_patterns.live_three + other_patterns.dead_three >= 2 {
            score += 5000;
        } else if self_patterns.live_three > 0 {
            score += 2000;
        }
        if self_patterns.dead_four > 0 {
            score += self_patterns.dead_four as i64 * 50;
        }
        if self_patterns.live_two > 0 {
            score += 200;
        }
        score
    }

    pub fn player_score(&self, board: &Board, player: &Player) -> i64 {
        let black_patterns = PATTERN_FINDER.count_patterns(board, &Player::Black);
        let white_patterns = PATTERN_FINDER.count_patterns(board, &Player::White);
        if player == &Player::Black {
            self.patterns_score(&black_patterns, &white_patterns)
        } else {
            self.patterns_score(&white_patterns, &black_patterns)
        }
    }
}

lazy_static! {
    pub static ref PATTERN_FINDER: Finder = Finder::default();
}
