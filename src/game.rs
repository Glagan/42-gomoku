use crate::{
    board::{Board, Move, Pawn},
    computer::Computer,
    player::Player,
    rules::RuleSet,
};
use colored::Colorize;
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
    pub rock_move: Vec<usize>,
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
            rock_move: vec![],
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
        self.rock_move = vec![];
        self.computer.clean();
    }

    pub fn start(&mut self, mode: GameMode) {
        self.reset();
        self.mode = mode;
        self.playing = true;
    }

    pub fn add_rock_move(&mut self, index: usize) {
        self.rock_move.push(index)
    }

    pub fn player_won(&mut self) {
        self.winner = match self.current_player {
            Player::Black => Winner::Black,
            Player::White => Winner::White,
        };
        self.computer.clean();
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

    pub fn play_player(&mut self, x: usize, y: usize) {
        if self.board.pieces[Board::coordinates_to_index(x, y)] == Pawn::None {
            self.board.set_move(
                &self.rules,
                &Move {
                    index: Board::coordinates_to_index(x, y),
                    player: self.current_player,
                },
            );
            self.add_rock_move(Board::coordinates_to_index(x, y));
            if self.board.is_winning(&self.rules, &self.current_player) {
                self.player_won();
            } else {
                self.next_player();
            }
        }
    }

    pub fn play_computer(&mut self) {
        let play_result = self
            .computer
            .play(&self.rules, &self.board, 3, &self.current_player);
        if let Ok(play) = play_result {
            println!(
                "computer play: {} in {}ms",
                play,
                self.play_time.elapsed().as_millis()
            );
            if let Some(movement) = play.movement {
                self.board.set_move(&self.rules, &movement);
                self.add_rock_move(movement.index);
                if self.board.is_winning(&self.rules, &self.current_player) {
                    self.player_won();
                } else {
                    self.next_player();
                }
            }
        } else {
            println!("{}", "computer returned an empty play result".red());
        }
    }
}
