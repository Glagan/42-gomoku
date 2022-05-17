use std::fmt::Display;

#[derive(PartialEq, Clone, Copy)]
pub enum Player {
    Black,
    White,
}

impl Player {
    pub fn pawn(&self) -> Pawn {
        if *self == Player::Black {
            Pawn::Black
        } else {
            Pawn::White
        }
    }
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub struct Move {
    pub player: Player,
    pub index: usize, // Index of the piece to place
}

const BOARD_SIZE: usize = 19;
const BOARD_PIECES: usize = BOARD_SIZE * BOARD_SIZE;

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
        Some(self.pieces[(x as f64 / BOARD_SIZE as f64) as usize + (y * BOARD_SIZE)].clone())
    }

    // All possible movements for the given player
    pub fn legal_moves(&self, player: &Player) -> Vec<Move> {
        todo!()
    }

    // Apply a movement to the current Board
    pub fn set_move(&mut self, movement: &Move) -> Result<(), String> {
        if movement.index >= BOARD_PIECES {
            return Err("Invalid index for movement".to_string());
        }
        // TODO capture
        self.pieces[movement.index] = movement.player.pawn();
        self.moves.push(movement.clone());
        Ok(())
    }

    // Apply a movement to a new copy of the current Board
    pub fn apply_move(&self, movement: &Move) -> Result<Board, String> {
        todo!()
    }

    // Calculate all patterns for a given player and return the board score
    pub fn evaluate(&self, player: &Player) -> u64 {
        todo!()
    }
}
