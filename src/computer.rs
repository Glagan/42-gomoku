use crate::pattern::{Pattern, PATTERN_FINDER};
use crate::{
    board::{Board, Move, Pawn, BOARD_PIECES},
    player::Player,
    rules::RuleSet,
};
use colored::Colorize;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fmt,
};

#[derive(Debug, Clone)]
pub struct SortedMove {
    pub movement: Move,
    pub pattern: Option<Pattern>,
}

impl Eq for SortedMove {}

impl PartialEq for SortedMove {
    fn eq(&self, other: &Self) -> bool {
        self.pattern == other.pattern
    }
}

impl Ord for SortedMove {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_is_none = self.pattern.is_none();
        let other_is_none = other.pattern.is_none();
        if self_is_none && other_is_none {
            return std::cmp::Ordering::Equal;
        } else if self_is_none {
            return std::cmp::Ordering::Less;
        } else if other_is_none {
            return std::cmp::Ordering::Greater;
        }
        other
            .pattern
            .unwrap()
            .partial_cmp(&self.pattern.unwrap())
            .unwrap()
    }
}

impl PartialOrd for SortedMove {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy)]
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
    pub moves: u16,
    pub flag: CacheFlag,
    pub movement: Option<Move>,
}

pub struct AlphaBetaIteration {
    depth: usize,
    alpha: i64,
    beta: i64,
}

pub struct MinimaxAction<'a> {
    board: &'a Board,
    movement: Option<Move>,
}

#[derive(Default)]
pub struct Computer {
    // (black_heuristic, white_heuristic)
    pub black_cache: HashMap<[Pawn; BOARD_PIECES as usize], CacheEntry>,
    pub white_cache: HashMap<[Pawn; BOARD_PIECES as usize], CacheEntry>,
}

impl Computer {
    pub fn clean(&mut self) {
        // TODO: Pre-calculate first three rounds cache to avoid slow startups ?
        self.black_cache = HashMap::new();
        self.white_cache = HashMap::new();
    }

