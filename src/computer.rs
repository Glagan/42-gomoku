use crate::{
    board::{Board, Move, BOARD_PIECES},
    pattern::{Pattern, PATTERN_FINDER},
    player::Player,
    rock::Rock,
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
    pub black_cache: HashMap<[Rock; BOARD_PIECES as usize], CacheEntry>,
    pub white_cache: HashMap<[Rock; BOARD_PIECES as usize], CacheEntry>,
}

impl Computer {
    pub fn clean(&mut self) {
        // self.black_cache = HashMap::new();
        // self.white_cache = HashMap::new();
    }

    // Calculate the patterns created by a movement and return it's score
    pub fn evaluate_action(&self, action: &MinimaxAction) -> i64 {
        PATTERN_FINDER.movement_score(action.board, &action.movement.unwrap())
    }

    pub fn cache(
        &mut self,
        player: Player,
    ) -> &mut HashMap<[Rock; BOARD_PIECES as usize], CacheEntry> {
        if player == Player::Black {
            &mut self.black_cache
        } else {
            &mut self.white_cache
        }
    }

    fn negamax_alpha_beta(
        &mut self,
        rules: &RuleSet,
        action: MinimaxAction,
        iteration: AlphaBetaIteration,
        player: Player,
        maximize: Player,
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
                if player == Player::Black {
                    Player::White
                } else {
                    Player::Black
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
        player: Player,
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

        Ok(best_move)
    }
}
