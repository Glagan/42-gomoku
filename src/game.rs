use crate::{
    board::{Board, Coordinates, Move},
    computer::{Algorithm, Computer},
    constants::DEPTH,
    heuristic::HEURISTIC,
    macros::coord,
    player::Player,
    rock::Rock,
    rules::RuleSet,
};
use colored::Colorize;
use rand::Rng;
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

#[derive(Debug, PartialEq)]
pub enum Opening {
    None,
    Swap2,
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
    pub opening_index: Option<usize>,
    pub completed_opening: bool,
    pub ask_player_choice: bool,
    pub player_place_stones: usize,
    pub placed_three_stones: bool,
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
            opening_index: Some(0),
            completed_opening: false,
            ask_player_choice: false,
            player_place_stones: 0,
            placed_three_stones: false,
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
        self.completed_opening = false;
        self.ask_player_choice = false;
        self.player_place_stones = 0;
        self.placed_three_stones = false;
    }

    pub fn start_pva(&mut self, color: Rock) {
        self.reset();
        if self.rules.game_ending_capture && !self.rules.capture {
            self.rules.game_ending_capture = false;
        }
        self.player_color = color;
        self.mode = GameMode::PvA;
        self.playing = true;
        if color == Rock::Black {
            self.computer_play_as = Player::White;
            if self.opening() == Opening::Swap2 {
                self.player_place_stones = 3
            }
        } else {
            self.computer_play_as = Player::Black;
            if self.opening() == Opening::Swap2 {
                self.random_swap2_opening();
                self.ask_player_choice = true;
            }
        }
    }

    pub fn start(&mut self, mode: GameMode) {
        self.reset();
        if self.rules.game_ending_capture && !self.rules.capture {
            self.rules.game_ending_capture = false;
        }
        self.mode = mode;
        println!(
            "Starting a game [{:#?}] ({:#?}) with rules: {:#?} opening {:#?}",
            mode,
            self.algorithm(),
            self.rules,
            self.opening()
        );
        self.playing = true;
        if self.opening() == Opening::Swap2 {
            // Place the first 3 stones in the main loop if a human player plays first
            if self.mode == GameMode::PvP {
                self.player_place_stones = 3
            }
            // Play the opening directly in AvA
            else if self.mode == GameMode::AvA {
                self.random_swap2_opening();
                // Randomly choose an option
                let mut rng = rand::thread_rng();
                let rand = rng.gen_range(1..=3);
                // Option 3, place more stone and resume thegame
                if rand == 3 {
                    // These stones are randomly generated
                    let coordinates = coord!(8, 8);
                    self.board.set_move(
                        &self.rules,
                        &Move {
                            coordinates,
                            player: Player::White,
                        },
                    );
                    self.rock_move.push(coordinates);
                    let coordinates = coord!(10, 10);
                    self.board.set_move(
                        &self.rules,
                        &Move {
                            coordinates,
                            player: Player::Black,
                        },
                    );
                    self.rock_move.push(coordinates);
                }
                // rand 1 and 2 change the player that play, but in AvA mode it does nothing
                self.completed_opening = true;
            }
        }
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

    pub fn play_opening(&mut self, coordinates: Coordinates) {
        if self.board.get(coordinates.x, coordinates.y) == Rock::None {
            let movement = Move {
                coordinates,
                player: self.current_player,
            };
            if self.board.is_move_legal(&self.rules, &movement) {
                self.board.set_move(&self.rules, &movement);
                self.rock_move.push(coordinates);
                self.next_player();
                self.player_place_stones -= 1;
                if self.player_place_stones == 0 {
                    // In PvA if the player placed 3 stones
                    // -- the computer randomly select an option
                    // -- and either pass the choice to the player or make the game resume
                    if self.mode == GameMode::PvA {
                        let mut rng = rand::thread_rng();
                        // Computer choose to play as black or white
                        if self.computer_play_as == Player::Black {
                            self.placed_three_stones = true;
                            let rand = rng.gen_range(1..=2);
                            if rand == 1 {
                                println!("Computer will play as Black");
                                self.computer_play_as = Player::Black;
                            } else {
                                println!("Computer will play as White");
                                self.computer_play_as = Player::White;
                            }
                            self.completed_opening = true;
                        }
                        // Computer choose to play or randomly play 2 stones and givee the choice back to the player
                        else if self.computer_play_as == Player::White {
                            let rand = rng.gen_range(1..=3);
                            // Option 1 and 2 resume the game
                            if rand == 1 {
                                println!("Computer will play as Black");
                                self.computer_play_as = Player::Black;
                                self.completed_opening = true;
                            } else if rand == 2 {
                                println!("Computer will play as White");
                                self.computer_play_as = Player::White;
                                self.completed_opening = true;
                            }
                            // Option 3, place more stone and ask the player
                            else {
                                self.place_random_stones(Player::White, 2);
                                self.placed_three_stones = true;
                                self.ask_player_choice = true;
                            }
                        }
                    }
                    // In PvP, the first 3 placed stones then ask the second player
                    // -- if he choose to place 2 stones again, it will then limit to select black or white
                    else if self.mode == GameMode::PvP {
                        self.ask_player_choice = true;
                    }
                }
            }
        }
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

    pub fn opening(&self) -> Opening {
        let index = self.opening_index.unwrap_or_default();
        match index {
            1 => Opening::Swap2,
            _ => Opening::None,
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

    pub fn random_opening_coordinates(&self) -> Coordinates {
        let mut rng = rand::thread_rng();
        let (mut x, mut y) = (rng.gen_range(7..=11), rng.gen_range(7..=11));
        while self.board.get(x, y) != Rock::None {
            x = rng.gen_range(7..=11);
            y = rng.gen_range(7..=11);
        }
        coord!(x, y)
    }

    pub fn place_random_stones(&mut self, starts_with: Player, amount: u8) {
        let mut player = starts_with;
        for _ in 0..amount {
            let coordinates = self.random_opening_coordinates();
            self.board.set_move(
                &self.rules,
                &Move {
                    coordinates,
                    player,
                },
            );
            self.rock_move.push(coordinates);
            player = player.opponent();
        }
    }

    pub fn random_swap2_opening(&mut self) {
        // Place 3 randoms stone, 2 Black and 1 White
        self.place_random_stones(Player::Black, 3);
        // Pass the choice to the other player
        self.current_player = Player::White;
        self.ask_player_choice = true;
    }

    pub fn play_as(&mut self, player: Player) {
        if self.mode == GameMode::PvA {
            if player == Player::Black {
                self.computer_play_as = Player::White;
            } else {
                self.computer_play_as = Player::Black;
            }
        }
        self.completed_opening = true;
    }

    pub fn player_place_stone(&mut self) {
        self.player_place_stones = 2;
        self.current_player = Player::White;
        self.computer_play_as = Player::White;
        self.placed_three_stones = true;
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
