use crate::{rules::RuleSet, Player};
use std::fmt::Display;

#[derive(PartialEq, Clone, Copy)]
pub enum Pawn {
    None,
    Black,
    White,
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

#[derive(Clone)]
pub struct Board {
    pub pieces: [Pawn; BOARD_PIECES],
    pub moves: Vec<Move>,
}

impl Default for Board {
    fn default() -> Board {
        Board {
            pieces: [Pawn::None; BOARD_PIECES],
            moves: vec![],
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

    pub fn index_to_coordinates(index: usize) -> usize {
        (index as f64 / BOARD_SIZE as f64) as usize + (index * BOARD_SIZE)
    }

    pub fn coordinates_to_index(x: usize, y: usize) -> usize {
        (y * BOARD_SIZE) + x
    }

    // All open intersections for the current Board
    // -- Empty cases within other pieces
    pub fn open_intersections(&self) -> Vec<usize> {
        if self.moves.len() == 0 {
            return vec![171]; // Only the center intersection is available if there is no previous moves
        }
        let mut intersections: Vec<usize> = vec![];
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                // If there is a piece on a case, check all 8 case around it
                if let Some(_) = self.get(x, y) {
                    for x_mov in [2, 0, 1] {
                        for y_mov in [2, 0, 1] {
                            if x_mov == 0 && y_mov == 0 {
                                continue;
                            }
                            if (x_mov == 2 && x == 0) || (x_mov > 0 && x == BOARD_SIZE - 1) {
                                continue;
                            }
                            if (y_mov == 2 && y == 0) || (y_mov > 0 && y == BOARD_SIZE - 1) {
                                continue;
                            }
                            let (new_x, new_y) = (
                                if x_mov == 2 { x - 1 } else { x + x_mov },
                                if y_mov == 2 { y - 1 } else { y + y_mov },
                            );
                            if let Some(pawn) = &self.get(new_x, new_y) {
                                if *pawn == Pawn::None {
                                    let index = Board::coordinates_to_index(new_x, new_y);
                                    if !intersections.contains(&index) {
                                        intersections.push(index);
                                    }
                                }
                            }
                        }
                    }
                    // Check top row
                    if x > 0 {
                        if y > 0 {
                            if let Some(_) = self.get(x - 1, y - 1) {}
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
    pub fn set_move(&mut self, rules: &RuleSet, movement: &Move) -> Result<(), String> {
        if movement.index >= BOARD_PIECES {
            return Err("Invalid index for movement".to_string());
        }
        if rules.capture {
            // TODO capture remove other pawns
            // TODO Return the number of captured pawns to increase the total (if rules.game_ending_capture)
        }
        self.pieces[movement.index] = movement.player.pawn();
        self.moves.push(movement.clone());
        Ok(())
    }

    // Apply a movement to a new copy of the current Board
    pub fn apply_move(&self, rules: &RuleSet, movement: &Move) -> Result<Board, String> {
        let mut new_board = self.clone();
        new_board.set_move(rules, movement)?;
        Ok(new_board)
    }

    // Check if the given player is winning on the current board
    // (Has an unbreakable winning position according to the rules)
    // TODO
    pub fn is_winning(&self, rules: &RuleSet, player: &Player) -> bool {
        false
    }
}
