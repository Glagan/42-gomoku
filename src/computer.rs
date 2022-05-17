use crate::{
    board::{Board, Move},
    player::Player,
};

#[derive(Debug)]
pub struct MiniMaxEvaluation {
    pub score: i64,
    pub movement: Option<Move>,
}

pub struct Computer {
    pub player: Player,
}

impl Computer {
    pub fn new(player: &Player) -> Computer {
        Computer { player: *player }
    }

    fn minimax(
        &self,
        board: &Board,
        depth: usize,
        player: &Player,
    ) -> Result<MiniMaxEvaluation, String> {
        if depth == 0 {
            return Ok(MiniMaxEvaluation {
                score: board.evaluate(player),
                movement: None,
            });
        }
        if *player == self.player {
            let mut max_eval = MiniMaxEvaluation {
                score: i64::min_value(),
                movement: None,
            };
            for movement in board.legal_moves(player).iter() {
                let new_board = board.apply_move(movement)?;
                let eval = self.minimax(
                    &new_board,
                    depth - 1,
                    if self.player == Player::Black {
                        &Player::White
                    } else {
                        &Player::Black
                    },
                )?;
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
            for movement in board.legal_moves(player).iter() {
                let new_board = board.apply_move(movement)?;
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
        alpha: &mut i64,
        beta: &mut i64,
        player: &Player,
    ) -> Result<MiniMaxEvaluation, String> {
        if depth == 0 {
            return Ok(MiniMaxEvaluation {
                score: board.evaluate(player),
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
            for movement in board.legal_moves(player).iter() {
                let new_board = board.apply_move(movement)?;
                let eval = self.minimax(&new_board, depth - 1, other_player)?;
                if eval.score > max_eval.score {
                    max_eval.score = eval.score;
                    max_eval.movement = Some(movement.clone());
                }
                if eval.score > *alpha {
                    *alpha = eval.score;
                }
                if beta <= alpha {
                    break;
                }
            }
            return Ok(max_eval);
        } else {
            let mut min_eval = MiniMaxEvaluation {
                score: i64::max_value(),
                movement: None,
            };
            for movement in board.legal_moves(other_player).iter() {
                let new_board = board.apply_move(movement)?;
                let eval = self.minimax(&new_board, depth - 1, &self.player)?;
                if eval.score < min_eval.score {
                    min_eval.score = eval.score;
                    min_eval.movement = Some(movement.clone());
                }
                if eval.score > *beta {
                    *beta = eval.score;
                }
                if beta <= alpha {
                    break;
                }
            }
            return Ok(min_eval);
        }
    }

    // Use the minimax algorithm to get the next best move
    pub fn play(&self, board: &Board, depth: usize) -> Result<MiniMaxEvaluation, String> {
        let mut alpha = i64::min_value();
        let mut beta = i64::max_value();
        let best_move =
            self.minimax_alpha_beta(board, depth, &mut alpha, &mut beta, &self.player)?;
        Ok(best_move)
    }
}
