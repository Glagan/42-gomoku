use crate::{
    board::{Board, Move, Pawn, BOARD_PIECES, BOARD_SIZE, DIRECTIONS},
    player::Player,
    rules::RuleSet,
};
use colored::Colorize;
use fixed_vec_deque::FixedVecDeque;
use std::{cell::RefCell, collections::HashMap, fmt};

#[derive(Debug)]
pub struct Evaluation {
    pub score: i64,
    pub movement: Option<Move>,
}

impl fmt::Display for Evaluation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(movement) = self.movement {
            write!(f, "score {} movement {}", self.score, movement)
        } else {
            write!(f, "score {} {}", self.score, "without movement !".red())
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PatternCategory {
    FiveInRow,
    LiveFour,
    DeadFour,
    LiveThree,
    DeadThree,
    LiveTwo,
    DeadTwo,
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

#[derive(PartialEq)]
pub enum CacheFlag {
    Exact = 0,
    Upperbound = 1,
    Lowerbound = 2,
}

pub struct CacheEntry {
    pub score: i64,
    pub rocks: u16,
    pub flag: CacheFlag,
    pub movement: Option<Move>,
}

// impl CacheEntry {
//     pub fn for_player(&self, player: &Player) -> i64 {
//         if player == &Player::Black {
//             self.black
//         } else {
//             self.white
//         }
//     }
// }

pub struct Computer {
    patterns: Vec<(Vec<u8>, u8, PatternCategory)>,
    // (black_heuristic, white_heuristic)
    pub black_cache: HashMap<[Pawn; BOARD_PIECES as usize], CacheEntry>,
    pub white_cache: HashMap<[Pawn; BOARD_PIECES as usize], CacheEntry>,
}

impl Default for Computer {
    fn default() -> Self {
        Computer {
            patterns: Computer::initialize_patterns(),
            black_cache: HashMap::new(),
            white_cache: HashMap::new(),
        }
    }
}

impl Computer {
    pub fn initialize_patterns() -> Vec<(Vec<u8>, u8, PatternCategory)> {
        let mut patterns: Vec<(Vec<u8>, u8, PatternCategory)> = vec![];

        patterns.push((vec![1, 1, 1, 1, 1], 5, PatternCategory::FiveInRow));
        // 2x1
        patterns.push((vec![0, 1, 1, 1, 1, 0], 6, PatternCategory::LiveFour));
        // 3x1
        patterns.push((vec![1, 1, 1, 1, 2], 5, PatternCategory::DeadFour));
        patterns.push((vec![2, 1, 1, 1, 1], 5, PatternCategory::DeadFour));
        // 4x1
        patterns.push((vec![1, 0, 1, 1, 1, 0], 6, PatternCategory::LiveFour));
        patterns.push((vec![0, 1, 1, 1, 0, 1], 6, PatternCategory::LiveFour));
        // 5x1
        patterns.push((vec![0, 1, 1, 0, 1, 1], 6, PatternCategory::LiveFour));
        patterns.push((vec![1, 1, 0, 1, 1, 0], 6, PatternCategory::LiveFour));
        // 6x1
        patterns.push((vec![1, 1, 1], 3, PatternCategory::LiveThree));
        // 7x1
        patterns.push((vec![1, 1, 0, 1], 4, PatternCategory::LiveThree));
        patterns.push((vec![1, 0, 1, 1], 4, PatternCategory::LiveThree));
        // 1x2
        patterns.push((vec![1, 1, 1, 2], 4, PatternCategory::DeadThree));
        patterns.push((vec![2, 1, 1, 1], 4, PatternCategory::DeadThree));
        // 2x2
        patterns.push((vec![1, 0, 1, 1, 2], 5, PatternCategory::DeadThree));
        patterns.push((vec![2, 1, 1, 0, 1], 5, PatternCategory::DeadThree));
        // 3x2
        patterns.push((vec![2, 1, 0, 1, 1], 5, PatternCategory::DeadThree));
        patterns.push((vec![1, 1, 0, 1, 2, 0], 6, PatternCategory::DeadThree));
        // 4x2
        patterns.push((vec![1, 0, 0, 1, 1], 5, PatternCategory::DeadThree));
        patterns.push((vec![1, 1, 0, 0, 1], 5, PatternCategory::DeadThree));
        // 5x2
        patterns.push((vec![1, 0, 1, 0, 1, 0], 6, PatternCategory::DeadThree));
        patterns.push((vec![0, 1, 0, 1, 0, 1], 6, PatternCategory::DeadThree));
        // 6x2
        patterns.push((vec![2, 0, 1, 1, 1, 0, 2], 7, PatternCategory::DeadThree));
        // 7x2
        patterns.push((vec![1, 0, 0, 0, 1], 5, PatternCategory::LiveTwo));
        // 1x3
        patterns.push((vec![1, 0, 1], 3, PatternCategory::LiveTwo));
        // 2x3
        patterns.push((vec![1, 0, 0, 1], 4, PatternCategory::LiveTwo));
        // 3x3
        patterns.push((vec![1, 1, 2], 3, PatternCategory::DeadTwo));
        patterns.push((vec![2, 1, 1], 3, PatternCategory::DeadTwo));
        // 4x3
        patterns.push((vec![1, 0, 1, 2], 4, PatternCategory::DeadTwo));
        patterns.push((vec![2, 1, 0, 1], 4, PatternCategory::DeadTwo));
        // 5x3
        patterns.push((vec![1, 0, 0, 1, 2], 5, PatternCategory::DeadTwo));
        patterns.push((vec![2, 1, 0, 0, 1], 5, PatternCategory::DeadTwo));
        // 6x3
        patterns.push((vec![1, 1], 2, PatternCategory::DeadTwo));

        patterns
    }

    pub fn clean(&mut self) {
        // TODO: Pre-calculate first three rounds cache to avoid slow startups ?
        self.black_cache = HashMap::new();
        self.white_cache = HashMap::new();
    }

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

    // For each rocks on the board check all 8 directions to count all patterns
    // -- in a sliding window of 6 around the rock
    pub fn get_patterns(&self, board: &Board, player: &Player) -> Vec<Pattern> {
        let mut patterns: Vec<Pattern> = vec![];
        // Sliding window of 6 (patterns length)
        let mut buf = FixedVecDeque::<[u8; 6]>::new();
        // Iterate trough each rocks on the board
        for existing_pawn in board.all_rocks.iter() {
            let (x, y) = Board::index_to_coordinates(*existing_pawn);
            let (x, y): (i16, i16) = (x.try_into().unwrap(), y.try_into().unwrap());
            for (dir_x, dir_y) in DIRECTIONS {
                // Initialize to -6 so the first 6 elements
                // -- can be set and the last one is the initial rock
                let mut length = 0;
                let best_pattern_index: RefCell<Option<usize>> = RefCell::new(None);
                let best_pattern_value: RefCell<Option<Pattern>> = RefCell::new(None);
                // from [x x x x x x] ? ? ? ? ? I  ? ? ? ? ?
                // to    x x x x x x  ? ? ? ? ? [I ? ? ? ? ?]
                let mut mov_x = dir_x * -6;
                let mut mov_y = dir_y * -6;
                for _ in 0..11 {
                    let (new_x, new_y) = (x + mov_x, y + mov_y);
                    // Check Board boundaries
                    if new_x >= 0
                        && new_y >= 0
                        && (new_x as usize) < BOARD_SIZE
                        && (new_y as usize) < BOARD_SIZE
                    {
                        *buf.push_front() = Computer::pawn_to_pattern_pawn(
                            board,
                            new_x as usize,
                            new_y as usize,
                            player,
                        );
                        length += 1;
                        if length >= 6 && buf.iter().filter(|pawn| *pawn == &1).count() >= 2 {
                            if let Some((index, (_, _, category))) =
                                self.patterns.iter().enumerate().find(
                                    |(_, (pattern, length, _))| {
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
                                if best_pattern_index.borrow().is_none()
                                    || best_pattern_index.borrow().unwrap() > index
                                {
                                    *best_pattern_index.borrow_mut() = Some(index);
                                    *best_pattern_value.borrow_mut() = Some(Pattern {
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
                let best_pattern = best_pattern_value.borrow().to_owned();
                if let Some(best_pattern) = best_pattern {
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
    pub fn patterns_score(self_patterns: &PatternCount, other_patterns: &PatternCount) -> i64 {
        let mut score: i64 = 0;
        if self_patterns.five_in_row > 0 {
            score += 100000;
        }
        if other_patterns.dead_four > 0 {
            score += 50000;
        }
        if self_patterns.live_four > 0 {
            score += 15000;
        }
        if self_patterns.live_three >= 1
            || other_patterns.dead_four == 2
            || other_patterns.dead_four == 1
        {
            score += 10000;
        }
        if other_patterns.dead_three >= 1 {
            score += 8000;
        }
        if self_patterns.live_three + other_patterns.dead_three >= 2 {
            score += 5000;
        } else if self_patterns.live_three > 0 {
            score += 2000;
        }
        if other_patterns.dead_three > 0 {
            score += 1500;
        }
        if self_patterns.dead_four > 0 {
            score += self_patterns.dead_four as i64 * 50;
        }
        if self_patterns.live_two > 0 {
            score += 200;
        }
        score
    }

    // Calculate all patterns for a both players and return the board score
    pub fn evaluate_board(&self, board: &Board, player: &Player) -> i64 {
        let black_patterns = self.count_patterns(board, &Player::Black);
        let white_patterns = self.count_patterns(board, &Player::White);
        if player == &Player::Black {
            Computer::patterns_score(&black_patterns, &white_patterns)
        } else {
            Computer::patterns_score(&white_patterns, &black_patterns)
        }
    }

    pub fn cache(
        &mut self,
        player: &Player,
    ) -> &mut HashMap<[Pawn; BOARD_PIECES as usize], CacheEntry> {
        if player == &Player::Black {
            &mut self.black_cache
        } else {
            &mut self.white_cache
        }
    }

    #[allow(dead_code)]
    fn minimax(
        &mut self,
        rules: &RuleSet,
        board: &Board,
        depth: usize,
        player: &Player,
        maximize: &Player,
    ) -> Result<Evaluation, String> {
        // Check cache to see if the board was already computed
        if self.cache(player).contains_key(&board.pieces) {
            let cache_entry = self.cache(player).get(&board.pieces).unwrap();
            if cache_entry.rocks >= board.rocks {
                return Ok(Evaluation {
                    score: cache_entry.score,
                    movement: cache_entry.movement,
                });
            }
        }

        // Check if it's a leaf and compute it's value
        if depth == 0 || board.is_winning(rules, player) {
            let score = self.evaluate_board(board, player);
            return Ok(Evaluation {
                score: score,
                movement: None,
            });
        }

        // Iterate each neighbor moves
        let other_player = if *player == Player::Black {
            &Player::White
        } else {
            &Player::Black
        };
        if player == maximize {
            let mut max_eval = Evaluation {
                score: i64::min_value(),
                movement: None,
            };
            for movement in board.intersections_legal_moves(rules, player).iter() {
                let new_board = board.apply_move(rules, movement);
                let eval = self.minimax(rules, &new_board, depth - 1, other_player, maximize)?;
                if eval.score > max_eval.score {
                    max_eval.score = eval.score;
                    max_eval.movement = Some(movement.clone());
                }
            }

            // Add to cache
            self.cache(player)
                .entry(board.pieces.clone())
                .or_insert(CacheEntry {
                    score: max_eval.score,
                    rocks: board.rocks,
                    flag: CacheFlag::Exact,
                    movement: max_eval.movement,
                });

            return Ok(max_eval);
        } else {
            let mut min_eval = Evaluation {
                score: i64::max_value(),
                movement: None,
            };
            for movement in board.intersections_legal_moves(rules, player).iter() {
                let new_board = board.apply_move(rules, movement);
                let eval = self.minimax(rules, &new_board, depth - 1, other_player, maximize)?;
                if eval.score < min_eval.score {
                    min_eval.score = eval.score;
                    min_eval.movement = Some(movement.clone());
                }
            }

            // Add to cache
            self.cache(player)
                .entry(board.pieces.clone())
                .or_insert(CacheEntry {
                    score: min_eval.score,
                    rocks: board.rocks,
                    flag: CacheFlag::Exact,
                    movement: min_eval.movement,
                });

            return Ok(min_eval);
        }
    }

    /*fn minimax_alpha_beta(
        &mut self,
        rules: &RuleSet,
        board: &Board,
        depth: usize,
        alpha: i64,
        beta: i64,
        player: &Player,
        maximize: &Player,
    ) -> Result<Evaluation, String> {
        if depth == 0 || board.is_winning(rules, player) {
            let scores = Computer::evaluate_board(board);
            return Ok(Evaluation {
                score: if player == &Player::Black {
                    scores.0
                } else {
                    scores.1
                },
                movement: None,
            });
        }
        let other_player = if *player == Player::Black {
            &Player::White
        } else {
            &Player::Black
        };
        if player == maximize {
            let mut alpha = alpha;
            let mut best_eval = Evaluation {
                score: i64::min_value(),
                movement: None,
            };
            for movement in board.intersections_legal_moves(rules, player).iter() {
                let new_board = board.apply_move(rules, movement);
                let eval = self.minimax_alpha_beta(
                    rules,
                    &new_board,
                    depth - 1,
                    alpha,
                    beta,
                    other_player,
                    maximize,
                )?;
                if eval.score > alpha {
                    alpha = eval.score;
                    best_eval.score = eval.score;
                    best_eval.movement = Some(movement.clone());
                }
                if beta <= alpha {
                    break;
                }
            }
            Ok(best_eval)
        } else {
            let mut beta = beta;
            let mut best_eval = Evaluation {
                score: i64::max_value(),
                movement: None,
            };
            for movement in board.intersections_legal_moves(rules, player).iter() {
                let new_board = board.apply_move(rules, movement);
                let eval = self.minimax_alpha_beta(
                    rules,
                    &new_board,
                    depth - 1,
                    alpha,
                    beta,
                    other_player,
                    maximize,
                )?;
                if eval.score < beta {
                    beta = eval.score;
                    best_eval.score = eval.score;
                    best_eval.movement = Some(movement.clone());
                }
                if beta <= alpha {
                    break;
                }
            }
            Ok(best_eval)
        }
    }*/

    #[allow(dead_code)]
    fn negamax_alpha_beta(
        &mut self,
        rules: &RuleSet,
        board: &Board,
        depth: usize,
        alpha: i64,
        beta: i64,
        player: &Player,
        maximize: &Player,
    ) -> Result<Evaluation, String> {
        let alpha_orig = alpha;
        let mut alpha = alpha;
        let mut beta = beta;

        // Check cache to see if the board was already computed
        if self.cache(maximize).contains_key(&board.pieces) {
            let cache_entry = self.cache(maximize).get(&board.pieces).unwrap();
            if cache_entry.rocks >= board.rocks {
                if cache_entry.flag == CacheFlag::Exact {
                    return Ok(Evaluation {
                        score: cache_entry.score,
                        movement: cache_entry.movement,
                    });
                } else if cache_entry.flag == CacheFlag::Lowerbound {
                    if cache_entry.score > alpha {
                        alpha = cache_entry.score
                    }
                } else if cache_entry.flag == CacheFlag::Upperbound {
                    if cache_entry.score < beta {
                        beta = cache_entry.score
                    }
                }

                if alpha >= beta {
                    return Ok(Evaluation {
                        score: cache_entry.score,
                        movement: cache_entry.movement,
                    });
                }
            }
        }

        // Check if it's a leaf and compute it's value
        if depth == 0 || board.is_winning(rules, player) {
            // println!("{}", board);
            let color = if player == maximize { 1 } else { -1 };
            let score = self.evaluate_board(&board, player);
            return Ok(Evaluation {
                score: color * score,
                movement: None,
            });
        }

        // Only the best evaluation is returned
        let mut best_eval = Evaluation {
            score: i64::min_value(),
            movement: None,
        };

        // Iterate each neighbor moves
        for movement in board.intersections_legal_moves(rules, player).iter() {
            let new_board = board.apply_move(rules, movement);
            let mut eval = self.negamax_alpha_beta(
                rules,
                &new_board,
                depth - 1,
                -beta,
                -alpha,
                if *player == Player::Black {
                    &Player::White
                } else {
                    &Player::Black
                },
                maximize,
            )?;
            eval.score = -eval.score;
            if eval.score > best_eval.score {
                alpha = eval.score;
                best_eval.score = eval.score;
                best_eval.movement = Some(movement.clone());
            }
            if alpha >= beta {
                break;
            }
        }

        // Add to cache
        let cache_entry = self
            .cache(maximize)
            .entry(board.pieces.clone())
            .or_insert(CacheEntry {
                score: best_eval.score,
                rocks: board.rocks,
                flag: CacheFlag::Exact,
                movement: best_eval.movement,
            });
        if best_eval.score <= alpha_orig {
            cache_entry.flag = CacheFlag::Upperbound;
        } else if best_eval.score >= beta {
            cache_entry.flag = CacheFlag::Lowerbound;
        }

        return Ok(best_eval);
    }

    // Use the negamax algorithm (minimax variant) to get the next best move
    pub fn play(
        &mut self,
        rules: &RuleSet,
        board: &Board,
        depth: usize,
        player: &Player,
    ) -> Result<Evaluation, String> {
        // Clean cache
        self.black_cache.retain(|_, v| v.rocks >= board.rocks);
        self.white_cache.retain(|_, v| v.rocks >= board.rocks);

        // Apply negamax recursively
        // let best_move = self.negamax_alpha_beta(
        //     rules,
        //     board,
        //     depth,
        //     i64::min_value(),
        //     i64::max_value(),
        //     player,
        //     player,
        // )?;
        // Apply minimax recursively
        let best_move = self.minimax(rules, board, depth, player, player)?;
        Ok(best_move)
    }
}
