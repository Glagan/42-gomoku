use crate::{
    board::{Board, Move},
    constants::DEPTH,
    heuristic::HEURISTIC,
    patterns::PatternCount,
    player::Player,
    rules::RuleSet,
};
use colored::Colorize;
use std::{cmp::Ordering, collections::BinaryHeap, fmt, time::Instant};

#[derive(Debug, Clone)]
pub enum Algorithm {
    Negamax,
    Minimax,
    Greedy,
}

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

    // * Negamax function

    fn negamax_alpha_beta(
        &self,
        rules: &RuleSet,
        action: MinimaxAction,
        iteration: AlphaBetaIteration,
        player: Player,
        color: i32,
        start_time: Instant,
    ) -> Result<Evaluation, String> {
        let mut alpha = iteration.alpha;
        let beta = iteration.beta;

        // Check if it's a leaf and compute it's valuelet is_leaf = iteration.depth == 0
        let is_leaf = iteration.depth == 0
            || if let Some(movement) = action.movement {
                action.board.is_winning(rules, movement.player)
                    || action.board.is_winning(rules, movement.player.opponent())
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

        // Only the best evaluation is returned0
        let mut best_eval = Evaluation {
            score: i32::min_value() + 1,
            movements: vec![],
        };

        // Iterate each neighbor moves
        let intersections = action.board.intersections_legal_moves(rules, player);
        let mut moves: BinaryHeap<SortedMove> = BinaryHeap::new();
        moves.reserve(intersections.len());
        for movement in intersections {
            let captures = action.board.set_move(rules, &movement);
            let pattern_count =
                HEURISTIC.count_movement_patterns(rules, action.board, &movement, captures);
            action.board.undo_move(rules, &movement);
            moves.push(SortedMove {
                movement,
                best_pattern: pattern_count.best_pattern(),
                pattern_count,
            });
        }

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
                start_time,
            )?;
            action.board.undo_move(rules, &sorted_movement.movement);
            let score = -eval.score;
            if score > best_eval.score {
                alpha = score;
                best_eval.score = score;
                best_eval.movements = eval.movements;
                best_eval.movements.insert(0, sorted_movement.movement);
                if alpha >= beta {
                    return Ok(best_eval);
                }
            }
            if start_time.elapsed().as_millis() > 480 && iteration.depth == DEPTH {
                break;
            }
        }

        Ok(best_eval)
    }

    // * Minimax function

    fn minimax_alpha_beta(
        &self,
        rules: &RuleSet,
        action: MinimaxAction,
        iteration: AlphaBetaIteration,
        player: Player,
        maximize: bool,
        start_time: Instant,
    ) -> Result<Evaluation, String> {
        let mut alpha = iteration.alpha;
        let mut beta = iteration.beta;

        // Check if it's a leaf and compute it's value
        // The current action is a movement for the *other* player
        // -- so we need to check if the *other* player is winning
        let is_leaf = iteration.depth == 0
            || if let Some(movement) = action.movement {
                action.board.is_winning(rules, movement.player)
                    || action.board.is_winning(rules, movement.player.opponent())
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
        let intersections = action.board.intersections_legal_moves(rules, player);
        let mut moves: BinaryHeap<SortedMove> = BinaryHeap::new();
        moves.reserve(intersections.len());
        for movement in intersections {
            let captures = action.board.set_move(rules, &movement);
            let pattern_count =
                HEURISTIC.count_movement_patterns(rules, action.board, &movement, captures);
            action.board.undo_move(rules, &movement);
            moves.push(SortedMove {
                movement,
                best_pattern: pattern_count.best_pattern(),
                pattern_count,
            });
        }

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
        if maximize {
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
                    !maximize,
                    start_time,
                )?;
                action.board.undo_move(rules, &sorted_movement.movement);
                if eval.score > best_eval.score {
                    best_eval.score = eval.score;
                    best_eval.movements = eval.movements;
                    best_eval.movements.insert(0, sorted_movement.movement);
                }
                if eval.score > alpha {
                    alpha = eval.score;
                }
                if eval.score >= beta {
                    return Ok(best_eval);
                }
                if start_time.elapsed().as_millis() > 480 && iteration.depth == DEPTH {
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
                    !maximize,
                    start_time,
                )?;
                action.board.undo_move(rules, &sorted_movement.movement);
                if eval.score < best_eval.score {
                    best_eval.score = eval.score;
                    best_eval.movements = eval.movements;
                    best_eval.movements.insert(0, sorted_movement.movement);
                }
                if eval.score < beta {
                    beta = eval.score;
                }
                if eval.score <= alpha {
                    return Ok(best_eval);
                }
                if start_time.elapsed().as_millis() > 480 && iteration.depth == DEPTH {
                    break;
                }
            }
            Ok(best_eval)
        }
    }

    // * Greedy function

    fn greedy(
        &self,
        rules: &RuleSet,
        action: MinimaxAction,
        depth: usize,
        player: Player,
    ) -> Result<Evaluation, String> {
        // Check if it's a leaf and compute it's value
        // The current action is a movement for the *other* player
        // -- so we need to check if the *other* player is winning
        let is_leaf = depth == 0
            || if let Some(movement) = action.movement {
                action.board.is_winning(rules, movement.player)
                    || action.board.is_winning(rules, movement.player.opponent())
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
        let intersections = action.board.intersections_legal_moves(rules, player);
        let mut moves: BinaryHeap<SortedMove> = BinaryHeap::new();
        moves.reserve(intersections.len());
        for movement in intersections {
            let captures = action.board.set_move(rules, &movement);
            let pattern_count =
                HEURISTIC.count_movement_patterns(rules, action.board, &movement, captures);
            action.board.undo_move(rules, &movement);
            moves.push(SortedMove {
                movement,
                best_pattern: pattern_count.best_pattern(),
                pattern_count,
            });
        }

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

        // Select the best move up to DEPTH
        let first_movement = moves.pop().unwrap();
        action.board.set_move(rules, &first_movement.movement);
        let eval = self.greedy(
            rules,
            MinimaxAction {
                board: action.board,
                movement: Some(&first_movement.movement),
                patterns: Some(&first_movement.pattern_count),
            },
            depth - 1,
            player.opponent(),
        )?;
        action.board.undo_move(rules, &first_movement.movement);
        let mut merged_eval = Evaluation {
            score: eval.score,
            movements: eval.movements,
        };
        merged_eval.movements.insert(0, first_movement.movement);
        Ok(merged_eval)
    }

    pub fn play(
        &self,
        algorithm: Algorithm,
        rules: &RuleSet,
        board: &mut Board,
        depth: usize,
        player: Player,
    ) -> Result<Evaluation, String> {
        Ok(match algorithm {
            // Use the negamax algorithm with alpha beta prunning to get the next best move
            Algorithm::Negamax => self.negamax_alpha_beta(
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
                Instant::now(),
            )?,
            // Use the minimax algorithm with alpha beta prunning to get the next best move
            Algorithm::Minimax => self.minimax_alpha_beta(
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
                true,
                Instant::now(),
            )?,
            // Select only the next move from the heuristic
            Algorithm::Greedy => self.greedy(
                rules,
                MinimaxAction {
                    board,
                    movement: None,
                    patterns: None,
                },
                depth,
                player,
            )?,
        })
    }
}
