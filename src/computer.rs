use std::collections::HashMap;

use crate::{
    board::{Board, Move, Pawn, BOARD_PIECES, BOARD_SIZE},
    player::Player,
    rules::RuleSet,
};

#[derive(Debug)]
pub struct MiniMaxEvaluation {
    pub score: i64,
    pub movement: Option<Move>,
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

#[allow(dead_code)]
const PATTERNS: [([u8; 6], PatternCategory); 54] = [
    // 1x1
    ([0, 1, 1, 1, 1, 1], PatternCategory::FiveInRow),
    ([1, 1, 1, 1, 1, 0], PatternCategory::FiveInRow),
    // 2x1
    ([0, 0, 1, 1, 1, 1], PatternCategory::LiveFour),
    ([0, 1, 1, 1, 1, 0], PatternCategory::LiveFour),
    ([1, 1, 1, 1, 0, 0], PatternCategory::LiveFour),
    // 3x1
    ([1, 1, 1, 1, 2, 0], PatternCategory::DeadFour),
    ([0, 1, 1, 1, 1, 2], PatternCategory::DeadFour),
    ([2, 1, 1, 1, 1, 0], PatternCategory::DeadFour),
    ([0, 2, 1, 1, 1, 1], PatternCategory::DeadFour),
    // 4x1
    ([1, 0, 1, 1, 1, 0], PatternCategory::DeadFour),
    ([0, 1, 0, 1, 1, 1], PatternCategory::DeadFour),
    ([1, 1, 1, 0, 1, 0], PatternCategory::DeadFour),
    ([0, 1, 1, 1, 0, 1], PatternCategory::DeadFour),
    // 5x1
    ([0, 1, 1, 0, 1, 1], PatternCategory::DeadFour),
    ([1, 1, 0, 1, 1, 0], PatternCategory::DeadFour),
    // 6x1
    ([0, 0, 0, 1, 1, 1], PatternCategory::LiveThree),
    ([0, 0, 1, 1, 1, 0], PatternCategory::LiveThree),
    ([0, 1, 1, 1, 0, 0], PatternCategory::LiveThree),
    ([1, 1, 1, 0, 0, 0], PatternCategory::LiveThree),
    // 7x1
    ([1, 1, 0, 1, 0, 0], PatternCategory::LiveThree),
    ([0, 1, 1, 0, 1, 0], PatternCategory::LiveThree),
    ([0, 0, 1, 1, 0, 1], PatternCategory::LiveThree),
    ([1, 0, 1, 1, 0, 0], PatternCategory::LiveThree),
    ([0, 1, 0, 1, 1, 0], PatternCategory::LiveThree),
    ([0, 0, 1, 0, 1, 1], PatternCategory::LiveThree),
    // 1x2
    ([1, 1, 1, 2, 0, 0], PatternCategory::DeadThree),
    ([0, 1, 1, 1, 2, 0], PatternCategory::DeadThree),
    ([0, 0, 1, 1, 1, 2], PatternCategory::DeadThree),
    ([2, 1, 1, 1, 0, 0], PatternCategory::DeadThree),
    ([0, 2, 1, 1, 1, 0], PatternCategory::DeadThree),
    ([0, 0, 2, 1, 1, 1], PatternCategory::DeadThree),
    // 2x2
    ([1, 0, 1, 1, 2, 0], PatternCategory::DeadThree),
    ([0, 1, 0, 1, 1, 2], PatternCategory::DeadThree),
    ([2, 1, 1, 0, 1, 0], PatternCategory::DeadThree),
    ([0, 2, 1, 1, 0, 1], PatternCategory::DeadThree),
    // 3x2
    ([2, 1, 0, 1, 1, 0], PatternCategory::DeadThree),
    ([0, 2, 1, 0, 1, 1], PatternCategory::DeadThree),
    ([1, 1, 0, 1, 2, 0], PatternCategory::DeadThree),
    ([0, 1, 1, 0, 1, 2], PatternCategory::DeadThree),
    // 4x2
    ([1, 0, 0, 1, 1, 0], PatternCategory::DeadThree),
    ([0, 1, 0, 0, 1, 1], PatternCategory::DeadThree),
    ([1, 1, 0, 0, 1, 0], PatternCategory::DeadThree),
    ([0, 1, 1, 0, 0, 1], PatternCategory::DeadThree),
    // 5x2
    ([1, 0, 1, 0, 1, 0], PatternCategory::DeadThree),
    ([0, 1, 0, 1, 0, 1], PatternCategory::DeadThree),
    // 6x2
    // ([2, 0, 1, 1, 1, 0, 2], PatternCategory::DeadThree),
    // 7x2
    ([1, 0, 0, 0, 1, 0], PatternCategory::LiveTwo),
    ([0, 1, 0, 0, 0, 1], PatternCategory::LiveTwo),
    // 1x3
    ([1, 0, 1, 0, 0, 0], PatternCategory::LiveTwo),
    ([0, 1, 0, 1, 0, 0], PatternCategory::LiveTwo),
    ([0, 0, 1, 0, 1, 0], PatternCategory::LiveTwo),
    ([0, 0, 0, 1, 0, 1], PatternCategory::LiveTwo),
    // 2x3
    ([1, 0, 0, 1, 0, 0], PatternCategory::LiveTwo),
    ([0, 1, 0, 0, 1, 0], PatternCategory::LiveTwo),
    ([0, 0, 1, 0, 0, 1], PatternCategory::LiveTwo),
    // 3x3
    // ([1, 1, 2, 0, 0, 0], PatternCategory::DeadTwo),
    // ([0, 1, 1, 2, 0, 0], PatternCategory::DeadTwo),
    // ([0, 0, 1, 1, 2, 0], PatternCategory::DeadTwo),
    // ([0, 0, 0, 1, 1, 2], PatternCategory::DeadTwo),
    // ([2, 1, 1, 0, 0, 0], PatternCategory::DeadTwo),
    // ([0, 2, 1, 1, 0, 0], PatternCategory::DeadTwo),
    // ([0, 0, 2, 1, 1, 0], PatternCategory::DeadTwo),
    // ([0, 0, 0, 2, 1, 1], PatternCategory::DeadTwo),
    // // 4x3
    // ([1, 0, 1, 2, 0, 0], PatternCategory::DeadTwo),
    // ([0, 1, 0, 1, 2, 0], PatternCategory::DeadTwo),
    // ([0, 0, 1, 0, 1, 2], PatternCategory::DeadTwo),
    // ([2, 1, 0, 1, 0, 0], PatternCategory::DeadTwo),
    // ([0, 2, 1, 0, 1, 0], PatternCategory::DeadTwo),
    // ([0, 0, 2, 1, 0, 1], PatternCategory::DeadTwo),
    // // 5x3
    // ([1, 0, 0, 1, 2, 0], PatternCategory::DeadTwo),
    // ([0, 1, 0, 0, 1, 2], PatternCategory::DeadTwo),
    // ([2, 1, 0, 0, 1, 0], PatternCategory::DeadTwo),
    // ([0, 2, 1, 0, 0, 1], PatternCategory::DeadTwo),
    // // 6x3
    // ([1, 1, 0, 0, 0, 0], PatternCategory::DeadTwo),
    // ([0, 1, 1, 0, 0, 0], PatternCategory::DeadTwo),
    // ([0, 0, 1, 1, 0, 0], PatternCategory::DeadTwo),
    // ([0, 0, 0, 1, 1, 0], PatternCategory::DeadTwo),
    // ([0, 0, 0, 0, 1, 1], PatternCategory::DeadTwo),
];

#[derive(Debug, Clone)]
pub struct Pattern {
    pub pieces: Vec<usize>,
    pub category: PatternCategory,
}

pub struct Computer {
    // (black_heuristic, white_heuristic)
    pub cache: HashMap<[Pawn; BOARD_PIECES as usize], (i64, i64)>,
}

impl Default for Computer {
    fn default() -> Self {
        Computer {
            cache: HashMap::new(),
        }
    }
}

impl Computer {
    pub fn clean(&mut self) {
        // TODO: Pre-calculate first three rounds cache to avoid slow startups ?
        self.cache = HashMap::new();
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

    // Create an array of size 6 and compare it with all the patterns
    pub fn get_horizontal_patterns(board: &Board, player: &Player) -> Vec<Pattern> {
        let mut patterns: Vec<Pattern> = vec![];
        let mut window: [u8; 6] = [0, 0, 0, 0, 0, 0];
        for y in 0..BOARD_SIZE {
            // Go trough the first 5 with an offset of 1
            // -- the next x loop will have the correct initial window
            for i in 0..5 {
                window[i + 1] = Computer::pawn_to_pattern_pawn(board, i, y, player);
            }
            for x in 5..BOARD_SIZE {
                window[0] = window[1];
                window[1] = window[2];
                window[2] = window[3];
                window[3] = window[4];
                window[4] = window[5];
                window[5] = Computer::pawn_to_pattern_pawn(board, x, y, player);
                if window.iter().filter(|pawn| *pawn == &1).count() >= 2 {
                    if let Some(found) = PATTERNS.iter().find(|pattern| pattern.0 == window) {
                        patterns.push(Pattern {
                            pieces: vec![
                                // TODO
                                Board::coordinates_to_index(x - 4, y),
                                Board::coordinates_to_index(x - 3, y),
                                Board::coordinates_to_index(x - 2, y),
                                Board::coordinates_to_index(x - 1, y),
                                Board::coordinates_to_index(x - 0, y),
                            ],
                            category: found.1,
                        });
                        continue;
                    }
                }
            }
        }
        patterns
    }

    pub fn get_vertical_patterns(board: &Board, player: &Player) -> Vec<Pattern> {
        let mut patterns: Vec<Pattern> = vec![];
        let mut window: [u8; 6] = [0, 0, 0, 0, 0, 0];
        for x in 0..BOARD_SIZE {
            // Go trough the first 5 with an offset of 1
            // -- the next y loop will have the correct initial window
            for i in 0..5 {
                window[i + 1] = Computer::pawn_to_pattern_pawn(board, x, i, player);
            }
            for y in 5..BOARD_SIZE {
                window[0] = window[1];
                window[1] = window[2];
                window[2] = window[3];
                window[3] = window[4];
                window[4] = window[5];
                window[5] = Computer::pawn_to_pattern_pawn(board, x, y, player);
                if window.iter().filter(|pawn| *pawn == &1).count() >= 2 {
                    if let Some(found) = PATTERNS.iter().find(|pattern| pattern.0 == window) {
                        patterns.push(Pattern {
                            pieces: vec![
                                // TODO
                                Board::coordinates_to_index(x, y - 4),
                                Board::coordinates_to_index(x, y - 3),
                                Board::coordinates_to_index(x, y - 2),
                                Board::coordinates_to_index(x, y - 1),
                                Board::coordinates_to_index(x, y - 0),
                            ],
                            category: found.1,
                        });
                        continue;
                    }
                }
            }
        }
        patterns
    }

    pub fn get_diagonal_left_patterns(board: &Board, player: &Player) -> Vec<Pattern> {
        let mut patterns: Vec<Pattern> = vec![];
        let mut window: [u8; 6] = [0, 0, 0, 0, 0, 0];
        for y in 0..(BOARD_SIZE - 6) {
            // Go trough the first 5 with an offset of 1
            // -- the next x loop will have the correct initial window
            for x in 0..(BOARD_SIZE - 6) {
                window[0] = Computer::pawn_to_pattern_pawn(board, x, y, player);
                window[1] = Computer::pawn_to_pattern_pawn(board, x + 1, y + 1, player);
                window[2] = Computer::pawn_to_pattern_pawn(board, x + 2, y + 2, player);
                window[3] = Computer::pawn_to_pattern_pawn(board, x + 3, y + 3, player);
                window[4] = Computer::pawn_to_pattern_pawn(board, x + 4, y + 4, player);
                window[5] = Computer::pawn_to_pattern_pawn(board, x + 5, y + 5, player);
                if window.iter().filter(|pawn| *pawn == &1).count() >= 2 {
                    if let Some(found) = PATTERNS.iter().find(|pattern| pattern.0 == window) {
                        patterns.push(Pattern {
                            pieces: vec![
                                // TODO
                                Board::coordinates_to_index(x, y - 4),
                                Board::coordinates_to_index(x, y - 3),
                                Board::coordinates_to_index(x, y - 2),
                                Board::coordinates_to_index(x, y - 1),
                                Board::coordinates_to_index(x, y - 0),
                            ],
                            category: found.1,
                        });
                        continue;
                    }
                }
            }
        }
        patterns
    }

    pub fn get_diagonal_right_patterns(board: &Board, player: &Player) -> Vec<Pattern> {
        let mut patterns: Vec<Pattern> = vec![];
        let mut window: [u8; 6] = [0, 0, 0, 0, 0, 0];
        for y in 0..(BOARD_SIZE - 6) {
            // Go trough the first 5 with an offset of 1
            // -- the next x loop will have the correct initial window
            for x in 5..BOARD_SIZE {
                window[0] = Computer::pawn_to_pattern_pawn(board, x, y, player);
                window[1] = Computer::pawn_to_pattern_pawn(board, x - 1, y + 1, player);
                window[2] = Computer::pawn_to_pattern_pawn(board, x - 2, y + 2, player);
                window[3] = Computer::pawn_to_pattern_pawn(board, x - 3, y + 3, player);
                window[4] = Computer::pawn_to_pattern_pawn(board, x - 4, y + 4, player);
                window[5] = Computer::pawn_to_pattern_pawn(board, x - 5, y + 5, player);
                if window.iter().filter(|pawn| *pawn == &1).count() >= 2 {
                    if let Some(found) = PATTERNS.iter().find(|pattern| pattern.0 == window) {
                        patterns.push(Pattern {
                            pieces: vec![
                                // TODO
                                Board::coordinates_to_index(x, y - 4),
                                Board::coordinates_to_index(x, y - 3),
                                Board::coordinates_to_index(x, y - 2),
                                Board::coordinates_to_index(x, y - 1),
                                Board::coordinates_to_index(x, y - 0),
                            ],
                            category: found.1,
                        });
                        continue;
                    }
                }
            }
        }
        patterns
    }

    // Get the list of *all* patterns on the board for a given player
    // Create a sliding window of length 6 and advance by 1 case at a time
    // Check each patterns inside the window and returns the first one
    // -- patterns are ordered by size
    pub fn get_patterns(board: &Board, player: &Player) -> Vec<Pattern> {
        let mut patterns: Vec<Pattern> = vec![];
        patterns.append(&mut Computer::get_horizontal_patterns(board, player));
        patterns.append(&mut Computer::get_vertical_patterns(board, player));
        patterns.append(&mut Computer::get_diagonal_left_patterns(board, player));
        patterns.append(&mut Computer::get_diagonal_right_patterns(board, player));
        patterns
    }

    pub fn get_patterns_count(board: &Board, player: &Player) -> PatternCount {
        let mut pattern_count = PatternCount::default();
        let patterns = Computer::get_patterns(board, player);
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
    pub fn compute_pattern_score(
        self_patterns: &PatternCount,
        other_patterns: &PatternCount,
    ) -> i64 {
        let mut score: i64 = 0;
        if self_patterns.five_in_row > 0 {
            score += 100000;
        }
        // ?
        // if other_patterns.dead_four > 0 {
        //     score += 25000;
        // }
        if self_patterns.live_four > 0 {
            score += 15000;
        }
        if self_patterns.live_three >= 1
            || other_patterns.dead_four == 2
            || other_patterns.dead_four == 1
        {
            score += 10000;
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
    pub fn evaluate_board(board: &Board) -> (i64, i64) {
        let black_patterns = Computer::get_patterns_count(board, &Player::Black);
        let white_patterns = Computer::get_patterns_count(board, &Player::White);
        (
            Computer::compute_pattern_score(&black_patterns, &white_patterns),
            Computer::compute_pattern_score(&white_patterns, &black_patterns),
        )
    }

    /*fn minimax(
        &mut self,
        rules: &RuleSet,
        board: &Board,
        depth: usize,
        player: &Player,
        maximize: &Player,
    ) -> Result<MiniMaxEvaluation, String> {
        if depth == 0 || board.is_winning(rules, player) {
            let scores = Computer::evaluate_board(board);
            // println!("{}", board);
            // println!("--- {}", score);
            return Ok(MiniMaxEvaluation {
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
            let mut max_eval = MiniMaxEvaluation {
                score: i64::min_value(),
                movement: None,
            };
            // println!(
            //     "examining {} moves",
            //     board.intersections_legal_moves(rules, player).len()
            // );
            let mut moves: Vec<(Board, Move)> = board
                .intersections_legal_moves(rules, player)
                .iter()
                .map(|movement| {
                    let new_board = board.apply_move(rules, movement);
                    (new_board, *movement)
                })
                .collect::<Vec<(Board, Move)>>();
            moves.sort_by(|a, b| {
                Computer::evaluate_board(&a.0).cmp(&Computer::evaluate_board(&b.0))
            });
            for (new_board, movement) in moves.iter() {
                // println!(
                //     "depth {} -- checking move {} for {:#?}",
                //     depth - 1,
                //     movement.index,
                //     movement.player
                // );
                let eval = self.minimax(rules, &new_board, depth - 1, other_player, maximize)?;
                if eval.score > max_eval.score {
                    max_eval.score = eval.score;
                    max_eval.movement = Some(movement.clone());
                }
            }
            return Ok(max_eval);
        } else {
            let mut min_eval = MiniMaxEvaluation {
                score: i64::max_value(),
                movement: None,
            };
            let mut moves: Vec<(Board, Move)> = board
                .intersections_legal_moves(rules, player)
                .iter()
                .map(|movement| {
                    let new_board = board.apply_move(rules, movement);
                    (new_board, *movement)
                })
                .collect::<Vec<(Board, Move)>>();
            moves.sort_by(|a, b| {
                Computer::evaluate_board(&b.0).cmp(&Computer::evaluate_board(&a.0))
            });
            for (new_board, movement) in moves.iter() {
                let new_board = board.apply_move(rules, movement);
                let eval = self.minimax(rules, &new_board, depth - 1, other_player, maximize)?;
                if eval.score < min_eval.score {
                    min_eval.score = eval.score;
                    min_eval.movement = Some(movement.clone());
                }
            }
            return Ok(min_eval);
        }
    }

    fn minimax_alpha_beta(
        &mut self,
        rules: &RuleSet,
        board: &Board,
        depth: usize,
        alpha: i64,
        beta: i64,
        player: &Player,
        maximize: &Player,
    ) -> Result<MiniMaxEvaluation, String> {
        if depth == 0 || board.is_winning(rules, player) {
            let scores = Computer::evaluate_board(board);
            return Ok(MiniMaxEvaluation {
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
            let mut best_eval = MiniMaxEvaluation {
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
            let mut best_eval = MiniMaxEvaluation {
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

    fn negamax_alpha_beta(
        &mut self,
        rules: &RuleSet,
        board: &Board,
        depth: usize,
        alpha: i64,
        beta: i64,
        player: &Player,
        maximize: &Player,
    ) -> Result<MiniMaxEvaluation, String> {
        let alpha_orig = alpha;
        let mut alpha = alpha;
        let mut beta = beta;

        // Check cache to see if the board was already computed
        // if self.cache.contains_key(&board.pieces) {
        //     let (black_heuristic, white_heuristic) = self.cache.get(&board.pieces).unwrap();
        //     if player == &Player::Black {
        //         if *black_heuristic > alpha {
        //             alpha = *black_heuristic;
        //         }
        //         if *white_heuristic < beta {
        //             beta = *white_heuristic;
        //         }
        //         if alpha >= beta {
        //             let color = if player == maximize { 1 } else { -1 };
        //             return Ok(MiniMaxEvaluation {
        //                 score: color * black_heuristic,
        //                 movement: None,
        //             });
        //         }
        //     } else {
        //         // ?
        //     }
        // }

        // Check if it's a leaf and compute it's value
        if depth == 0 || board.is_winning(rules, player) {
            // println!("{}", board);
            let color = if player == maximize { 1 } else { -1 };
            let scores = self.cache.get(&board.pieces).unwrap();
            return Ok(MiniMaxEvaluation {
                score: color
                    * if player == &Player::Black {
                        scores.0
                    } else {
                        scores.1
                    },
                movement: None,
            });
        }

        // Only the best evaluation is returned
        let mut best_eval = MiniMaxEvaluation {
            score: i64::min_value(),
            movement: None,
        };

        // Sort each neighbors movements and add them to the cache
        let mut moves: Vec<(Board, Move)> = board
            .intersections_legal_moves(rules, player)
            .iter()
            .map(|movement| {
                let new_board = board.apply_move(rules, movement);
                (new_board, *movement)
            })
            .collect::<Vec<(Board, Move)>>();
        if player == &Player::Black {
            moves.sort_by(|a, b| {
                if !self.cache.contains_key(&a.0.pieces) {
                    let scores = Computer::evaluate_board(&a.0);
                    self.cache.insert(a.0.pieces.clone(), scores);
                }
                if !self.cache.contains_key(&b.0.pieces) {
                    let scores = Computer::evaluate_board(&b.0);
                    self.cache.insert(b.0.pieces.clone(), scores);
                }
                self.cache
                    .get(&a.0.pieces)
                    .unwrap()
                    .0
                    .cmp(&self.cache.get(&b.0.pieces).unwrap().0)
            });
        } else {
            moves.sort_by(|a, b| {
                if !self.cache.contains_key(&a.0.pieces) {
                    let scores = Computer::evaluate_board(&a.0);
                    self.cache.insert(a.0.pieces.clone(), scores);
                }
                if !self.cache.contains_key(&b.0.pieces) {
                    let scores = Computer::evaluate_board(&b.0);
                    self.cache.insert(b.0.pieces.clone(), scores);
                }
                self.cache
                    .get(&a.0.pieces)
                    .unwrap()
                    .1
                    .cmp(&self.cache.get(&b.0.pieces).unwrap().1)
            });
        }

        // Iterate each (sorted) moves
        for (new_board, movement) in moves.iter() {
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
            if eval.score > alpha {
                alpha = eval.score;
                best_eval.score = eval.score;
                best_eval.movement = Some(movement.clone());
            }
            if alpha >= beta {
                break;
            }
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
    ) -> Result<MiniMaxEvaluation, String> {
        let alpha = i64::min_value();
        let beta = i64::max_value();
        let best_move =
            self.negamax_alpha_beta(rules, board, depth, alpha, beta, player, player)?;
        Ok(best_move)
    }
}
