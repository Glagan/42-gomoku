use crate::rock::Rock;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Player {
    Black,
    White,
}

impl Player {
    pub fn rock(&self) -> Rock {
        if *self == Player::Black {
            Rock::Black
        } else {
            Rock::White
        }
    }
}
