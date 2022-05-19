use crate::{board::Board, player::Player, rules::RuleSet};
use std::time::{Duration, Instant};

#[derive(PartialEq, Copy, Clone)]
pub enum GameMode {
    None,
    PvP,
    PvA,
    AvA,
}

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
    pub play_time: Instant,
    pub previous_play_time: Duration,
    pub current_player: Player,
    pub black_capture: usize,
    pub white_capture: usize,
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
            play_time: now,
            previous_play_time: now - now,
            current_player: Player::Black,
            black_capture: 0,
            white_capture: 0,
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
        let now = Instant::now();
        self.play_time = now;
        self.previous_play_time = now - now;
        self.current_player = Player::Black;
        self.black_capture = 0;
        self.white_capture = 0;
        self.winner = Winner::None;
    }

    pub fn start(&mut self, mode: GameMode) {
        self.reset();
        self.mode = mode;
        self.playing = true;
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
