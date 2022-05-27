use crate::board::Rock;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Player {
    Black,
    White,
}

impl Player {
    pub fn pawn(&self) -> Rock {
        if *self == Player::Black {
            Rock::Black
        } else {
            Rock::White
        }
    }
}
