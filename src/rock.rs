#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Rock {
    None,
    Black,
    White,
}

impl ToString for Rock {
    fn to_string(&self) -> String {
        match self {
            Rock::None => "0".to_string(),
            Rock::Black => "1".to_string(),
            Rock::White => "2".to_string(),
        }
    }
}

impl Rock {
    pub fn opponent(&self) -> Rock {
        if self == &Rock::Black {
            Rock::White
        } else {
            Rock::Black
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum PlayerRock {
    None,
    Player,
    Opponent,
}

impl ToString for PlayerRock {
    fn to_string(&self) -> String {
        match self {
            PlayerRock::None => "0".to_string(),
            PlayerRock::Player => "P".to_string(),
            PlayerRock::Opponent => "E".to_string(),
        }
    }
}

impl PlayerRock {
    pub fn opponent(&self) -> PlayerRock {
        if self == &PlayerRock::Player {
            PlayerRock::Opponent
        } else {
            PlayerRock::Player
        }
    }
}
