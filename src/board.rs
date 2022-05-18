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
        if self.moves.len() == 0 {
            return vec![180]; // Only the center intersection is available if there is no previous moves
        }
        let mut intersections: Vec<usize> = vec![];
        for (existing_pawn, _) in self
            .pieces
            .iter()
            .enumerate()
            .filter(|&pawn| pawn.1 != &Pawn::None)
        {
            let (x, y) = Board::index_to_coordinates(existing_pawn);
            for x_mov in [4, 3, 0, 1, 2] {
                for y_mov in [4, 3, 0, 1, 2] {
                    if x_mov == 0 && y_mov == 0 {
                        continue;
                    }
                    if (x_mov == 4 && x <= 1)
                        || (x_mov == 3 && x <= 0)
                        || (x_mov > 0 && x + x_mov >= BOARD_SIZE)
                    {
                        continue;
                    }
                    if (y_mov == 4 && y <= 1)
                        || (y_mov == 3 && y <= 0)
                        || (y_mov > 0 && y + y_mov >= BOARD_SIZE)
                    {
                        continue;
                    }
                    let (new_x, new_y) = (
                        if x_mov == 3 {
                            x - 1
                        } else if x_mov == 4 {
                            x - 2
                        } else {
                            x + x_mov
                        },
                        if y_mov == 3 {
                            y - 1
                        } else if y_mov == 4 {
                            y - 2
                        } else {
                            y + y_mov
                        },
                    );
                    if let Some(pawn) = &self.get(new_x, new_y) {
                        if *pawn == Pawn::None {
                            // println!(
                            //     "x {} y {} -> new_x {} new_y {} ({} {})",
                            //     x, y, new_x, new_y, x_mov, y_mov
                            // );
                            let index = Board::coordinates_to_index(new_x, new_y);
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
