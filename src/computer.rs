use crate::{
    board::{Board, Move, Pawn, BOARD_SIZE},
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
// const PATTERNS: [([usize; 6], PatternCategory); 77] = [
const PATTERNS: [([usize; 6], PatternCategory); 34] = [
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
    // ([1, 1, 1, 2, 0, 0], PatternCategory::DeadThree),
    // ([0, 1, 1, 1, 2, 0], PatternCategory::DeadThree),
    // ([0, 0, 1, 1, 1, 2], PatternCategory::DeadThree),
    // ([2, 1, 1, 1, 0, 0], PatternCategory::DeadThree),
    // ([0, 2, 1, 1, 1, 0], PatternCategory::DeadThree),
    // ([0, 0, 2, 1, 1, 1], PatternCategory::DeadThree),
    // // 2x2
    // ([1, 0, 1, 1, 2, 0], PatternCategory::DeadThree),
    // ([0, 1, 0, 1, 1, 2], PatternCategory::DeadThree),
    // ([2, 1, 1, 0, 1, 0], PatternCategory::DeadThree),
    // ([0, 2, 1, 1, 0, 1], PatternCategory::DeadThree),
    // // 3x2
    // ([2, 1, 0, 1, 1, 0], PatternCategory::DeadThree),
    // ([0, 2, 1, 0, 1, 1], PatternCategory::DeadThree),
    // ([1, 1, 0, 1, 2, 0], PatternCategory::DeadThree),
    // ([0, 1, 1, 0, 1, 2], PatternCategory::DeadThree),
    // // 4x2
    // ([1, 0, 0, 1, 1, 0], PatternCategory::DeadThree),
    // ([0, 1, 0, 0, 1, 1], PatternCategory::DeadThree),
    // ([1, 1, 0, 0, 1, 0], PatternCategory::DeadThree),
    // ([0, 1, 1, 0, 0, 1], PatternCategory::DeadThree),
    // // 5x2
    // ([1, 0, 1, 0, 1, 0], PatternCategory::DeadThree),
    // ([0, 1, 0, 1, 0, 1], PatternCategory::DeadThree),
    // // 6x2
    // // ([2, 0, 1, 1, 1, 0, 2], PatternCategory::DeadThree),
    // // 7x2
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
    // // 3x3
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
    pub rules: RuleSet,
    pub player: Player,
}

impl Computer {
    pub fn new(rules: &RuleSet, player: &Player) -> Computer {
        Computer {
            rules: *rules,
            player: *player,
        }
    }

    pub fn pawn_to_pattern_pawn(board: &Board, x: usize, y: usize, player: &Player) -> usize {
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
    pub fn get_horizontal_patterns(&self, board: &Board, player: &Player) -> Vec<Pattern> {
        let mut patterns: Vec<Pattern> = vec![];
        let mut window: [usize; 6] = [0, 0, 0, 0, 0, 0];
        for y in 0..BOARD_SIZE {
            // Go trough the first 5 with an offset of 1
            // -- the next x loop will have the correct initial window
            for i in 0..5 {
                window[i + 1] = Computer::pawn_to_pattern_pawn(board, i, y, player);
            }
            for x in 5..(BOARD_SIZE - 6) {
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

    pub fn get_vertical_patterns(&self, board: &Board, player: &Player) -> Vec<Pattern> {
        let mut patterns: Vec<Pattern> = vec![];
        let mut window: [usize; 6] = [0, 0, 0, 0, 0, 0];
        for x in 0..BOARD_SIZE {
            // Go trough the first 5 with an offset of 1
            // -- the next y loop will have the correct initial window
            for i in 0..5 {
                window[i + 1] = Computer::pawn_to_pattern_pawn(board, x, i, player);
            }
            for y in 5..(BOARD_SIZE - 6) {
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

    pub fn get_diagonal_left_patterns(&self, board: &Board, player: &Player) -> Vec<Pattern> {
        let mut patterns: Vec<Pattern> = vec![];
        let mut window: [usize; 6] = [0, 0, 0, 0, 0, 0];
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

    pub fn get_diagonal_right_patterns(&self, board: &Board, player: &Player) -> Vec<Pattern> {
        let mut patterns: Vec<Pattern> = vec![];
        let mut window: [usize; 6] = [0, 0, 0, 0, 0, 0];
        for y in (BOARD_SIZE - 1)..=6 {
            // Go trough the first 5 with an offset of 1
            // -- the next x loop will have the correct initial window
            for x in (BOARD_SIZE - 1)..=6 {
                window[0] = Computer::pawn_to_pattern_pawn(board, x, y, player);
                window[1] = Computer::pawn_to_pattern_pawn(board, x - 1, y - 1, player);
                window[2] = Computer::pawn_to_pattern_pawn(board, x - 2, y - 2, player);
                window[3] = Computer::pawn_to_pattern_pawn(board, x - 3, y - 3, player);
                window[4] = Computer::pawn_to_pattern_pawn(board, x - 4, y - 4, player);
                window[5] = Computer::pawn_to_pattern_pawn(board, x - 5, y - 5, player);
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
    // TODO Get vertical patterns
    // TODO Get diagonal left and right patterns
    pub fn get_patterns(&self, board: &Board, player: &Player) -> Vec<Pattern> {
        let mut patterns: Vec<Pattern> = vec![];
        patterns.append(&mut self.get_horizontal_patterns(board, player));
        patterns.append(&mut self.get_vertical_patterns(board, player));
        patterns.append(&mut self.get_diagonal_left_patterns(board, player));
        patterns.append(&mut self.get_diagonal_right_patterns(board, player));
        patterns
    }

    // Calculate all patterns for a given player and return the board score
    // TODO
    pub fn evaluate_board(&self, board: &Board, player: &Player) -> i64 {
        let patterns = self.get_patterns(board, player);
        if !patterns.is_empty() {
            // println!("--- {} patterns", patterns.len());
            // println!("patterns {:#?}", patterns);
            let mut score: i64 = 0;
            let mut pattern_count = PatternCount::default();
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
            if pattern_count.five_in_row > 0 {
                score += 100000;
            }
            if pattern_count.live_four >= 1 {
                score += 15000;
            }
            if (pattern_count.live_three >= 2 && pattern_count.dead_four == 2)
                || (pattern_count.live_three == 1 && pattern_count.dead_four == 1)
            {
                score += 10000;
            }
            // LiveThree + jLiveThree ? Other player LiveThree ?
            if pattern_count.live_three >= 1 {
                score += 5000;
            }
            if pattern_count.dead_four > 0 {
                score += 1000;
            }
            // jDeadFour ? Other player DeadFour ?
            // CDeadfour ??
            // Debug
            if pattern_count.live_two > 0 {
                score += 200;
            }
            return score;
        }
        0
    }

    fn minimax(
        &self,
        board: &Board,
        depth: usize,
        player: &Player,
    ) -> Result<MiniMaxEvaluation, String> {
        if depth == 0 || board.is_winning(&self.rules, player) {
            let score = self.evaluate_board(board, player);
            // println!("{}", board);
            // println!("--- {}", score);
            return Ok(MiniMaxEvaluation {
                score: score,
                movement: None,
            });
        }
        let other_player = if self.player == Player::Black {
            &Player::White
        } else {
            &Player::Black
        };
        if *player == self.player {
            let mut max_eval = MiniMaxEvaluation {
                score: i64::min_value(),
                movement: None,
            };
            for movement in board.intersections_legal_moves(&self.rules, player).iter() {
                let new_board = board.apply_move(&self.rules, movement)?;
                let eval = self.minimax(&new_board, depth - 1, other_player)?;
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
            for movement in board
                .intersections_legal_moves(&self.rules, other_player)
                .iter()
            {
                let new_board = board.apply_move(&self.rules, movement)?;
                let eval = self.minimax(&new_board, depth - 1, &self.player)?;
                if eval.score < min_eval.score {
                    min_eval.score = eval.score;
                    min_eval.movement = Some(movement.clone());
                }
            }
            return Ok(min_eval);
        }
    }

    fn minimax_alpha_beta(
        &self,
        board: &Board,
        depth: usize,
        alpha: i64,
        beta: i64,
        player: &Player,
    ) -> Result<MiniMaxEvaluation, String> {
        if depth == 0 || board.is_winning(&self.rules, player) {
            return Ok(MiniMaxEvaluation {
                score: self.evaluate_board(board, player),
                movement: None,
            });
        }
        let other_player = if self.player == Player::Black {
            &Player::White
        } else {
            &Player::Black
        };
        if *player == self.player {
            let mut alpha = alpha;
            let mut best_eval = MiniMaxEvaluation {
                score: i64::min_value(),
                movement: None,
            };
            for movement in board.intersections_legal_moves(&self.rules, player).iter() {
                let new_board = board.apply_move(&self.rules, movement)?;
                let eval =
                    self.minimax_alpha_beta(&new_board, depth - 1, alpha, beta, other_player)?;
                if eval.score >= alpha {
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
                score: 0,
                movement: None,
            };
            for movement in board
                .intersections_legal_moves(&self.rules, other_player)
                .iter()
            {
                let new_board = board.apply_move(&self.rules, movement)?;
                let eval =
                    self.minimax_alpha_beta(&new_board, depth - 1, alpha, beta, &self.player)?;
                if eval.score <= beta {
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
    }

    fn negamax_alpha_beta(
        &self,
        board: &Board,
        depth: usize,
        alpha: i64,
        beta: i64,
        player: &Player,
    ) -> Result<MiniMaxEvaluation, String> {
        if depth == 0 || board.is_winning(&self.rules, player) {
            // println!("{}\n---", board);
            let color = if *player == self.player { 1 } else { -1 };
            return Ok(MiniMaxEvaluation {
                score: color * self.evaluate_board(board, player),
                movement: None,
            });
        }
        let mut best_eval = MiniMaxEvaluation {
            score: i64::min_value(),
            movement: None,
        };
        let mut alpha = alpha;
        for movement in board.intersections_legal_moves(&self.rules, player).iter() {
            let new_board = board.apply_move(&self.rules, movement)?;
            let mut eval = self.negamax_alpha_beta(
                &new_board,
                depth - 1,
                -beta,
                -alpha,
                if self.player == Player::Black {
                    &Player::White
                } else {
                    &Player::Black
                },
            )?;
            eval.score = -eval.score;
            if eval.score > alpha {
                alpha = eval.score;
                best_eval = MiniMaxEvaluation {
                    score: eval.score,
                    movement: Some(movement.clone()),
                };
            }
            if alpha >= beta {
                // return Ok(MiniMaxEvaluation {
                //     score: i64::min_value(),
                //     movement: None,
                // });
                break;
            }
        }
        return Ok(best_eval);
    }

    // Use the minimax algorithm to get the next best move
    pub fn play(&self, board: &Board, depth: usize) -> Result<MiniMaxEvaluation, String> {
        let alpha = i64::min_value();
        let beta = i64::max_value();
        let best_move = self.negamax_alpha_beta(board, depth, alpha, beta, &self.player)?;
        Ok(best_move)
    }
}
