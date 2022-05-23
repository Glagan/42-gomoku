use crate::{player::Player, rules::RuleSet};
use colored::Colorize;
use fixed_vec_deque::FixedVecDeque;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Pawn {
    None = 0,
    Black = 1,
    White = 2,
}

impl ToString for Pawn {
    fn to_string(&self) -> String {
        match self {
            Pawn::None => "0".to_string(),
            Pawn::Black => "1".to_string(),
            Pawn::White => "2".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub player: Player,
    pub index: usize, // Index of the piece to place
}

#[derive(Debug, Clone, Copy)]
pub struct PossibleMove {
    pub index: usize,
    pub legal: bool,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (x, y) = Board::index_to_coordinates(self.index);
        if self.player == Player::Black {
            write!(
                f,
                "{} {} ({}x{})",
                "black".white().on_black(),
                self.index,
                x,
                y
            )
        } else {
            write!(
                f,
                "{} {} ({}x{})",
                "white".black().on_white(),
                self.index,
                x,
                y
            )
        }
    }
}

pub const BOARD_SIZE: usize = 19;
pub const BOARD_PIECES: usize = BOARD_SIZE * BOARD_SIZE;
pub const DIRECTIONS: [(i16, i16); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Clone)]
pub struct Board {
    pub pieces: [Pawn; BOARD_PIECES],
    pub moves: Vec<Move>,
    pub rocks: u16,
    pub black_rocks: Vec<usize>,
    pub white_rocks: Vec<usize>,
    pub all_rocks: Vec<usize>,
    pub black_capture: u8,
    pub white_capture: u8,
}

