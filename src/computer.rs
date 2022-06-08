use crate::{
    board::{Board, Move},
    heuristic::HEURISTIC,
    patterns::PatternCount,
    player::Player,
    rules::RuleSet,
};
use colored::Colorize;
use std::{cmp::Ordering, collections::BinaryHeap, fmt};

#[derive(Debug, Clone)]
pub struct SortedMove {
    pub movement: Move,
    pub pattern_count: PatternCount,
    pub best_pattern: u8,
}

impl Eq for SortedMove {}

impl PartialEq for SortedMove {
    fn eq(&self, other: &Self) -> bool {
        self.best_pattern == other.best_pattern
    }
}

impl Ord for SortedMove {
    fn cmp(&self, other: &Self) -> Ordering {
        self.best_pattern.cmp(&other.best_pattern)
    }
}

impl PartialOrd for SortedMove {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
pub struct Evaluation {
    pub score: i32,
    pub movements: Vec<Move>,
}

impl fmt::Display for Evaluation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(movement) = self.movements.first() {
            write!(f, "score {} movement {}", self.score, movement)
        } else {
            write!(f, "score {} {}", self.score, "without movement !".red())
        }
    }
}

pub struct AlphaBetaIteration {
    depth: usize,
    alpha: i32,
    beta: i32,
}

pub struct MinimaxAction<'a> {
    board: &'a mut Board,
    movement: Option<&'a Move>,
    patterns: Option<&'a PatternCount>,
}

#[derive(Default, Clone)]
pub struct Computer;

impl Computer {
    // Calculate the patterns created by a movement and return it's score
    pub fn evaluate_action(&self, action: &MinimaxAction) -> i32 {
        HEURISTIC.patterns_score(action.patterns.as_ref().unwrap())
    }

    // * Minimax functions

    #[cfg(not(feature = "negamax"))]
    fn minimax_alpha_beta(
        &self,
        rules: &RuleSet,
        action: MinimaxAction,
        iteration: AlphaBetaIteration,
        player: Player,
        maximize: Player,
    ) -> Result<Evaluation, String> {
        let mut alpha = iteration.alpha;
        let mut beta = iteration.beta;

        // Check if it's a leaf and compute it's value
        // The current action is a movement for the *other* player
        // -- so we need to check if the *other* player is winning
        let is_leaf = iteration.depth == 0
            || if let Some(movement) = action.movement {
                action.board.is_winning(rules, movement.player)
            } else {
                false
            };
        if is_leaf {
            if action.movement.is_none() {
                return Err("Empty movement in minimax leaf".to_string());
            }
            let score = self.evaluate_action(&action);
            return Ok(Evaluation {
                score,
                movements: vec![],
            });
        }

        // Generate moves
        let mut moves: BinaryHeap<SortedMove> = action
            .board
            .intersections_legal_moves(rules, player)
            .iter()
            .map(|&movement| {
                let captures = action.board.set_move(rules, &movement);
                let pattern_count =
                    HEURISTIC.count_movement_patterns(rules, action.board, &movement, captures);
                action.board.undo_move(rules, &movement);
                SortedMove {
                    movement,
                    best_pattern: pattern_count.best_pattern(),
                    pattern_count,
                }
            })
            .rev()
            .collect();

        // Check if there is no moves remaining
        if moves.is_empty() {
            if action.movement.is_none() {
                return Ok(Evaluation {
                    score: 0,
                    movements: vec![],
                });
            } else {
                let score = self.evaluate_action(&action);
                return Ok(Evaluation {
                    score,
                    movements: vec![],
                });
            }
        }

        // Optimise for player ...
        if player == maximize {
            let mut best_eval = Evaluation {
                score: i32::min_value() + 1,
                movements: vec![],
            };
            while let Some(sorted_movement) = moves.pop() {
                action.board.set_move(rules, &sorted_movement.movement);
                let eval = self.minimax_alpha_beta(
                    rules,
                    MinimaxAction {
                        board: action.board,
                        movement: Some(&sorted_movement.movement),
                        patterns: Some(&sorted_movement.pattern_count),
                    },
                    AlphaBetaIteration {
                        depth: iteration.depth - 1,
                        alpha,
                        beta,
                    },
                    player.opponent(),
                    maximize,
                )?;
                action.board.undo_move(rules, &sorted_movement.movement);
                if eval.score > best_eval.score {
                    best_eval.score = eval.score;
                    best_eval.movements = eval.movements;
                    best_eval.movements.insert(0, sorted_movement.movement);
                }
                if best_eval.score > alpha {
                    alpha = best_eval.score;
                }
                if best_eval.score >= beta {
                    break;
                }
            }
            Ok(best_eval)
        }
        // ... or for the opponent
        else {
            let mut best_eval = Evaluation {
                score: i32::max_value(),
                movements: vec![],
            };
            while let Some(sorted_movement) = moves.pop() {
                action.board.set_move(rules, &sorted_movement.movement);
                let eval = self.minimax_alpha_beta(
                    rules,
                    MinimaxAction {
                        board: action.board,
                        movement: Some(&sorted_movement.movement),
                        patterns: Some(&sorted_movement.pattern_count),
                    },
                    AlphaBetaIteration {
                        depth: iteration.depth - 1,
                        alpha,
                        beta,
                    },
                    player.opponent(),
                    maximize,
                )?;
                action.board.undo_move(rules, &sorted_movement.movement);
                if eval.score < best_eval.score {
                    best_eval.score = eval.score;
                    best_eval.movements = eval.movements;
                    best_eval.movements.insert(0, sorted_movement.movement);
                }
                if best_eval.score < beta {
                    beta = best_eval.score;
                }
                if best_eval.score <= alpha {
                    break;
                }
            }
            Ok(best_eval)
        }
    }

