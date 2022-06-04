use crate::{
    board::{Board, Move},
    constants::NB_THREAD,
    pattern::{PatternCount, PATTERN_FINDER},
    player::Player,
    rules::RuleSet,
};
use colored::Colorize;
use std::{cmp::Ordering, collections::BinaryHeap, fmt, sync::mpsc, thread};

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
        PATTERN_FINDER.patterns_score(action.patterns.as_ref().unwrap())
    }

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
                return Err("Empty movement in negamax leaf".to_string());
            }
            let score = self.evaluate_action(&action);
            return Ok(Evaluation {
                score,
                movement: None,
            });
        }

        // Generate moves
        let mut moves: BinaryHeap<SortedMove> = action
            .board
            .intersections_legal_moves(rules, player)
            .iter()
            .map(|&movement| {
                action.board.set_move(rules, &movement);
                let pattern_count =
                    PATTERN_FINDER.count_movement_patterns(rules, action.board, &movement);
                action.board.undo_move(rules, &movement);
                SortedMove {
                    movement,
                    best_pattern: pattern_count.best_pattern(),
                    pattern_count,
                }
            })
            .collect();
        // Optimise for player ...
        if player == maximize {
            let mut best_eval = Evaluation {
                score: i32::min_value(),
                movement: None,
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
                    // println!("best eval patterns {:#?}", sorted_movement.pattern_count);
                    best_eval.movement = Some(sorted_movement.movement);
                }
                if best_eval.score >= beta {
                    break;
                }
                if best_eval.score > alpha {
                    alpha = best_eval.score;
                }
            }
            Ok(best_eval)
        }
        // ... or for the opponent
        else {
            let mut best_eval = Evaluation {
                score: i32::max_value(),
                movement: None,
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
                    // println!("best eval patterns {:#?}", sorted_movement.pattern_count);
                    best_eval.movement = Some(sorted_movement.movement);
                }
                if best_eval.score <= alpha {
                    break;
                }
                if best_eval.score < beta {
                    beta = best_eval.score;
                }
            }
            Ok(best_eval)
        }
    }

    fn negamax_alpha_beta(
        &self,
        rules: &RuleSet,
        action: MinimaxAction,
        iteration: AlphaBetaIteration,
        player: Player,
    ) -> Result<Evaluation, String> {
        let mut alpha = iteration.alpha;
        let beta = iteration.beta;

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
                return Err("Empty movement in negamax leaf".to_string());
            }
            let score = self.evaluate_action(&action);
            return Ok(Evaluation {
                score,
                movement: None,
            });
        }

        // Only the best evaluation is returned
        let mut best_eval = Evaluation {
            score: i32::min_value() + 1,
            movement: None,
        };

        // Generate moves
        let mut moves: BinaryHeap<SortedMove> = action
            .board
            .intersections_legal_moves(rules, player)
            .iter()
            .map(|&movement| {
                action.board.set_move(rules, &movement);
                let pattern_count =
                    PATTERN_FINDER.count_movement_patterns(rules, action.board, &movement);
                action.board.undo_move(rules, &movement);
                SortedMove {
                    movement,
                    best_pattern: pattern_count.best_pattern(),
                    pattern_count,
                }
            })
            .collect();
        // Iterate each neighbor moves
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
            )?;
            action.board.undo_move(rules, &sorted_movement.movement);
            let score = -eval.score;
            if score > best_eval.score {
                best_eval.score = score;
                best_eval.movement = Some(sorted_movement.movement);
                if best_eval.score > alpha {
                    alpha = score;
                    if alpha >= beta {
                        break;
                    }
                }
            }
        }

        Ok(best_eval)
    }

    fn initial_negamax_alpha_beta(
        &self,
        rules: &RuleSet,
        action: MinimaxAction,
        iteration: AlphaBetaIteration,
        player: Player,
        mut moves: BinaryHeap<SortedMove>,
    ) -> Result<Evaluation, String> {
        let mut alpha = iteration.alpha;
        let beta = iteration.beta;

        // Only the best evaluation is returned
        let mut best_eval = Evaluation {
            score: i32::min_value() + 1,
            movement: None,
        };

        // Iterate each already generated moves
        // These moves are for the *current* player and should not decrease depth
        // -- since this is the *initial* call but the moves are already generated
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
                    depth: iteration.depth,
                    alpha,
                    beta,
                },
                player,
            )?;
            action.board.undo_move(rules, &sorted_movement.movement);
            let score = eval.score;
            if score > best_eval.score {
                best_eval.score = score;
                best_eval.movement = Some(sorted_movement.movement);
            }
            if score > alpha {
                alpha = score;
            }
            if alpha >= beta {
                break;
            }
        }

        Ok(best_eval)
    }

    pub fn get_all_first_movements_sorted(
        &self,
        rules: &RuleSet,
        board: &mut Board,
        player: Player,
    ) -> Vec<BinaryHeap<SortedMove>> {
        let mut moves: BinaryHeap<SortedMove> = board
            .intersections_legal_moves(rules, player)
            .iter()
            .map(|&movement| {
                board.set_move(rules, &movement);
                let pattern_count = PATTERN_FINDER.count_movement_patterns(rules, board, &movement);
                board.undo_move(rules, &movement);
                SortedMove {
                    movement,
                    best_pattern: pattern_count.best_pattern(),
                    pattern_count,
                }
            })
            .collect();

        // Split the moves between NB_THREAD lists
        // -- and send the NB_THREAD first (best) moves to different lists, one for each thread
        let mut sorted_list_of_moves: Vec<BinaryHeap<SortedMove>> =
            vec![BinaryHeap::new(); NB_THREAD];
        let mut index: usize = 0;
        while let Some(sorted_movement) = moves.pop() {
            sorted_list_of_moves[index % NB_THREAD].push(sorted_movement);
            index += 1;
        }
        sorted_list_of_moves
    }

    // Use the negamax algorithm (minimax variant) to get the next best move
    pub fn play(
        &self,
        rules: &RuleSet,
        board: &mut Board,
        depth: usize,
        player: Player,
    ) -> Result<Evaluation, String> {
        /* WITHOUT THREADS */
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
                alpha: i32::min_value(),
                beta: i32::max_value(),
            },
            player,
            player,
        )?;

        Ok(best_move)
        /* WITHOUT THREADS */

        /* WITHOUT THREADS */
        // // Apply negamax recursively
        // let best_move = self.negamax_alpha_beta(
        //     rules,
        //     MinimaxAction {
        //         board,
        //         movement: None,
        //         patterns: None,
        //     },
        //     AlphaBetaIteration {
        //         depth,
        //         alpha: i32::min_value() + 1,
        //         beta: i32::max_value(),
        //     },
        //     player,
        // )?;

        // Ok(best_move)
        /* WITHOUT THREADS */

        /* WITH THREADS */
        // // Get all possible moves to launch them in multiple threads
        // let sorted_list_of_moves = self.get_all_first_movements_sorted(rules, board, player);

        // // Open channel
        // let (tx, rx) = mpsc::channel();
        // for moves in sorted_list_of_moves {
        //     let rules_clone = *rules;
        //     let mut board_clone = board.clone();
        //     let self_clone = self.clone();
        //     let tx_clone = tx.clone();

        //     thread::spawn(move || {
        //         let thread_result = self_clone.initial_negamax_alpha_beta(
        //             &rules_clone,
        //             MinimaxAction {
        //                 board: &mut board_clone,
        //                 movement: None,
        //                 patterns: None,
        //             },
        //             AlphaBetaIteration {
        //                 depth,
        //                 alpha: i32::min_value() + 1,
        //                 beta: i32::max_value(),
        //             },
        //             player,
        //             moves,
        //         );
        //         let _ = tx_clone.send(thread_result);
        //     });
        // }

        // let mut best_move = Evaluation {
        //     score: i32::min_value() + 1,
        //     movement: None,
        // };

        // for _ in 0..NB_THREAD {
        //     let thread_result = rx.recv().unwrap().unwrap();
        //     // println!("Return of thread nb {} | score {}", i, thread_result.score);
        //     if thread_result.score >= best_move.score {
        //         // println!(
        //         //     "Better score found, prev {} | new {}",
        //         //     best_move.score, thread_result.score
        //         // );
        //         best_move.score = thread_result.score;
        //         best_move.movement = thread_result.movement;
        //     }
        // }

        // Ok(best_move)
        /* WITH THREADS */
    }
}
