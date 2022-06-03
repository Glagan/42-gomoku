use crate::{
    board::{Board, Move},
    pattern::{PatternCount, PATTERN_FINDER},
    player::Player,
    rules::RuleSet,
};
use colored::Colorize;
use std::{cmp::Ordering, collections::BinaryHeap, fmt, sync::mpsc, thread};

pub const NB_THREAD: usize = 4;

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
    movement: Option<Move>,
    patterns: Option<PatternCount>,
}

#[derive(Default, Clone)]
pub struct Computer;

impl Computer {
    // Calculate the patterns created by a movement and return it's score
    pub fn evaluate_action(&self, action: &MinimaxAction) -> i32 {
        PATTERN_FINDER.patterns_score(action.patterns.as_ref().unwrap())
    }

    fn negamax_alpha_beta(
        &mut self,
        rules: &RuleSet,
        action: MinimaxAction,
        iteration: AlphaBetaIteration,
        player: Player,
        color: i32,
    ) -> Result<Evaluation, String> {
        let mut alpha = iteration.alpha;
        let beta = iteration.beta;

        // Check if it's a leaf and compute it's value
        if iteration.depth == 0 || action.board.is_winning(rules, player) {
            if action.movement.is_none() {
                return Err("Empty movement in negamax leaf".to_string());
            }
            let winning: bool = action.board.is_winning(rules, player);
            let score = self.evaluate_action(&action);
            // if winning {
            //     println!(
            //         "winning at depth {} | score {} | color {}",
            //         iteration.depth, score, color
            //     );
            //     println!("{}", action.board);
            // }
            return Ok(Evaluation {
                score: color * score,
                movement: None,
            });
        }

        // Only the best evaluation is returned
        let mut best_eval = Evaluation {
            score: i32::min_value() + 1,
            movement: None,
        };

        // Iterate each neighbor moves
        let mut moves: BinaryHeap<SortedMove> = action
            .board
            .intersections_legal_moves(rules, player)
            .iter()
            .map(|&movement| {
                action.board.set_move(rules, &movement);
                let pattern_count = PATTERN_FINDER.count_movement_patterns(action.board, &movement);
                action.board.undo_move(rules, &movement);
                SortedMove {
                    movement,
                    best_pattern: pattern_count.best_pattern(),
                    pattern_count,
                }
            })
            .collect();
        while let Some(sorted_movement) = moves.pop() {
            action.board.set_move(rules, &sorted_movement.movement);
            let eval = self.negamax_alpha_beta(
                rules,
                MinimaxAction {
                    board: action.board,
                    movement: Some(sorted_movement.movement),
                    patterns: Some(sorted_movement.pattern_count),
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
            // let score = -eval.score;
            let score = eval.score;
            if score > best_eval.score {
                alpha = score;
                best_eval.score = score;
                best_eval.movement = Some(sorted_movement.movement);
                if alpha >= beta {
                    break;
                }
            }
        }

        Ok(best_eval)
    }

    fn launch_one_thread(
        &mut self,
        rules: &RuleSet,
        action: MinimaxAction,
        iteration: AlphaBetaIteration,
        player: Player,
        maximize: Player,
        mut moves: BinaryHeap<SortedMove>,
    ) -> Result<Evaluation, String> {
        // let alpha_orig = iteration.alpha;
        let mut alpha = iteration.alpha;
        let beta = iteration.beta;

        // Only the best evaluation is returned
        let mut best_eval = Evaluation {
            score: i32::min_value() + 1,
            movement: None,
        };
        while let Some(sorted_movement) = moves.pop() {
            action.board.set_move(rules, &sorted_movement.movement);
            let eval = self.negamax_alpha_beta(
                rules,
                MinimaxAction {
                    board: action.board,
                    movement: Some(sorted_movement.movement),
                    patterns: Some(sorted_movement.pattern_count),
                },
                AlphaBetaIteration {
                    depth: iteration.depth - 1,
                    alpha: -beta,
                    beta: -alpha,
                },
                player.opponent(),
                1,
            )?;
            action.board.undo_move(rules, &sorted_movement.movement);
            //eval.score = -eval.score;
            let score = eval.score;
            if score > best_eval.score {
                alpha = score;
                best_eval.score = score;
                best_eval.movement = Some(sorted_movement.movement);
                if alpha >= iteration.beta {
                    break;
                }
            }
        }
        Ok(best_eval)
    }

    pub fn get_all_first_movements_sorted(
        &mut self,
        rules: &RuleSet,
        action: MinimaxAction,
        player: Player,
    ) -> Vec<BinaryHeap<SortedMove>> {
        let mut moves: BinaryHeap<SortedMove> = action
            .board
            .intersections_legal_moves(rules, player)
            .iter()
            .map(|&movement| {
                action.board.set_move(rules, &movement);
                let pattern_count = PATTERN_FINDER.count_movement_patterns(action.board, &movement);
                action.board.undo_move(rules, &movement);
                SortedMove {
                    movement,
                    best_pattern: pattern_count.best_pattern(),
                    pattern_count,
                }
            })
            .collect();

        let mut sorted_list_of_moves: Vec<BinaryHeap<SortedMove>> = vec![];
        for _ in 0..NB_THREAD {
            sorted_list_of_moves.push(BinaryHeap::new());
        }
        let mut index: usize = 0;
        while let Some(sorted_movement) = moves.pop() {
            sorted_list_of_moves[index % NB_THREAD].push(sorted_movement);
            index += 1;
        }
        sorted_list_of_moves
    }

    // Use the negamax algorithm (minimax variant) to get the next best move
    pub fn play(
        &mut self,
        rules: &RuleSet,
        board: &mut Board,
        depth: usize,
        player: Player,
    ) -> Result<Evaluation, String> {
        /*WITHOUT THREAD */
        // let action = MinimaxAction {
        //     board,
        //     movement: None,
        // };

        // let moves: BinaryHeap<SortedMove> = action
        //     .board
        //     .intersections_legal_moves(rules, player)
        //     .iter()
        //     .map(|&movement| SortedMove {
        //         movement,
        //         pattern: PATTERN_FINDER.best_pattern_for_rock(action.board, movement.index),
        //     })
        //     .collect();

        // let best_move = self.launch_one_thread(
        //     rules,
        //     MinimaxAction {
        //         board,
        //         movement: None,
        //     },
        //     AlphaBetaIteration {
        //         depth,
        //         alpha: i32::min_value() + 1,
        //         beta: i32::max_value(),
        //     },
        //     player,
        //     player,
        //     moves,
        // );
        // best_move
        /* WITHOUT THREAD */

        //Get all possible moves to launch them in multiple threads
        let sorted_list_of_moves = self.get_all_first_movements_sorted(
            rules,
            MinimaxAction {
                board,
                movement: None,
                patterns: None,
            },
            player,
        );

        //Open channel
        let (tx, rx) = mpsc::channel();

        for moves in sorted_list_of_moves {
            let rules_clone = rules.clone();
            let mut board_clone = board.clone();
            let mut self_clone = self.clone();
            let tx_clone = tx.clone();

            thread::spawn(move || {
                let thread_result = self_clone.launch_one_thread(
                    &rules_clone,
                    MinimaxAction {
                        board: &mut board_clone,
                        movement: None,
                        patterns: None,
                    },
                    AlphaBetaIteration {
                        depth,
                        alpha: i32::min_value() + 1,
                        beta: i32::max_value(),
                    },
                    player.opponent(),
                    player,
                    moves,
                );
                let _ = tx_clone.send(thread_result);
            });
        }

        let mut best_move = Evaluation {
            score: 0,
            movement: None,
        };

        for i in 0..NB_THREAD {
            let thread_result = rx.recv().unwrap().unwrap();
            //println!("Return of thread nb {} | score {}", i, thread_result.score);
            if thread_result.score >= best_move.score {
                // println!(
                //     "Better score found, prev {} | new {}",
                //     best_move.score, thread_result.score
                // );
                best_move.score = thread_result.score;
                best_move.movement = thread_result.movement;
            }
        }

        Ok(best_move)
    }
}
