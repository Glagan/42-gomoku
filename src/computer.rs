use crate::pattern::PATTERN_FINDER;
use crate::{
    board::{Board, Move, Pawn, BOARD_PIECES},
    pattern::PatternCount,
    player::Player,
    rules::RuleSet,
};
use colored::Colorize;
use std::{collections::HashMap, fmt};

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
    // (black_heuristic, white_heuristic)
    pub black_cache: HashMap<[Pawn; BOARD_PIECES as usize], CacheEntry>,
    pub white_cache: HashMap<[Pawn; BOARD_PIECES as usize], CacheEntry>,
}

impl Default for Computer {
    fn default() -> Self {
        Computer {
            black_cache: HashMap::new(),
            white_cache: HashMap::new(),
        }
    }
}

impl Computer {
    pub fn clean(&mut self) {
        // TODO: Pre-calculate first three rounds cache to avoid slow startups ?
        self.black_cache = HashMap::new();
        self.white_cache = HashMap::new();
    }

    // Calculate all patterns for a both players and return the board score
    pub fn evaluate_board(&self, board: &Board, player: &Player) -> i64 {
        PATTERN_FINDER.player_score(board, player)
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
