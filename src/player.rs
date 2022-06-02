use crate::rock::Rock;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Player {
    Black,
    White,
}

impl Player {
    #[inline(always)]
    pub fn rock(&self) -> Rock {
        if self == &Player::Black {
            Rock::Black
        } else {
            Rock::White
        }
    }

    #[inline(always)]
    pub fn opponent(&self) -> Player {
        if self == &Player::Black {
            Player::White
        } else {
            Player::Black
        }
    }
}
