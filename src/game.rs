use crate::{board::Board, computer::Computer, player::Player, rules::RuleSet};
use std::time::{Duration, Instant};

#[derive(PartialEq, Copy, Clone)]
pub enum GameMode {
    None,
    PvP,
    PvA,
    AvA,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Winner {
    None,
    Black,
    White,
    Tie,
}

pub struct Game {
    pub playing: bool,
    pub board: Board,
    pub mode: GameMode,
    pub rules: RuleSet,
    pub computer: Computer,
    pub play_time: Instant,
    pub previous_play_time: Duration,
    pub current_player: Player,
    pub winner: Winner,
}

impl Default for Game {
    fn default() -> Self {
        let now = Instant::now();
        Game {
            playing: false,
            board: Board::default(),
            mode: GameMode::None,
            rules: RuleSet::default(),
            computer: Computer::default(),
            play_time: now,
            previous_play_time: now - now,
            current_player: Player::Black,
            winner: Winner::None,
        }
    }
}

impl Game {
    pub fn reset(&mut self) {
        self.playing = false;
        self.board = Board::default();
        self.mode = GameMode::None;
        self.rules = RuleSet::default();
        self.computer = Computer::default();
        let now = Instant::now();
        self.play_time = now;
        self.previous_play_time = now - now;
        self.current_player = Player::Black;
        self.winner = Winner::None;
        self.computer.clean();
    }

    pub fn start(&mut self, mode: GameMode) {
        self.reset();
        self.mode = mode;
        self.playing = true;
    }

    pub fn player_won(&mut self) {
        self.winner = match self.current_player {
            Player::Black => Winner::Black,
            Player::White => Winner::White,
        };
    }

    pub fn next_player(&mut self) {
        if self.current_player == Player::Black {
            self.current_player = Player::White;
        } else {
            self.current_player = Player::Black;
        }
        self.previous_play_time = self.play_time.elapsed();
        self.play_time = Instant::now();
    }
}