    // Calculate the patterns created by a movement and return it's score
    pub fn evaluate_action(&self, action: &MinimaxAction) -> i64 {
        PATTERN_FINDER.movement_score(action.board, &action.movement.unwrap())
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

    // #[allow(dead_code)]
    // fn minimax(
    //     &mut self,
    //     rules: &RuleSet,
    //     action: MinimaxAction,
    //     depth: usize,
    //     player: &Player,
    //     maximize: &Player,
    // ) -> Result<Evaluation, String> {
    //     // Check cache to see if the board was already computed
    //     if self.cache(player).contains_key(&action.board.pieces) {
    //         let cache_entry = self.cache(player).get(&action.board.pieces).unwrap();
    //         if cache_entry.rocks >= action.board.rocks {
    //             return Ok(Evaluation {
    //                 score: cache_entry.score,
    //                 movement: cache_entry.movement,
    //             });
    //         }
    //     }

    //     // Check if it's a leaf and compute it's value
    //     if depth == 0 || action.board.is_winning(rules, player) {
    //         if action.movement.is_none() {
    //             return Err("Empty movement in negamax leaf".to_string());
    //         }
    //         let score = self.evaluate_action(&action);
    //         return Ok(Evaluation {
    //             score,
    //             movement: None,
    //         });
    //     }

    //     // Iterate each neighbor moves
    //     let other_player = if *player == Player::Black {
    //         &Player::White
    //     } else {
    //         &Player::Black
    //     };
    //     if player == maximize {
    //         let mut max_eval = Evaluation {
    //             score: i64::min_value() + 1,
    //             movement: None,
    //         };
    //         for movement in action.board.intersections_legal_moves(rules, player).iter() {
    //             let new_board = action.board.apply_move(rules, movement);
    //             let eval = self.minimax(
    //                 rules,
    //                 MinimaxAction {
    //                     board: &new_board,
    //                     movement: Some(*movement),
    //                 },
    //                 depth - 1,
    //                 other_player,
    //                 maximize,
    //             )?;
    //             if eval.score > max_eval.score {
    //                 max_eval.score = eval.score;
    //                 max_eval.movement = Some(*movement);
    //             }
    //         }

    //         // Add to cache
    //         self.cache(player)
    //             .entry(action.board.pieces)
    //             .or_insert(CacheEntry {
    //                 score: max_eval.score,
    //                 rocks: action.board.rocks,
    //                 flag: CacheFlag::Exact,
    //                 movement: max_eval.movement,
    //             });

    //         Ok(max_eval)
    //     } else {
    //         let mut min_eval = Evaluation {
    //             score: i64::max_value(),
    //             movement: None,
    //         };
    //         for movement in action.board.intersections_legal_moves(rules, player).iter() {
    //             let new_board = action.board.apply_move(rules, movement);
    //             let eval = self.minimax(
    //                 rules,
    //                 MinimaxAction {
    //                     board: &new_board,
    //                     movement: Some(*movement),
    //                 },
    //                 depth - 1,
    //                 other_player,
    //                 maximize,
    //             )?;
    //             if eval.score < min_eval.score {
    //                 min_eval.score = eval.score;
    //                 min_eval.movement = Some(*movement);
    //             }
    //         }

    //         // Add to cache
    //         self.cache(player)
    //             .entry(action.board.pieces)
    //             .or_insert(CacheEntry {
    //                 score: min_eval.score,
    //                 rocks: action.board.rocks,
    //                 flag: CacheFlag::Exact,
    //                 movement: min_eval.movement,
    //             });

    //         Ok(min_eval)
    //     }
    // }

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

    fn negamax_alpha_beta(
        &mut self,
        rules: &RuleSet,
        action: MinimaxAction,
        iteration: AlphaBetaIteration,
        player: &Player,
        maximize: &Player,
    ) -> Result<Evaluation, String> {
        let alpha_orig = iteration.alpha;
        let mut alpha = iteration.alpha;
        let mut beta = iteration.beta;

        // Check cache to see if the board was already computed
        if self.cache(player).contains_key(&action.board.pieces) {
            let cache_entry = self.cache(player).get(&action.board.pieces).unwrap();
            if cache_entry.moves >= action.board.moves {
                if cache_entry.flag == CacheFlag::Exact {
                    return Ok(Evaluation {
                        score: cache_entry.score,
                        movement: cache_entry.movement,
                    });
                } else if cache_entry.flag == CacheFlag::Lowerbound {
                    if cache_entry.score > alpha {
                        alpha = cache_entry.score
                    }
                } else if cache_entry.flag == CacheFlag::Upperbound && cache_entry.score < beta {
                    beta = cache_entry.score
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
        if iteration.depth == 0 || action.board.is_winning(rules, player) {
            if action.movement.is_none() {
                return Err("Empty movement in negamax leaf".to_string());
            }
            let color = if player == maximize { 1 } else { -1 };
            let score = self.evaluate_action(&action);
            return Ok(Evaluation {
                score: color * score,
                movement: None,
            });
        }

        // Only the best evaluation is returned
        let mut best_eval = Evaluation {
            score: i64::min_value() + 1,
            movement: None,
        };

        // Iterate each neighbor moves
        let mut moves: BinaryHeap<SortedMove> = action
            .board
            .intersections_legal_moves(rules, player)
            .iter()
            .map(|&movement| SortedMove {
                movement,
                pattern: PATTERN_FINDER.best_pattern_for_rock(action.board, movement.index),
            })
            .collect();
        while let Some(sorted_movement) = moves.pop() {
            let new_board = action.board.apply_move(rules, &sorted_movement.movement);
            let mut eval = self.negamax_alpha_beta(
                rules,
                MinimaxAction {
                    board: &new_board,
                    movement: Some(sorted_movement.movement),
                },
                AlphaBetaIteration {
                    depth: iteration.depth - 1,
                    alpha: -beta,
                    beta: -alpha,
                },
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
                best_eval.movement = Some(sorted_movement.movement);
            }
            if alpha >= beta {
                break;
            }
        }

        // Add to cache
        let cache_entry = self
            .cache(player)
            .entry(action.board.pieces)
            .or_insert(CacheEntry {
                score: if player == maximize {
                    best_eval.score
                } else {
                    -best_eval.score
                },
                moves: action.board.moves,
                flag: CacheFlag::Exact,
                movement: best_eval.movement,
            });
        if best_eval.score <= alpha_orig {
            cache_entry.flag = CacheFlag::Upperbound;
        } else if best_eval.score >= beta {
            cache_entry.flag = CacheFlag::Lowerbound;
        }

        Ok(best_eval)
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
        self.black_cache.retain(|_, v| v.moves >= board.moves);
        self.white_cache.retain(|_, v| v.moves >= board.moves);

        // Apply negamax recursively d);
        let best_move = self.negamax_alpha_beta(
            rules,
            MinimaxAction {
                board,
                movement: None,
            },
            AlphaBetaIteration {
                depth,
                alpha: i64::min_value() + 1,
                beta: i64::max_value(),
            },
            player,
            player,
        )?;

        // Apply minimax recursively
        // let best_move = self.minimax(
        //     rules,
        //     MinimaxAction {
        //         board,
        //         movement: None,
        //     },
        //     depth,
        //     player,
        //     player,
        // )?;

        // branch.display(0);
        Ok(best_move)
    }
}