    // Use the minimax algorithm with alpha beta prunning to get the next best move
    #[cfg(not(feature = "negamax"))]
    pub fn play(
        &self,
        rules: &RuleSet,
        board: &mut Board,
        depth: usize,
        player: Player,
    ) -> Result<Evaluation, String> {
        // Apply minimax recursively
        let best_move = self.minimax_alpha_beta(
            rules,
            MinimaxAction {
                board,
                movement: None,
                patterns: None,
            },
            AlphaBetaIteration {
                depth,
                alpha: i32::min_value() + 1,
                beta: i32::max_value(),
            },
            player,
            player,
        )?;

        Ok(best_move)
    }

    // * Negamax functions

    #[cfg(feature = "negamax")]
    fn negamax_alpha_beta(
        &self,
        rules: &RuleSet,
        action: MinimaxAction,
        iteration: AlphaBetaIteration,
        player: Player,
        color: i32,
    ) -> Result<Evaluation, String> {
        let mut alpha = iteration.alpha;
        let beta = iteration.beta;

        // Check if it's a leaf and compute it's valuelet is_leaf = iteration.depth == 0
        let is_leaf = iteration.depth == 0
            || if let Some(movement) = action.movement {
                action.board.is_winning(rules, movement.player)
            } else {
                false
            };
        if is_leaf {
            if action.movement.is_none() {
                return Err("Empty movement in negamax leaf".to_string());
            }
            let score = self.evaluate_action(&action);
            return Ok(Evaluation {
                score: color * score,
                movements: vec![],
            });
        }

        // Only the best evaluation is returned
        let mut best_eval = Evaluation {
            score: i32::min_value() + 1,
            movements: vec![],
        };

        // Iterate each neighbor moves
        let mut moves: BinaryHeap<SortedMove> = action
            .board
            .intersections_legal_moves(rules, player)
            .iter()
            .map(|&movement| {
                let captures = action.board.set_move(rules, &movement);
                let pattern_count =
                    HEURISTIC.count_movement_patterns(rules, action.board, &movement, captures);
                action.board.undo_move(rules, &movement);
                SortedMove {
                    movement,
                    best_pattern: pattern_count.best_pattern(),
                    pattern_count,
                }
            })
            .rev()
            .collect();

        // Check if there is no moves remaining
        if moves.is_empty() {
            if action.movement.is_none() {
                return Ok(Evaluation {
                    score: 0,
                    movements: vec![],
                });
            } else {
                let score = self.evaluate_action(&action);
                return Ok(Evaluation {
                    score: color * score,
                    movements: vec![],
                });
            }
        }

        while let Some(sorted_movement) = moves.pop() {
            action.board.set_move(rules, &sorted_movement.movement);
            let eval = self.negamax_alpha_beta(
                rules,
                MinimaxAction {
                    board: action.board,
                    movement: Some(&sorted_movement.movement),
                    patterns: Some(&sorted_movement.pattern_count),
                },
                AlphaBetaIteration {
                    depth: iteration.depth - 1,
                    alpha: -beta,
                    beta: -alpha,
                },
                player.opponent(),
                -color,
            )?;
            action.board.undo_move(rules, &sorted_movement.movement);
            let score = -eval.score;
            if score > best_eval.score {
                alpha = score;
                best_eval.score = score;
                best_eval.movements = eval.movements;
                best_eval.movements.insert(0, sorted_movement.movement);
                if alpha >= beta {
                    break;
                }
            }
        }

        Ok(best_eval)
    }

    // Use the negamax algorithm with alpha beta prunning to get the next best move
    #[cfg(feature = "negamax")]
    pub fn play(
        &self,
        rules: &RuleSet,
        board: &mut Board,
        depth: usize,
        player: Player,
    ) -> Result<Evaluation, String> {
        // Apply negamax recursively
        let best_move = self.negamax_alpha_beta(
            rules,
            MinimaxAction {
                board,
                movement: None,
                patterns: None,
            },
            AlphaBetaIteration {
                depth,
                alpha: i32::min_value() + 1,
                beta: i32::max_value(),
            },
            player,
            1,
        )?;

        Ok(best_move)
    }
}
