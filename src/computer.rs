use crate::{
    board::{Board, Move},
    pattern::{Pattern, PATTERN_FINDER},
    player::Player,
    rules::RuleSet,
};
use colored::Colorize;
use std::{cmp::Ordering, collections::BinaryHeap, fmt, sync::mpsc, thread};

pub const NB_THREAD: usize = 4;

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

/*#[derive(PartialEq)]
pub enum CacheFlag {
    Exact = 0,
    Upperbound = 1,
    Lowerbound = 2,
}

pub struct CacheEntry {
    pub score: i32,
    pub moves: u16,
    pub flag: CacheFlag,
    pub movement: Option<Move>,
}*/

pub struct AlphaBetaIteration {
    depth: usize,
    alpha: i32,
    beta: i32,
}

pub struct MinimaxAction<'a> {
    board: &'a mut Board,
    movement: Option<Move>,
}

#[derive(Default, Clone)]
pub struct Computer {
    // pub black_cache: HashMap<[Rock; BOARD_PIECES as usize], CacheEntry>,
// pub white_cache: HashMap<[Rock; BOARD_PIECES as usize], CacheEntry>,
}

impl Computer {
    pub fn clean(&mut self) {
        // self.black_cache = HashMap::new();
        // self.white_cache = HashMap::new();
    }

    // Calculate the patterns created by a movement and return it's score
    pub fn evaluate_action(&self, action: &MinimaxAction) -> i32 {
        PATTERN_FINDER.movement_score(action.board, &action.movement.unwrap())
    }

    /*pub fn cache(
        &mut self,
        player: Player,
    ) -> &mut HashMap<[Rock; BOARD_PIECES as usize], CacheEntry> {
        if player == Player::Black {
            &mut self.black_cache
        } else {
            &mut self.white_cache
        }
    }*/

    fn negamax_alpha_beta(
        &mut self,
        rules: &RuleSet,
        action: MinimaxAction,
        iteration: AlphaBetaIteration,
        player: Player,
        maximize: Player,
    ) -> Result<Evaluation, String> {
        // let alpha_orig = iteration.alpha;
        let mut alpha = iteration.alpha;
        let beta = iteration.beta;

        // Check cache to see if the board was already computed
        /*if self.cache(player).contains_key(&action.board.pieces) {
            let cache_entry = self.cache(player).get(&action.board.pieces).unwrap();
            if cache_entry.moves >= action.board.moves {
                if cache_entry.flag == CacheFlag::Exact {
                    let color = if player == maximize { 1 } else { -1 };
                    return Ok(Evaluation {
                        score: color * cache_entry.score,
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
                    let color = if player == maximize { 1 } else { -1 };
                    return Ok(Evaluation {
                        score: color * cache_entry.score,
                        movement: cache_entry.movement,
                    });
                }
            }
        }*/

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
                pattern: PATTERN_FINDER.best_pattern_for_rock(action.board, movement.index),
            })
            .collect();
        while let Some(sorted_movement) = moves.pop() {
            action.board.set_move(rules, &sorted_movement.movement);
            let mut eval = self.negamax_alpha_beta(
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
                if player == Player::Black {
                    Player::White
                } else {
                    Player::Black
                },
                maximize,
            )?;
            action.board.undo_move(rules, &sorted_movement.movement);
            //eval.score = -eval.score;
            if eval.score > best_eval.score {
                alpha = alpha.max(eval.score);
                best_eval.score = eval.score;
                best_eval.movement = Some(sorted_movement.movement);
                if alpha >= beta {
                    break;
                }
            }
        }

        // Add to cache
        /*let cache_entry = self
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
        }*/

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
            let mut eval = self.negamax_alpha_beta(
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
                if player == Player::Black {
                    Player::White
                } else {
                    Player::Black
                },
                maximize,
            )?;
            action.board.undo_move(rules, &sorted_movement.movement);
            //eval.score = -eval.score;
            if eval.score > best_eval.score {
                alpha = alpha.max(eval.score);
                best_eval.score = eval.score;
                best_eval.movement = Some(sorted_movement.movement);
                if alpha >= beta {
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
            .map(|&movement| SortedMove {
                movement,
                pattern: PATTERN_FINDER.best_pattern_for_rock(action.board, movement.index),
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
        // Clean cache
        // self.black_cache.retain(|_, v| v.moves >= board.moves);
        // self.white_cache.retain(|_, v| v.moves >= board.moves);

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
                    },
                    AlphaBetaIteration {
                        depth,
                        alpha: i32::min_value() + 1,
                        beta: i32::max_value(),
                    },
                    player,
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
