#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum Player {
    Black,
    White,
}

impl Player {
    #[inline(always)]
    pub fn opponent(&self) -> Player {
        if self == &Player::Black {
            Player::White
        } else {
            Player::Black
        }
    }
}
