use crate::{
    bitboard::BitBoard,
    board::{Board, Index, Move, WINDOW_DIRECTIONS},
    player::Player,
};
use bitvec::prelude::*;

#[repr(u8)]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Category {
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
    pub five_in_row: usize,
    pub killed_five: usize,
    pub live_four: usize,
    pub killed_four: usize,
    pub dead_four: usize,
    pub killed_three: usize,
    pub live_three: usize,
    pub cut_three: usize,
    pub dead_three: usize,
    pub live_two: usize,
    pub dead_two: usize,
}

impl PatternCount {
    // Order by which to sort the generated moves
    // Gives priority to moves that save the game or end the game
    pub fn best_pattern(&self) -> usize {
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

pub struct DualPattern {
    pub left: BitArray,
    pub right: BitArray,
    pub central_bit: Vec<usize>,
    pub category: Category,
}

pub struct Finder {
    // Group of entries format
    // [window_6, window_5, window_4, window_3]
    // Entries format
    // self pattern, opponent pattern, pattern  category
    // All bits set to 0 in the self pattern will be check with a sliding window
    patterns_by_window: Vec<Vec<DualPattern>>,
}

impl Default for Finder {
    fn default() -> Self {
        let mut patterns_by_window: Vec<Vec<DualPattern>> = vec![];
        // * Window 7
        // (vec![2, 0, 1, 1, 1, 0, 2], 7, Pattern::LiveThree)
        // * Window 6
        patterns_by_window.push(vec![
            DualPattern {
                left: bitarr![1, 0, 0, 0, 0, 1],
                right: bitarr![0, 1, 1, 1, 1, 1],
                central_bit: vec![1, 2, 3, 4],
                category: Category::LiveFour,
            },
            DualPattern {
                left: bitarr![1, 0, 0, 0, 0, 1],
                right: bitarr![1, 1, 1, 1, 1, 0],
                central_bit: vec![1, 2, 3, 4],
                category: Category::LiveFour,
            },
            // --
            DualPattern {
                left: bitarr![0, 1, 1, 1, 1, 0],
                right: bitarr![1, 0, 0, 0, 0, 1],
                central_bit: vec![0, 5],
                category: Category::KilledFive,
            },
            // --
            DualPattern {
                left: bitarr![1, 1, 0, 1, 1],
                right: bitarr![0, 0, 1, 0, 0],
                central_bit: vec![3],
                category: Category::KilledFive,
            },
            // --
            DualPattern {
                left: bitarr![1, 0, 0, 1, 0, 0],
                right: bitarr![1, 1, 1, 1, 1, 1],
                central_bit: vec![1, 2, 4, 5],
                category: Category::LiveFour,
            },
            DualPattern {
                left: bitarr![0, 0, 1, 0, 0, 1],
                right: bitarr![1, 1, 1, 1, 1, 1],
                central_bit: vec![0, 1, 3, 4],
                category: Category::LiveFour,
            },
            // --
            DualPattern {
                left: bitarr![0, 0, 1, 0, 1, 1],
                right: bitarr![1, 1, 1, 1, 0, 1],
                central_bit: vec![0, 1, 3],
                category: Category::LiveFour,
            },
            // --
            DualPattern {
                left: bitarr![1, 1, 0, 1, 0, 0],
                right: bitarr![1, 0, 1, 1, 1, 1],
                central_bit: vec![2, 4, 5],
                category: Category::DeadThree,
            },
        ]);
        // * Window 5
        patterns_by_window.push(vec![
            DualPattern {
                left: bitarr![0, 0, 0, 0, 0],
                right: bitarr![1, 1, 1, 1, 1],
                central_bit: vec![0, 1, 2, 4],
                category: Category::FiveInRow,
            },
            // --
            DualPattern {
                left: bitarr![1, 0, 0, 0, 0],
                right: bitarr![1, 1, 1, 1, 1],
                central_bit: vec![1, 2, 3, 4],
                category: Category::LiveFour,
            },
            // --
            DualPattern {
                left: bitarr![0, 1, 0, 0, 0],
                right: bitarr![1, 1, 1, 1, 1],
                central_bit: vec![0, 2, 3, 4],
                category: Category::LiveFour,
            },
            DualPattern {
                left: bitarr![0, 0, 1, 0, 0],
                right: bitarr![1, 1, 1, 1, 1],
                central_bit: vec![0, 1, 3, 4],
                category: Category::LiveFour,
            },
            DualPattern {
                left: bitarr![0, 0, 0, 1, 0],
                right: bitarr![1, 1, 1, 1, 1],
                central_bit: vec![0, 1, 2, 4],
                category: Category::LiveFour,
            },
            // --
            DualPattern {
                left: bitarr![0, 1, 1, 1, 1],
                right: bitarr![1, 0, 0, 0, 0],
                central_bit: vec![0],
                category: Category::KilledFive,
            },
            DualPattern {
                left: bitarr![1, 1, 1, 1, 0],
                right: bitarr![0, 0, 0, 0, 1],
                central_bit: vec![4],
                category: Category::KilledFive,
            },
            // --
            DualPattern {
                left: bitarr![1, 1, 1, 0, 1],
                right: bitarr![0, 0, 0, 1, 0],
                central_bit: vec![3],
                category: Category::KilledFive,
            },
            DualPattern {
                left: bitarr![1, 0, 1, 1, 1],
                right: bitarr![0, 1, 0, 0, 0],
                central_bit: vec![1],
                category: Category::KilledFive,
            },
            // --
            DualPattern {
                left: bitarr![0, 1, 1, 1, 0],
                right: bitarr![1, 0, 0, 0, 1],
                central_bit: vec![0, 4],
                category: Category::KilledFour,
            },
            // --
            DualPattern {
                left: bitarr![1, 0, 0, 0, 1],
                right: bitarr![1, 1, 1, 1, 1],
                central_bit: vec![1, 2, 3],
                category: Category::LiveThree,
            },
            // --
            DualPattern {
                left: bitarr![1, 0, 0, 0, 1],
                right: bitarr![1, 1, 1, 1, 0],
                central_bit: vec![1, 2, 3],
                category: Category::LiveThree,
            },
            DualPattern {
                left: bitarr![1, 0, 0, 0, 1],
                right: bitarr![0, 1, 1, 1, 1],
                central_bit: vec![1, 2, 3],
                category: Category::LiveThree,
            },
            // --
            DualPattern {
                left: bitarr![0, 1, 0, 1, 0],
                right: bitarr![1, 1, 1, 1, 1],
                central_bit: vec![0, 2, 4],
                category: Category::LiveThree,
            },
            // --
            DualPattern {
                left: bitarr![1, 1, 1, 1, 0],
                right: bitarr![1, 0, 0, 0, 1],
                central_bit: vec![4],
                category: Category::CutThree,
            },
            DualPattern {
                left: bitarr![0, 1, 1, 1, 1],
                right: bitarr![1, 0, 0, 0, 1],
                central_bit: vec![0],
                category: Category::CutThree,
            },
            // --
            DualPattern {
                left: bitarr![1, 1, 1, 0, 1],
                right: bitarr![1, 0, 0, 1, 0],
                central_bit: vec![3],
                category: Category::CutThree,
            },
            DualPattern {
                left: bitarr![1, 0, 1, 1, 1],
                right: bitarr![0, 1, 0, 0, 1],
                central_bit: vec![1],
                category: Category::CutThree,
            },
            // --
            DualPattern {
                left: bitarr![0, 1, 0, 1, 1],
                right: bitarr![1, 1, 1, 1, 0],
                central_bit: vec![0, 2],
                category: Category::DeadThree,
            },
            DualPattern {
                left: bitarr![1, 1, 0, 1, 0],
                right: bitarr![0, 1, 1, 1, 1],
                central_bit: vec![2, 4],
                category: Category::DeadThree,
            },
            // --
            DualPattern {
                left: bitarr![1, 0, 0, 1, 0],
                right: bitarr![0, 1, 1, 1, 1],
                central_bit: vec![1, 2, 4],
                category: Category::DeadThree,
            },
            DualPattern {
                left: bitarr![0, 1, 0, 0, 1],
                right: bitarr![1, 1, 1, 1, 0],
                central_bit: vec![0, 2, 3],
                category: Category::DeadThree,
            },
            // --
            DualPattern {
                left: bitarr![1, 0, 1, 0, 0],
                right: bitarr![0, 1, 1, 1, 1],
                central_bit: vec![1, 3, 4],
                category: Category::DeadThree,
            },
            DualPattern {
                left: bitarr![0, 0, 1, 0, 1],
                right: bitarr![1, 1, 1, 1, 0],
                central_bit: vec![0, 1, 3],
                category: Category::DeadThree,
            },
            // --
            DualPattern {
                left: bitarr![0, 1, 1, 0, 0],
                right: bitarr![1, 1, 1, 1, 1],
                central_bit: vec![0, 3, 4],
                category: Category::DeadThree,
            },
            DualPattern {
                left: bitarr![0, 0, 1, 1, 0],
                right: bitarr![1, 1, 1, 1, 1],
                central_bit: vec![0, 1, 4],
                category: Category::DeadThree,
            },
            // --
            DualPattern {
                left: bitarr![1, 0, 0, 0, 1],
                right: bitarr![1, 1, 1, 1, 0],
                central_bit: vec![1, 2, 3],
                category: Category::DeadThree,
            },
            DualPattern {
                left: bitarr![1, 0, 0, 0, 1],
                right: bitarr![0, 1, 1, 1, 1],
                central_bit: vec![1, 2, 3],
                category: Category::DeadThree,
            },
            // --
            DualPattern {
                left: bitarr![0, 1, 1, 0, 1],
                right: bitarr![1, 1, 1, 1, 0],
                central_bit: vec![0, 3],
                category: Category::LiveTwo,
            },
            DualPattern {
                left: bitarr![1, 0, 1, 1, 0],
                right: bitarr![0, 1, 1, 1, 1],
                central_bit: vec![1, 4],
                category: Category::LiveTwo,
            },
            // --
            DualPattern {
                left: bitarr![0, 1, 1, 1, 0],
                right: bitarr![1, 1, 1, 1, 1],
                central_bit: vec![1, 4],
                category: Category::DeadTwo,
            },
        ]);
        // * Window 4
        patterns_by_window.push(vec![
            DualPattern {
                left: bitarr![1, 0, 1, 1],
                right: bitarr![0, 1, 0, 0],
                central_bit: vec![1],
                category: Category::KilledFour,
            },
            DualPattern {
                left: bitarr![1, 1, 0, 1],
                right: bitarr![0, 0, 1, 0],
                central_bit: vec![2],
                category: Category::KilledFour,
            },
            // --
            DualPattern {
                left: bitarr![1, 1, 1, 0],
                right: bitarr![0, 0, 0, 1],
                central_bit: vec![3],
                category: Category::KilledFour,
            },
            DualPattern {
                left: bitarr![0, 1, 1, 1],
                right: bitarr![1, 0, 0, 0],
                central_bit: vec![0],
                category: Category::KilledFour,
            },
            // --
            DualPattern {
                left: bitarr![1, 0, 1, 1],
                right: bitarr![1, 1, 1, 1],
                central_bit: vec![1],
                category: Category::LiveThree,
            },
            DualPattern {
                left: bitarr![1, 1, 0, 1],
                right: bitarr![1, 1, 1, 1],
                central_bit: vec![2],
                category: Category::LiveThree,
            },
            // --
            DualPattern {
                left: bitarr![0, 1, 0, 1],
                right: bitarr![1, 1, 1, 0],
                central_bit: vec![0, 2],
                category: Category::LiveTwo,
            },
            DualPattern {
                left: bitarr![1, 0, 1, 0],
                right: bitarr![0, 1, 1, 1],
                central_bit: vec![1, 3],
                category: Category::LiveTwo,
            },
            // --
            DualPattern {
                left: bitarr![0, 1, 1, 0],
                right: bitarr![1, 1, 1, 1],
                central_bit: vec![0, 3],
                category: Category::LiveTwo,
            },
        ]);
        // * Window 3
        patterns_by_window.push(vec![
            DualPattern {
                left: bitarr![0, 0, 0],
                right: bitarr![1, 1, 1],
                central_bit: vec![0, 1, 2],
                category: Category::LiveThree,
            },
            DualPattern {
                left: bitarr![0, 1, 0],
                right: bitarr![1, 1, 1],
                central_bit: vec![0, 2],
                category: Category::LiveTwo,
            },
            DualPattern {
                left: bitarr![0, 0, 1],
                right: bitarr![1, 1, 0],
                central_bit: vec![0, 1],
                category: Category::DeadTwo,
            },
            DualPattern {
                left: bitarr![1, 0, 0],
                right: bitarr![0, 1, 1],
                central_bit: vec![1, 2],
                category: Category::DeadTwo,
            },
        ]);
        // * Window 2
        // (bits![0, 0], bits![1, 1], Category::DeadTwo)
        Finder { patterns_by_window }
    }
}

impl Finder {
    // For each rocks on the board check all 8 directions to count all patterns
    // -- in a sliding window of 6 around the rock
    pub fn get_patterns_for_movement(&self, board: &Board, movement: &Move) -> Vec<Category> {
        let mut patterns: Vec<Category> = vec![];
        let index = movement.index;
        let boards: &[BitArray<[usize; 6]>; 4];
        let opponent_boards: &[BitArray<[usize; 6]>; 4];
        if movement.player == Player::Black {
            boards = &board.boards[Index::BLACK];
            opponent_boards = &board.boards[Index::WHITE];
        } else {
            boards = &board.boards[Index::WHITE];
            opponent_boards = &board.boards[Index::BLACK];
        };
        // For each windows in all the windows
        // -- For each patterns in the window
        // -- -- compare the pattern on each [0] positions in the pattern
        let windows = [
            &BitBoard.window_six,
            &BitBoard.window_five,
            &BitBoard.window_four,
            &BitBoard.window_three,
        ];
        for (window_index, window) in self.patterns_by_window.iter().enumerate() {
            let slices = windows[window_index];
            for pattern in window {
                for &central_bit in pattern.central_bit.iter() {
                    let slices = slices[central_bit];
                    // Iterate on all directions
                    for direction in WINDOW_DIRECTIONS {
                        let slice = slices[direction][index];
                        if boards[direction][slice.0..=slice.1]
                            .eq(&pattern.left[0..6 - window_index])
                            && opponent_boards[direction][slice.0..=slice.1]
                                .eq(&pattern.right[0..6 - window_index])
                        {
                            patterns.push(pattern.category);
                        }
                    }
                }
            }
        }
        patterns
    }

    pub fn count_movement_patterns(&self, board: &Board, movement: &Move) -> PatternCount {
        let mut pattern_count = PatternCount::default();
        let patterns = self.get_patterns_for_movement(board, movement);
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
            } else if pattern == Category::KilledThree {
                pattern_count.killed_three += 1;
            } else if pattern == Category::CutThree {
                pattern_count.cut_three += 1;
            } else if pattern == Category::LiveThree {
                pattern_count.live_three += 1;
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

    // TODO
    pub fn movement_patterns_score(&self, patterns: &PatternCount) -> i64 {
        let mut score: i64 = 0;
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
            score += patterns.dead_four as i64 * 50;
        }
        if patterns.live_two > 0 {
            score += 200;
        }
        score
    }
}

lazy_static! {
    pub static ref PatternFinder: Finder = Finder::default();
}
