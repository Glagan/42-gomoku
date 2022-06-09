use crate::{
    board::{Board, Coordinates, Move},
    computer::{Algorithm, Computer},
    constants::DEPTH,
    heuristic::HEURISTIC,
    player::Player,
    rock::Rock,
    rules::RuleSet,
};
use colored::Colorize;
use std::time::{Duration, Instant};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GameMode {
    None,
    PvP,
    PvA,
    AvA,
}

#[derive(Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(PartialEq)]
pub enum Winner {
    None,
    Black,
    White,
    Draw,
}

pub struct Game {
    pub in_options: bool,
    pub playing: bool,
    pub board: Board,
    pub mode: GameMode,
    pub player_color: Rock,
    pub computer_play_as: Player,
    pub rules: RuleSet,
    pub computer: Computer,
    pub generate_recommended_move: bool,
    pub computer_generated_moves: bool,
    pub computer_expected_moves: Vec<Move>,
    pub play_time: Instant,
    pub previous_play_time: Duration,
    pub computer_average_play_time: f64,
    pub computer_highest_play_time: Duration,
    pub computer_lowest_play_time: Duration,
    pub current_player: Player,
    pub winner: Winner,
    pub rock_move: Vec<Coordinates>,
    pub undone_moves: Vec<Move>,
    pub show_computer_generated_moves: bool,
    pub algorithm_index: Option<usize>,
    pub difficulty_index: Option<usize>,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            in_options: false,
            playing: false,
            board: Board::default(),
            mode: GameMode::None,
            player_color: Rock::None,
            computer_play_as: Player::Black,
            rules: RuleSet::default(),
            computer: Computer::default(),
            generate_recommended_move: false,
            computer_generated_moves: false,
            computer_expected_moves: vec![],
            play_time: Instant::now(),
            previous_play_time: Duration::from_millis(0),
            computer_average_play_time: 0.,
            computer_highest_play_time: Duration::from_millis(0),
            computer_lowest_play_time: Duration::MAX,
            current_player: Player::Black,
            winner: Winner::None,
            rock_move: vec![],
            undone_moves: vec![],
            show_computer_generated_moves: true,
            algorithm_index: Some(0),
            difficulty_index: Some(1),
        }
    }
}

impl Game {
    pub fn reset(&mut self) {
        self.in_options = false;
        self.playing = false;
        self.board = Board::default();
        self.mode = GameMode::None;
        self.player_color = Rock::None;
        self.computer_play_as = Player::Black;
        self.computer = Computer::default();
        self.computer_generated_moves = false;
        self.computer_expected_moves = vec![];
        self.play_time = Instant::now();
        self.previous_play_time = Duration::from_millis(0);
        self.computer_average_play_time = 0.;
        self.computer_highest_play_time = Duration::from_millis(0);
        self.computer_lowest_play_time = Duration::MAX;
        self.current_player = Player::Black;
        self.winner = Winner::None;
        self.rock_move = vec![];
        self.undone_moves = vec![];
        self.difficulty_index = Some(1);
    }

    pub fn start_pva(&mut self, color: Rock) {
        self.reset();
        if self.rules.game_ending_capture && !self.rules.capture {
            self.rules.game_ending_capture = false;
        }
        self.player_color = color;
        if color == Rock::Black {
            self.computer_play_as = Player::White;
        } else {
            self.computer_play_as = Player::Black;
        }
        self.mode = GameMode::PvA;
        self.playing = true;
    }

    pub fn start(&mut self, mode: GameMode) {
        self.reset();
        if self.rules.game_ending_capture && !self.rules.capture {
            self.rules.game_ending_capture = false;
        }
        self.mode = mode;
        println!(
            "Starting a game [{:#?}] ({:#?}) with rules: {:#?}",
            mode,
            self.algorithm(),
            self.rules
        );
        self.playing = true;
    }

    pub fn player_won(&mut self) {
        self.computer_expected_moves = vec![];
        self.winner = match self.current_player {
            Player::Black => Winner::Black,
            Player::White => Winner::White,
        };
    }

    pub fn game_draw(&mut self) {
        self.computer_expected_moves = vec![];
        self.winner = Winner::Draw;
    }

    pub fn next_player(&mut self) {
        if self.current_player == Player::Black {
            self.current_player = Player::White;
        } else {
            self.current_player = Player::Black;
        }
        self.previous_play_time = self.play_time.elapsed();
        self.computer_generated_moves = false;
        // Check draw
        if !self.board.player_can_play(&self.rules, self.current_player) {
            self.game_draw()
        }
        self.play_time = Instant::now();
    }