impl Default for Board {
    fn default() -> Board {
        Board {
            pieces: [Pawn::None; BOARD_PIECES],
            moves: vec![],
            rocks: 0,
            black_rocks: vec![],
            white_rocks: vec![],
            all_rocks: vec![],
            black_capture: 0,
            white_capture: 0,
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..BOARD_SIZE {
            write!(
                f,
                "{}",
                self.pieces[(BOARD_SIZE * row)..(BOARD_SIZE * (row + 1))]
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(&" ")
            )?;
            if row != BOARD_SIZE - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Board {
    pub fn display(&self, level: usize) {
        for row in 0..BOARD_SIZE {
            print!(
                "{:indent$}{}\n",
                "",
                self.pieces[(BOARD_SIZE * row)..(BOARD_SIZE * (row + 1))]
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(&" "),
                indent = level
            );
        }
    }

    // Helper function to get a Board case with (x, y) coordinates
    pub fn get(&self, x: usize, y: usize) -> Option<Pawn> {
        if x >= BOARD_SIZE || y >= BOARD_SIZE {
            return None;
        }
        Some(self.pieces[Board::coordinates_to_index(x, y)].clone())
    }

    pub fn index_to_coordinates(index: usize) -> (usize, usize) {
        (
            index % BOARD_SIZE,
            (index as f64 / BOARD_SIZE as f64) as usize,
        )
    }

    pub fn coordinates_to_index(x: usize, y: usize) -> usize {
        x + (y * BOARD_SIZE)
    }

    // All open intersections for the current Board
    // -- Empty cases within other pieces
    pub fn open_intersections(&self) -> Vec<usize> {
        // Only the center intersection is available if there is no previous moves
        if self.moves.len() == 0 {
            return vec![((BOARD_SIZE as f64 / 2.) * BOARD_SIZE as f64) as usize];
        }
        let mut intersections: Vec<usize> = vec![];
        for existing_pawn in self.all_rocks.iter() {
            let (x, y) = Board::index_to_coordinates(*existing_pawn);
            let (x, y): (i16, i16) = (x.try_into().unwrap(), y.try_into().unwrap());
            for (mov_x, mov_y) in DIRECTIONS {
                let (new_x, new_y) = (x + mov_x, y + mov_y);
                // Check Board boundaries
                if new_x >= 0
                    && new_y >= 0
                    && (new_x as usize) < BOARD_SIZE
                    && (new_y as usize) < BOARD_SIZE
                {
                    if let Some(pawn) = &self.get(new_x as usize, new_y as usize) {
                        if *pawn == Pawn::None {
                            let index = Board::coordinates_to_index(new_x as usize, new_y as usize);
                            if !intersections.contains(&index) {
                                intersections.push(index);
                            }
                        }
                    }
                }
            }
        }
        intersections
    }

    // Check if a move *can* be executed according to the rules
    pub fn is_move_legal(&self, rules: &RuleSet, movement: &Move) -> bool {
        // Forbid movements that would create a "double three"
        // Pattern: [1 1 1 0 >< 0 1 1 1] where [><] means any direction change
        if rules.no_double_three {
            // TODO
        }
        // Forbid movements that would put a pawn in a "recursive capture" state
        // Pattern: [2 1 0 2] or [2 0 1 2] where [0] is the movement index
        if rules.capture {
            let (x, y) = Board::index_to_coordinates(movement.index);
            let self_pawn = Some(if movement.player == Player::Black {
                Pawn::Black
            } else {
                Pawn::White
            });
            let other_pawn = Some(if movement.player == Player::Black {
                Pawn::White
            } else {
                Pawn::Black
            });
            // Left
            if x > 1
                && x < BOARD_SIZE - 1
                && self.get(x - 1, y) == self_pawn
                && self.get(x - 2, y) == other_pawn
                && self.get(x + 1, y) == other_pawn
            {
                return false;
            }
            // Right
            if x > 0
                && x < BOARD_SIZE - 2
                && self.get(x - 1, y) == other_pawn
                && self.get(x + 1, y) == self_pawn
                && self.get(x + 2, y) == other_pawn
            {
                return false;
            }
            // Top
            if y > 1
                && y < BOARD_SIZE - 1
                && self.get(x, y - 1) == self_pawn
                && self.get(x, y - 2) == other_pawn
                && self.get(x, y + 1) == other_pawn
            {
                return false;
            }
            // Bottom
            if y > 0
                && y < BOARD_SIZE - 2
                && self.get(x, y - 1) == other_pawn
                && self.get(x, y + 1) == self_pawn
                && self.get(x, y + 2) == other_pawn
            {
                return false;
            }
            // Top-Left
            if x > 1
                && y > 1
                && x < BOARD_SIZE - 1
                && y < BOARD_SIZE - 1
                && self.get(x - 1, y - 1) == self_pawn
                && self.get(x - 2, y - 2) == other_pawn
                && self.get(x + 1, y + 1) == other_pawn
            {
                return false;
            }
            // Top-Right
            if x > 0
                && y > 0
                && x < BOARD_SIZE - 2
                && y < BOARD_SIZE - 1
                && self.get(x + 1, y - 1) == self_pawn
                && self.get(x + 2, y - 2) == other_pawn
                && self.get(x - 1, y - 1) == other_pawn
            {
                return false;
            }
            // Bottom-Left
            if x > 1
                && y > 1
                && x < BOARD_SIZE - 1
                && y < BOARD_SIZE - 1
                && self.get(x - 1, y + 1) == self_pawn
                && self.get(x - 2, y + 2) == other_pawn
                && self.get(x + 1, y - 1) == other_pawn
            {
                return false;
            }
            // Bottom-Right
            if x > 0
                && y > 0
                && x < BOARD_SIZE - 2
                && y < BOARD_SIZE - 1
                && self.get(x + 1, y + 1) == self_pawn
                && self.get(x + 2, y + 2) == other_pawn
                && self.get(x - 1, y - 1) == other_pawn
            {
                return false;
            }
        }
        true
    }

    // All *legal* possible movements from the intersections for a given player
    pub fn intersections_legal_moves(&self, rules: &RuleSet, player: &Player) -> Vec<Move> {
        // Analyze each intersections and check if a Pawn can be set on it
        // -- for the current player according to the rules
        let intersections = self.open_intersections();
        let mut moves: Vec<Move> = vec![];
        for index in intersections.iter() {
            let movement = Move {
                player: *player,
                index: *index,
            };
            if self.is_move_legal(rules, &movement) {
                moves.push(movement);
            }
        }
        moves
    }

    // All possible movements from the intersections for a given player
    pub fn intersections_all_moves(&self, rules: &RuleSet, player: &Player) -> Vec<PossibleMove> {
        // Analyze each intersections and check if a Pawn can be set on it
        // -- for the current player according to the rules
        let intersections = self.open_intersections();
        let mut moves: Vec<PossibleMove> = vec![];
        for index in intersections.iter() {
            let movement = Move {
                player: *player,
                index: *index,
            };
            moves.push(PossibleMove {
                index: *index,
                legal: self.is_move_legal(rules, &movement),
            });
        }
        moves
    }

    // Apply a movement to the current Board
    pub fn set_move(&mut self, rules: &RuleSet, movement: &Move) {
        if rules.capture {
            // TODO capture remove other pawns
            // TODO Return the number of captured pawns to increase the total (if rules.game_ending_capture)
        }
        self.pieces[movement.index] = movement.player.pawn();
        if movement.player == Player::Black {
            self.black_rocks.push(movement.index);
        } else {
            self.white_rocks.push(movement.index);
        }
        self.all_rocks.push(movement.index);
        self.rocks += 1;
        self.moves.push(movement.clone());
    }

    pub fn undo_move(&mut self, rules: &RuleSet) {
        // TODO Handle capture
        self.rocks -= 1;
        if let Some(last_move) = self.moves.pop() {
            if last_move.player == Player::Black {
                self.black_rocks.pop();
            } else {
                self.white_rocks.pop();
            }
            self.pieces[last_move.index] = Pawn::None;
            self.all_rocks.pop();
        }
    }

    // Apply a movement to a new copy of the current Board
    pub fn apply_move(&self, rules: &RuleSet, movement: &Move) -> Board {
        let mut new_board = self.clone();
        new_board.set_move(rules, movement);
        new_board
    }

    pub fn has_five_in_a_row(&self, player: &Player) -> bool {
        let rocks = if player == &Player::Black {
            &self.black_rocks
        } else {
            &self.white_rocks
        };
        let player_pawn = player.pawn();
        for rock in rocks.iter() {
            let pos = Board::index_to_coordinates(*rock);
            let (x, y): (i16, i16) = (pos.0.try_into().unwrap(), pos.1.try_into().unwrap());
            // Check all 8 directions from the rock to see if there is five in a row
            for (dir_x, dir_y) in DIRECTIONS {
                // Create a window of length 5 and update it on each move
                // If there is five in a row in the window, return true
                let mut length = 0;
                // from [? ? ? ? ?] ? ? ? ? I ? ? ? ?
                // to    ? ? ? ? ?  ? ? ? ? [I ? ? ? ?]
                let mut buf = FixedVecDeque::<[usize; 5]>::new();
                let mut mov_x = dir_x * -5;
                let mut mov_y = dir_y * -5;
                for _ in 0..10 {
                    let (new_x, new_y) = (x + mov_x, y + mov_y);
                    // Check Board boundaries
                    if new_x >= 0
                        && new_y >= 0
                        && (new_x as usize) < BOARD_SIZE
                        && (new_y as usize) < BOARD_SIZE
                    {
                        // 1 for player pawn and 0 for anything else
                        *buf.push_front() =
                            if self.get(new_x as usize, new_y as usize).unwrap() == player_pawn {
                                1
                            } else {
                                0
                            };
                        length += 1;
                        if length >= 5 && buf == [1, 1, 1, 1, 1] {
                            return true;
                        }
                    }
                    mov_x += dir_x;
                    mov_y += dir_y;
                }
            }
        }
        false
    }

    // Check if the given player is winning on the current board
    // (Has an unbreakable winning position according to the rules)
    pub fn is_winning(&self, rules: &RuleSet, player: &Player) -> bool {
        if rules.capture {
            if (player == &Player::Black && self.black_capture >= 10)
                || (player == &Player::White && self.white_capture >= 10)
            {
                return true;
            }
            if rules.game_ending_capture {
                // TODO if game.capture -> check if five in a row can't be captured
                return self.has_five_in_a_row(player);
            }
        }
        self.has_five_in_a_row(player)
    }
}
