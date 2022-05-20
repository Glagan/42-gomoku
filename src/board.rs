use crate::{player::Player, rules::RuleSet};
use fixed_vec_deque::FixedVecDeque;
use std::fmt::Display;

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
    pub black_rocks: Vec<usize>,
    pub white_rocks: Vec<usize>,
    pub black_capture: u8,
    pub white_capture: u8,
}

impl Default for Board {
    fn default() -> Board {
        Board {
            pieces: [Pawn::None; BOARD_PIECES],
            moves: vec![],
            black_rocks: vec![],
            white_rocks: vec![],
            black_capture: 0,
            white_capture: 0,
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
        // Black rocks
        for existing_pawn in self.black_rocks.iter() {
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
        // White rocks
        // TODO de-duplicate logic
        for existing_pawn in self.black_rocks.iter() {
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
    // TODO
    pub fn is_move_legal(&self, rules: &RuleSet, movement: &Move) -> bool {
        // Forbid movements that would create a "double three"
        if rules.no_double_three {
            // TODO
        }
        // Forbid movements that would put a pawn in a "capture" state
        if rules.capture {
            // TODO
        }
        true
    }

    // All possible movements from the intersections for a given player
    pub fn intersections_legal_moves(&self, rules: &RuleSet, player: &Player) -> Vec<Move> {
        // Analyze each intersections and check if a Pawn can be set on it
        // -- for the current player according to the rules
        let intersections = self.open_intersections();
        // println!("---\n{}\n--- intersections {:#?}", self, intersections);
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
        self.moves.push(movement.clone());
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
            for (orig_x, orig_y) in DIRECTIONS {
                // Create a window of length 5 and update it on each move
                // If there is five in a row in the window, return true
                let mut buf = FixedVecDeque::<[usize; 5]>::new();
                let mut mov_x = orig_x * -4;
                let mut mov_y = orig_y * -4;
                for _ in 0..8 {
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
                        if buf == [1, 1, 1, 1, 1] {
                            return true;
                        }
                    }
                    mov_x += orig_x;
                    mov_y += orig_y;
                }
            }
        }
        false
    }

    // Check if the given player is winning on the current board
    // (Has an unbreakable winning position according to the rules)
    pub fn is_winning(&self, rules: &RuleSet, player: &Player) -> bool {
        if rules.game_ending_capture
            && ((player == &Player::Black && self.black_capture >= 10)
                || (player == &Player::White && self.white_capture >= 10))
        {
            return true;
        }
        // TODO if game.capture -> check if five in a row can't be captured
        self.has_five_in_a_row(player)
    }
}