    pub fn play_player(&mut self, coordinates: Coordinates) {
        if self.board.get(coordinates.x, coordinates.y) == Rock::None {
            let movement = Move {
                coordinates,
                player: self.current_player,
            };
            if self.board.is_move_legal(&self.rules, &movement) {
                let captures = self.board.set_move(&self.rules, &movement);
                println!(
                    "player played: {} with a score of {}",
                    movement,
                    HEURISTIC.movement_score(&self.rules, &self.board, &movement, captures),
                );
                self.computer_generated_moves = false;
                self.rock_move.push(coordinates);
                if self.board.is_winning(&self.rules, movement.player) {
                    self.player_won();
                } else {
                    self.next_player();
                }
                println!("{}", self.board);
            }
        }
    }

    fn algorithm(&self) -> Algorithm {
        let index = self.algorithm_index.unwrap_or_default();
        match index {
            2 => Algorithm::Greedy,
            1 => Algorithm::Minimax,
            _ => Algorithm::Negamax,
        }
    }

    fn difficulty(&self) -> Difficulty {
        let index = self.difficulty_index.unwrap_or_default();
        match index {
            2 => Difficulty::Hard,
            0 => Difficulty::Easy,
            _ => Difficulty::Medium,
        }
    }

    fn difficulty_depth(&self) -> usize {
        match self.difficulty() {
            Difficulty::Hard => DEPTH + 2,
            Difficulty::Medium => DEPTH.max(1),
            Difficulty::Easy => (DEPTH - 2).max(1),
        }
    }

    pub fn generate_computer_recommended_moves(&mut self) {
        if self.computer_generated_moves {
            return;
        }
        let play_result = self.computer.play(
            self.algorithm(),
            &self.rules,
            &mut self.board,
            DEPTH,
            self.current_player,
        );
        if let Ok(play) = play_result {
            if !play.movements.is_empty() {
                self.computer_expected_moves = play.movements;
            }
        }
        self.computer_generated_moves = true;
    }

    pub fn play_computer(&mut self) {
        let depth = if self.mode == GameMode::PvA {
            self.difficulty_depth()
        } else {
            DEPTH
        };
        let play_result = self.computer.play(
            self.algorithm(),
            &self.rules,
            &mut self.board,
            depth,
            self.current_player,
        );
        if let Ok(play) = play_result {
            // Collect times
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
            // Handle the movement
            self.computer_expected_moves = play.movements;
            let mut next_move = self.computer_expected_moves.first();
            if next_move.is_none() {
                let mut legal_moves = self
                    .board
                    .intersections_legal_moves(&self.rules, self.current_player);
                if !legal_moves.is_empty() {
                    self.computer_expected_moves = vec![legal_moves.pop().unwrap()];
                    next_move = self.computer_expected_moves.first();
                }
            }
            if let Some(movement) = next_move {
                let captures = self.board.set_move(&self.rules, movement);
                println!(
                    "computer played: {} with a score of {} in {}ms",
                    movement,
                    HEURISTIC.movement_score(&self.rules, &self.board, movement, captures),
                    play_time.as_millis()
                );
                self.rock_move.push(movement.coordinates);
                if self.board.is_winning(&self.rules, movement.player) {
                    self.player_won();
                } else {
                    self.next_player();
                }
            } else {
                self.game_draw();
            }
            println!("{}", self.board);
        } else {
            println!("{}", "computer returned an empty play result".red());
        }
    }

    pub fn undo_move(&mut self) {
        if let Some(last_coordinates) = self.rock_move.pop() {
            let undone_move = Move {
                coordinates: last_coordinates,
                player: if self.board.get(last_coordinates.x, last_coordinates.y) == Rock::Black {
                    Player::Black
                } else {
                    Player::White
                },
            };
            self.board.undo_move(&self.rules, &undone_move);
            self.undone_moves.push(undone_move);
            self.current_player = self.current_player.opponent();
        }
    }

    pub fn redo_move(&mut self) {
        let last_move = self.undone_moves.pop();
        if let Some(last_move) = last_move {
            self.board.set_move(&self.rules, &last_move);
            self.rock_move.push(last_move.coordinates);
            self.current_player = self.current_player.opponent();
        }
    }
}
