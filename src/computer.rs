use crate::{
    board::{Board, Move},
    pattern::{Pattern, PATTERN_FINDER},
    player::Player,
    rules::RuleSet,
};
use colored::Colorize;
use std::{cmp::Ordering, collections::BinaryHeap, fmt};

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
}

#[derive(Default)]
pub struct Computer;

impl Computer {
    // Calculate the patterns created by a movement and return it's score
    pub fn evaluate_action(&self, action: &MinimaxAction) -> i32 {
        PATTERN_FINDER.movement_score(action.board, &action.movement.unwrap())
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
        let win_move = if let Some(movement) = &action.movement {
            action.board.move_make_win(rules, movement)
        } else {
            false
        };
        if iteration.depth == 0 || win_move {
            if action.movement.is_none() {
                return Err("Empty movement in negamax leaf".to_string());
            }
            let score = self.evaluate_action(&action);
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
            .map(|&movement| SortedMove {
                movement,
                pattern: PATTERN_FINDER.best_pattern_for_rock(action.board, &movement.coordinates),
            })
            .collect();
        while let Some(sorted_movement) = moves.pop() {
            action.board.set_move(rules, &sorted_movement.movement);
            let eval = self.negamax_alpha_beta(
                rules,
                MinimaxAction {
                    board: action.board,
                    movement: Some(sorted_movement.movement),
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

    // Use the negamax algorithm (minimax variant) to get the next best move
    pub fn play(
        &mut self,
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
