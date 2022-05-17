use crate::board::Pawn;

#[derive(Debug, PartialEq, Clone, Copy)]
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
