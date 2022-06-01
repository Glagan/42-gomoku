use crate::{
    board::{Board, Move, Rock},
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
    pub recommended_move: Option<Move>,
    pub play_time: Instant,
    pub previous_play_time: Duration,
    pub computer_average_play_time: f64,
    pub computer_highest_play_time: Duration,
    pub computer_lowest_play_time: Duration,
    pub current_player: Player,
    pub winner: Winner,
    pub rock_move: Vec<usize>,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            playing: false,
            board: Board::default(),
            mode: GameMode::None,
            rules: RuleSet::default(),
            computer: Computer::default(),
            recommended_move: None,
            play_time: Instant::now(),
            previous_play_time: Duration::from_millis(0),
            computer_average_play_time: 0.,
            computer_highest_play_time: Duration::from_millis(0),
            computer_lowest_play_time: Duration::MAX,
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
        self.recommended_move = None;
        self.play_time = Instant::now();
        self.previous_play_time = Duration::from_millis(0);
        self.computer_average_play_time = 0.;
        self.computer_highest_play_time = Duration::from_millis(0);
        self.computer_lowest_play_time = Duration::MAX;
        self.current_player = Player::Black;
        self.winner = Winner::None;
        self.rock_move = vec![];
        self.computer.clean();
    }

    pub fn start(&mut self, mode: GameMode) {
        self.reset();
        if self.rules.game_ending_capture && !self.rules.capture {
            self.rules.game_ending_capture = false;
        }
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
        if self.board.get(x, y) == Rock::None {
            let movement = Move {
                index: Board::coordinates_to_index(x, y),
                player: self.current_player,
            };
            if self.board.is_move_legal(&self.rules, &movement) {
                self.board.set_move(&self.rules, &movement);
                // self.board.display_all_bitboards();
                self.recommended_move = None;
                self.add_rock_move(Board::coordinates_to_index(x, y));
                if self.board.move_make_win(&self.rules, &movement) {
                    self.player_won();
                } else {
                    self.next_player();
                }
            }
        }
    }

    pub fn computer_recommended_move(&mut self) -> Option<Move> {
        if self.recommended_move.is_some() {
            return self.recommended_move;
        }
        let play_result = self
            .computer
            .play(&self.rules, &mut self.board, 10, self.current_player);
        if let Ok(play) = play_result {
            self.recommended_move = play.movement;
        }
        self.recommended_move
    }

    pub fn play_computer(&mut self) {
        let play_result = self
            .computer
            .play(&self.rules, &mut self.board, 10, self.current_player);
        if let Ok(play) = play_result {
            let play_time = self.play_time.elapsed();
            if self.computer_average_play_time == 0. {
                self.computer_average_play_time = play_time.as_millis() as f64;
            } else {
                self.computer_average_play_time =
                    (self.computer_average_play_time + play_time.as_millis() as f64) / 2.;
            }
            if play_time > self.computer_highest_play_time {
                self.computer_highest_play_time = play_time;
            }
            if play_time < self.computer_lowest_play_time {
                self.computer_lowest_play_time = play_time;
            }
            println!("computer played: {} in {}ms", play, play_time.as_millis());
            if let Some(movement) = play.movement {
                self.board.set_move(&self.rules, &movement);
                // self.board.display_all_bitboards();
                self.add_rock_move(movement.index);
                if self.board.move_make_win(&self.rules, &movement) {
                    self.player_won();
                } else {
                    self.next_player();
                }
            }
            println!("{}", self.board);
        } else {
            println!("{}", "computer returned an empty play result".red());
        }
    }
}
