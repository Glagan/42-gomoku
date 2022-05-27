use crate::{
    pattern::Finder,
    player::Player,
    rules::RuleSet,
    transpose::{ANTI_DIAGONAL_TRANSPOSE, DIAGONAL_TRANSPOSE, VERTICAL_TRANSPOSE},
};
use bitvec::prelude::*;
use colored::Colorize;
use fixed_vec_deque::FixedVecDeque;
use std::fmt;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Rock {
    None = 0,
    Black = 1,
    White = 2,
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

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub player: Player,
    pub index: usize, // Index of the piece to place
}

#[derive(Debug, Clone, Copy)]
pub struct PossibleMove {
    pub index: usize,
    pub legal: bool,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (x, y) = Board::index_to_coordinates(self.index);
        if self.player == Player::Black {
            write!(
                f,
                "{} {} ({}x{})",
                "black".white().on_black(),
                self.index,
                x,
                y
            )
        } else {
            write!(
                f,
                "{} {} ({}x{})",
                "white".black().on_white(),
                self.index,
                x,
                y
            )
        }
    }
}

pub struct Index;
impl Index {
    pub const HORIZONTAL_BLACK: usize = 0;
    pub const HORIZONTAL_WHITE: usize = 1;
    pub const VERTICAL_BLACK: usize = 2;
    pub const VERTICAL_WHITE: usize = 3;
    pub const DIAGONAL_BLACK: usize = 4;
    pub const DIAGONAL_WHITE: usize = 5;
    pub const ANTI_DIAGONAL_BLACK: usize = 6;
    pub const ANTI_DIAGONAL_WHITE: usize = 7;
}

pub const BOARD_SIZE: usize = 19;
pub const BOARD_PIECES: usize = BOARD_SIZE * BOARD_SIZE;
pub const DIRECTIONS: [(i16, i16); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Clone)]
pub struct Board {
    pub boards: [bitvec::array::BitArray<[usize; 6], Lsb0>; 8],
    pub moves: u16,
    pub black_rocks: Vec<usize>,
    pub white_rocks: Vec<usize>,
    pub all_rocks: Vec<usize>,
    pub black_capture: u8,
    pub white_capture: u8,
    // pub capture_moves: HashMap<usize, Vec<usize>>,
}

impl Default for Board {
    fn default() -> Board {
        let mut board = Board {
            boards: [
                bitarr![0; 361],
                bitarr![0; 361],
                bitarr![0; 361],
                bitarr![0; 361],
                bitarr![0; 361],
                bitarr![0; 361],
                bitarr![0; 361],
                bitarr![0; 361],
            ],
            moves: 0,
            black_rocks: vec![],
            white_rocks: vec![],
            all_rocks: vec![],
            black_capture: 0,
            white_capture: 0,
            // capture_moves: HashMap::new(),
        };
        for bitboard in board.boards.iter_mut() {
            bitboard.fill(true);
        }
        board
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                write!(
                    f,
                    "{: >3}",
                    if !self.boards[Index::HORIZONTAL_WHITE][col + row * BOARD_SIZE] {
                        format!("{}", col + row * BOARD_SIZE).black().on_white()
                    } else if !self.boards[Index::HORIZONTAL_BLACK][col + row * BOARD_SIZE] {
                        format!("{}", col + row * BOARD_SIZE)
                            .white()
                            .on_bright_black()
                    } else {
                        format!("{}", col + row * BOARD_SIZE).dimmed()
                    }
                )?;
            }
            if row != BOARD_SIZE - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Board {
    // Helper function to get a Board case with (x, y) coordinates
    pub fn get(&self, x: usize, y: usize) -> Rock {
        let index = Board::coordinates_to_index(x, y);
        if !self.boards[Index::HORIZONTAL_WHITE][index] {
            Rock::White
        } else if !self.boards[Index::HORIZONTAL_BLACK][index] {
            Rock::Black
        } else {
            Rock::None
        }
    }

    pub fn at(&self, index: usize) -> Rock {
        if !self.boards[Index::HORIZONTAL_WHITE][index] {
            Rock::White
        } else if !self.boards[Index::HORIZONTAL_BLACK][index] {
            Rock::Black
        } else {
            Rock::None
        }
    }

    pub fn index_to_coordinates(index: usize) -> (usize, usize) {
        (
            index % BOARD_SIZE,
            (index as f64 / BOARD_SIZE as f64) as usize,
        )
    }

    pub fn coordinates_to_index(x: usize, y: usize) -> usize {
        x + (y * BOARD_SIZE)
    }

    // All open intersections for the current Board
    // -- Empty cases within other pieces
    pub fn open_intersections(&self) -> Vec<usize> {
        // Only the center intersection is available if there is no previous moves
        if self.moves == 0 {
            return vec![((BOARD_SIZE as f64 / 2.) * BOARD_SIZE as f64) as usize];
        }
        let mut intersections: Vec<usize> = vec![];
        for existing_pawn in self.all_rocks.iter() {
            let (x, y) = Board::index_to_coordinates(*existing_pawn);
            let (x, y): (i16, i16) = (x.try_into().unwrap(), y.try_into().unwrap());
            for (mov_x, mov_y) in DIRECTIONS {
                let (new_x, new_y) = (x + mov_x, y + mov_y);
                // Check Board boundaries
                if new_x >= 0
                    && new_y >= 0
                    && (new_x as usize) < BOARD_SIZE
                    && (new_y as usize) < BOARD_SIZE
                {
                    let pawn = self.get(new_x as usize, new_y as usize);
                    if pawn == Rock::None {
                        let index = Board::coordinates_to_index(new_x as usize, new_y as usize);
                        if !intersections.contains(&index) {
                            intersections.push(index);
                        }
                    }
                }
            }
        }
        intersections
    }

    // Pattern: [0 1 1 1 0]
    // Since the move rock can be in any 1 position, we need to check all possible patterns:
    // [0 ? 1 1 0], [0 1 ? 1 0], [0 1 1 ? 0]
    pub fn move_create_free_three_direct_pattern(&self, movement: &Move) -> u8 {
        let player = movement.player;
        let self_pawn = player.pawn();
        let no_pawn = Rock::None;
        let (x, y) = Board::index_to_coordinates(movement.index);

        // Horizontal
        let mut total = 0;
        if (x > 0
            && x < BOARD_SIZE - 3
            && self.get(x - 1, y) == no_pawn
            && self.get(x + 1, y) == self_pawn
            && self.get(x + 2, y) == self_pawn
            && self.get(x + 3, y) == no_pawn)
            || (x > 1
                && x < BOARD_SIZE - 2
                && self.get(x - 2, y) == no_pawn
                && self.get(x - 1, y) == self_pawn
                && self.get(x + 1, y) == self_pawn
                && self.get(x + 2, y) == no_pawn)
            || (x > 2
                && x < BOARD_SIZE - 1
                && self.get(x - 3, y) == no_pawn
                && self.get(x - 2, y) == self_pawn
                && self.get(x - 1, y) == self_pawn
                && self.get(x + 1, y) == no_pawn)
        {
            total += 1;
        }

        // Vertical
        if (y > 0
            && y < BOARD_SIZE - 3
            && self.get(x, y - 1) == no_pawn
            && self.get(x, y + 1) == self_pawn
            && self.get(x, y + 2) == self_pawn
            && self.get(x, y + 3) == no_pawn)
            || (y > 1
                && y < BOARD_SIZE - 2
                && self.get(x, y - 2) == no_pawn
                && self.get(x, y - 1) == self_pawn
                && self.get(x, y + 1) == self_pawn
                && self.get(x, y + 2) == no_pawn)
            || (y > 2
                && y < BOARD_SIZE - 1
                && self.get(x, y - 3) == no_pawn
                && self.get(x, y - 2) == self_pawn
                && self.get(x, y - 1) == self_pawn
                && self.get(x, y + 1) == no_pawn)
        {
            total += 1;
        }

        // Left Diagonal
        if (x > 0
            && x < BOARD_SIZE - 3
            && y > 0
            && y < BOARD_SIZE - 3
            && self.get(x - 1, y - 1) == no_pawn
            && self.get(x + 1, y + 1) == self_pawn
            && self.get(x + 2, y + 2) == self_pawn
            && self.get(x + 3, y + 3) == no_pawn)
            || (x > 1
                && x < BOARD_SIZE - 2
                && y > 1
                && y < BOARD_SIZE - 2
                && self.get(x - 2, y - 2) == no_pawn
                && self.get(x - 1, y - 1) == self_pawn
                && self.get(x + 1, y + 1) == self_pawn
                && self.get(x + 2, y + 2) == no_pawn)
            || (x > 2
                && x < BOARD_SIZE - 1
                && y > 2
                && y < BOARD_SIZE - 1
                && self.get(x - 3, y - 3) == no_pawn
                && self.get(x - 2, y - 2) == self_pawn
                && self.get(x - 1, y - 1) == self_pawn
                && self.get(x + 1, y + 1) == no_pawn)
        {
            total += 1;
        }

        // Right Diagonal
        if (x > 2
            && x < BOARD_SIZE - 1
            && y > 0
            && y < BOARD_SIZE - 3
            && self.get(x + 1, y - 1) == no_pawn
            && self.get(x - 1, y + 1) == self_pawn
            && self.get(x - 2, y + 2) == self_pawn
            && self.get(x - 3, y + 3) == no_pawn)
            || (x > 1
                && x < BOARD_SIZE - 2
                && y > 1
                && y < BOARD_SIZE - 2
                && self.get(x + 2, y - 2) == no_pawn
                && self.get(x + 1, y - 1) == self_pawn
                && self.get(x - 1, y + 1) == self_pawn
                && self.get(x - 2, y + 2) == no_pawn)
            || (x > 0
                && x < BOARD_SIZE - 3
                && y > 2
                && y < BOARD_SIZE - 1
                && self.get(x + 3, y - 3) == no_pawn
                && self.get(x + 2, y - 2) == self_pawn
                && self.get(x + 1, y - 1) == self_pawn
                && self.get(x - 1, y + 1) == no_pawn)
        {
            total += 1;
        }

        total
    }

    // Pattern: [0 1 1 0 1 0] and [0 1 0 1 1 0]
    pub fn move_create_free_three_secondary_pattern(&self, movement: &Move) -> u8 {
        let player = movement.player;
        let player_pawn = player.pawn();
        let pos = Board::index_to_coordinates(movement.index);
        let (x, y): (i16, i16) = (pos.0.try_into().unwrap(), pos.1.try_into().unwrap());
        let mut buf = FixedVecDeque::<[u8; 6]>::new();
        let mut total = 0;
        // Check all 8 directions from the rock to see if there is a free three pattern
        for (dir_x, dir_y) in DIRECTIONS {
            // Create a window of length 6 and update it on each move
            // If there is the given pattern, return true
            let mut length = 0;
            let mut mov_x = dir_x * -6;
            let mut mov_y = dir_y * -6;
            for _ in 0..12 {
                let (new_x, new_y) = (x + mov_x, y + mov_y);
                // Check Board boundaries
                if new_x >= 0
                    && new_y >= 0
                    && (new_x as usize) < BOARD_SIZE
                    && (new_y as usize) < BOARD_SIZE
                {
                    // 1 for player pawn and 0 for anything else
                    *buf.push_back() = if new_x == x && new_y == y
                        || self.get(new_x as usize, new_y as usize) == player_pawn
                    {
                        1
                    } else {
                        Finder::pawn_to_pattern_pawn(self, new_x as usize, new_y as usize, &player)
                    };
                    length += 1;
                    if length >= 6 && (buf == [0, 1, 0, 1, 1, 0] || buf == [0, 1, 1, 0, 1, 0]) {
                        total += 1;
                        continue; // TODO the pattern is counted twice (left/right -> 1 + 1)
                    }
                }
                mov_x += dir_x;
                mov_y += dir_y;
            }
        }

        total
    }

    // Pattern: [0 1 1 1 0] and [0 1 1 0 1 0] ([0 1 0 1 1 0] is just *right* and the original is left)
    // For the pattern to be considered a free-three, it strictly need to have both ends "free"
    // -- so borders does *not* count
    pub fn move_create_free_three(&self, movement: &Move) -> u8 {
        self.move_create_free_three_direct_pattern(movement)
            + self.move_create_free_three_secondary_pattern(movement)
    }

    fn is_move_legal_double_free_three(&self, movement: &Move) -> bool {
        let created_free_threes = self.move_create_free_three(movement);
        if created_free_threes >= 2 {
            return false;
        }
        if created_free_threes == 1 {
            return !self.has_free_three(&movement.player);
        }
        true
    }

    // Pattern: [2 1 0 2] or [2 0 1 2] where [0] is the movement index
    fn is_move_legal_recursive_capture(&self, movement: &Move) -> bool {
        let player = movement.player;
        let (x, y) = Board::index_to_coordinates(movement.index);
        let self_pawn = player.pawn();
        let other_pawn = self_pawn.opponent();

        // Left
        if (x > 1
            && x < BOARD_SIZE - 1
            && self.get(x - 1, y) == self_pawn
            && self.get(x - 2, y) == other_pawn
            && self.get(x + 1, y) == other_pawn)
            // Right
            || (x > 0
                && x < BOARD_SIZE - 2
                && self.get(x - 1, y) == other_pawn
                && self.get(x + 1, y) == self_pawn
                && self.get(x + 2, y) == other_pawn)
            // Top
            || (y > 1
                && y < BOARD_SIZE - 1
                && self.get(x, y - 1) == self_pawn
                && self.get(x, y - 2) == other_pawn
                && self.get(x, y + 1) == other_pawn)
            // Bottom
            || (y > 0
                && y < BOARD_SIZE - 2
                && self.get(x, y - 1) == other_pawn
                && self.get(x, y + 1) == self_pawn
                && self.get(x, y + 2) == other_pawn)
            // Top-Left
            || (x > 1
                && y > 1
                && x < BOARD_SIZE - 1
                && y < BOARD_SIZE - 1
                && self.get(x - 1, y - 1) == self_pawn
                && self.get(x - 2, y - 2) == other_pawn
                && self.get(x + 1, y + 1) == other_pawn)
            // Top-Right
            || (x > 0
                && y > 1
                && x < BOARD_SIZE - 2
                && y < BOARD_SIZE - 1
                && self.get(x + 1, y - 1) == self_pawn
                && self.get(x + 2, y - 2) == other_pawn
                && self.get(x - 1, y + 1) == other_pawn)
            // Bottom-Left
            || (x > 1
                && y > 0
                && x < BOARD_SIZE - 1
                && y < BOARD_SIZE - 2
                && self.get(x - 1, y + 1) == self_pawn
                && self.get(x - 2, y + 2) == other_pawn
                && self.get(x + 1, y - 1) == other_pawn)
            // Bottom-Right
            || (x > 0
                && y > 0
                && x < BOARD_SIZE - 2
                && y < BOARD_SIZE - 2
                && self.get(x + 1, y + 1) == self_pawn
                && self.get(x + 2, y + 2) == other_pawn
                && self.get(x - 1, y - 1) == other_pawn)
        {
            return false;
        }

        true
    }

    // Check if a move *can* be executed according to the rules
    pub fn is_move_legal(&self, rules: &RuleSet, movement: &Move) -> bool {
        // TODO >
        /*// Forbid movements that would create a "double three"
        // Pattern: [1 1 1 0 >< 0 1 1 1] where [><] means any direction change
        if rules.no_double_three && !self.is_move_legal_double_free_three(movement) {
            return false;
        }
        // Forbid movements that would put a pawn in a "recursive capture" state
        if rules.capture && !self.is_move_legal_recursive_capture(movement) {
            return false;
        }*/
        // TODO <
        true
    }

    // All *legal* possible movements from the intersections for a given player
    pub fn intersections_legal_moves(&self, rules: &RuleSet, player: &Player) -> Vec<Move> {
        // Analyze each intersections and check if a Pawn can be set on it
        // -- for the current player according to the rules
        let intersections = self.open_intersections();
        let mut moves: Vec<Move> = vec![];
        for index in intersections.iter() {
            let movement = Move {
                player: *player,
                index: *index,
            };
            if self.is_move_legal(rules, &movement) {
                moves.push(movement);
            }
        }
        moves
    }

    // All possible movements from the intersections for a given player
    // TODO Star pattern
    pub fn intersections_all_moves(&self, rules: &RuleSet, player: &Player) -> Vec<PossibleMove> {
        // Analyze each intersections and check if a Pawn can be set on it
        // -- for the current player according to the rules
        let intersections = self.open_intersections();
        let mut moves: Vec<PossibleMove> = vec![];
        for index in intersections.iter() {
            let movement = Move {
                player: *player,
                index: *index,
            };
            moves.push(PossibleMove {
                index: *index,
                legal: self.is_move_legal(rules, &movement),
            });
        }
        moves
    }

    /*fn check_capture(&mut self, movement: &Move) {
        let player_pawn: Rock;
        let opponant_pawn: Rock;
        let (x, y) = Board::index_to_coordinates(movement.index);
        let mut remove_vect: Vec<usize> = vec![];

        // println!(
        //     "Check_capture : index {} | x : {} | y : {}",
        //     movement.index, x, y
        // );
        if movement.player == Player::Black {
            player_pawn = Rock::Black;
            opponant_pawn = Rock::White;
        } else {
            player_pawn = Rock::White;
            opponant_pawn = Rock::Black;
        }

        if x >= 3
            && self.pieces[movement.index - 1] == opponant_pawn
            && self.pieces[movement.index - 2] == opponant_pawn
            && self.pieces[movement.index - 3] == player_pawn
        {
            remove_vect.push(movement.index - 1);
            remove_vect.push(movement.index - 2);
        }
        // println!("1 if");
        if x + 3 < BOARD_SIZE
            && self.pieces[movement.index + 1] == opponant_pawn
            && self.pieces[movement.index + 2] == opponant_pawn
            && self.pieces[movement.index + 3] == player_pawn
        {
            remove_vect.push(movement.index + 1);
            remove_vect.push(movement.index + 2);
        }
        // println!("2 if");
        if y >= 3
            && self.pieces[movement.index - BOARD_SIZE] == opponant_pawn
            && self.pieces[movement.index - (BOARD_SIZE * 2)] == opponant_pawn
            && self.pieces[movement.index - (BOARD_SIZE * 3)] == player_pawn
        {
            remove_vect.push(movement.index - BOARD_SIZE);
            remove_vect.push(movement.index - (BOARD_SIZE * 2));
        }
        // println!("3 if");
        if y + 3 < BOARD_SIZE
            && self.pieces[movement.index + BOARD_SIZE] == opponant_pawn
            && self.pieces[movement.index + BOARD_SIZE * 2] == opponant_pawn
            && self.pieces[movement.index + BOARD_SIZE * 3] == player_pawn
        {
            remove_vect.push(movement.index + BOARD_SIZE);
            remove_vect.push(movement.index + BOARD_SIZE * 2);
        }
        // println!("4 if");
        if y >= 3
            && x >= 3
            && self.pieces[movement.index - BOARD_SIZE - 1] == opponant_pawn
            && self.pieces[movement.index - (BOARD_SIZE * 2) - 2] == opponant_pawn
            && self.pieces[movement.index - (BOARD_SIZE * 3) - 3] == player_pawn
        {
            remove_vect.push(movement.index - BOARD_SIZE - 1);
            remove_vect.push(movement.index - (BOARD_SIZE * 2) - 2);
        }
        // println!("5 if");
        if y + 3 < BOARD_SIZE
            && x >= 3
            && self.pieces[movement.index + BOARD_SIZE - 1] == opponant_pawn
            && self.pieces[movement.index + (BOARD_SIZE * 2) - 2] == opponant_pawn
            && self.pieces[movement.index + (BOARD_SIZE * 3) - 3] == player_pawn
        {
            remove_vect.push(movement.index + BOARD_SIZE - 1);
            remove_vect.push(movement.index + (BOARD_SIZE * 2) - 2);
        }
        // println!("6 if");
        if y + 3 < BOARD_SIZE
            && x + 3 <= BOARD_SIZE
            && self.pieces[movement.index + BOARD_SIZE + 1] == opponant_pawn
            && self.pieces[movement.index + (BOARD_SIZE * 2) + 2] == opponant_pawn
            && self.pieces[movement.index + (BOARD_SIZE * 3) + 3] == player_pawn
        {
            remove_vect.push(movement.index + BOARD_SIZE + 1);
            remove_vect.push(movement.index + (BOARD_SIZE * 2) + 2);
        }
        // println!("7 if");
        if y >= 3
            && x + 3 <= BOARD_SIZE
            && self.pieces[movement.index - BOARD_SIZE + 1] == opponant_pawn
            && self.pieces[movement.index - (BOARD_SIZE * 2) + 2] == opponant_pawn
            && self.pieces[movement.index - (BOARD_SIZE * 3) + 3] == player_pawn
        {
            remove_vect.push(movement.index - BOARD_SIZE + 1);
            remove_vect.push(movement.index - (BOARD_SIZE * 2) + 2);
        }
        // println!("8 if");
        for &idx in remove_vect.iter() {
            // println!("try to remove : {}", idx);
            self.pieces[idx] = Rock::None;
            if player_pawn == Rock::Black {
                self.black_capture += 1;
                self.white_rocks
                    .remove(self.white_rocks.iter().position(|x| *x == idx).unwrap());
                self.all_rocks
                    .remove(self.all_rocks.iter().position(|x| *x == idx).unwrap());
            } else {
                self.white_capture += 1;
                self.black_rocks
                    .remove(self.black_rocks.iter().position(|x| *x == idx).unwrap());
                self.all_rocks
                    .remove(self.all_rocks.iter().position(|x| *x == idx).unwrap());
            }
        }
        // if !remove_vect.is_empty() {
        //     self.capture_moves.insert(movement.index, remove_vect);
        // }
    }*/

    // Apply a movement to the current Board
    pub fn set_move(&mut self, rules: &RuleSet, movement: &Move) {
        if movement.player == Player::Black {
            self.boards[Index::HORIZONTAL_BLACK].set(movement.index, false);
            self.boards[Index::VERTICAL_BLACK].set(VERTICAL_TRANSPOSE[movement.index], false);
            self.boards[Index::DIAGONAL_BLACK].set(DIAGONAL_TRANSPOSE[movement.index], false);
            self.boards[Index::ANTI_DIAGONAL_BLACK]
                .set(ANTI_DIAGONAL_TRANSPOSE[movement.index], false);
            self.black_rocks.push(movement.index);
        } else {
            self.boards[Index::HORIZONTAL_WHITE].set(movement.index, false);
            self.boards[Index::VERTICAL_WHITE].set(VERTICAL_TRANSPOSE[movement.index], false);
            self.boards[Index::DIAGONAL_WHITE].set(DIAGONAL_TRANSPOSE[movement.index], false);
            self.boards[Index::ANTI_DIAGONAL_WHITE]
                .set(ANTI_DIAGONAL_TRANSPOSE[movement.index], false);
            self.white_rocks.push(movement.index);
        }
        self.all_rocks.push(movement.index);
        self.moves += 1;
        // TODO >
        // if rules.capture {
        //     self.check_capture(movement);
        // }
        // TODO <
        // for bitboard in self.boards.iter() {
        //     println!("{}", bitboard.to_string());
        // }
        // println!("---")
    }

    pub fn undo_move(&mut self, rules: &RuleSet, movement: &Move) {
        // TODO >
        /*if rules.capture && self.capture_moves.contains_key(&movement.index) {
            let opponent_pawn = movement.player.pawn().opponent();
            let rocks = if opponent_pawn == Pawn::Black {
                &mut self.black_rocks
            } else {
                &mut self.white_rocks
            };
            let captures = if opponent_pawn == Pawn::Black {
                &mut self.white_capture
            } else {
                &mut self.black_capture
            };
            for &captured_rock in self.capture_moves.get(&movement.index).unwrap() {
                self.pieces[captured_rock] = opponent_pawn;
                rocks.push(captured_rock);
                self.all_rocks.push(captured_rock);
                *captures -= 1;
            }
            self.capture_moves.remove(&movement.index);
        }*/
        // TODO <
        if movement.player == Player::Black {
            self.boards[Index::HORIZONTAL_BLACK].set(movement.index, true);
            self.boards[Index::VERTICAL_BLACK].set(VERTICAL_TRANSPOSE[movement.index], true);
            self.boards[Index::DIAGONAL_BLACK].set(DIAGONAL_TRANSPOSE[movement.index], true);
            self.boards[Index::ANTI_DIAGONAL_BLACK]
                .set(ANTI_DIAGONAL_TRANSPOSE[movement.index], true);
            self.black_rocks.remove(
                self.black_rocks
                    .iter()
                    .position(|x| *x == movement.index)
                    .unwrap(),
            );
        } else {
            self.boards[Index::HORIZONTAL_WHITE].set(movement.index, true);
            self.boards[Index::VERTICAL_WHITE].set(VERTICAL_TRANSPOSE[movement.index], true);
            self.boards[Index::DIAGONAL_WHITE].set(DIAGONAL_TRANSPOSE[movement.index], true);
            self.boards[Index::ANTI_DIAGONAL_WHITE]
                .set(ANTI_DIAGONAL_TRANSPOSE[movement.index], true);
            self.white_rocks.remove(
                self.white_rocks
                    .iter()
                    .position(|x| *x == movement.index)
                    .unwrap(),
            );
        }
        self.all_rocks.remove(
            self.all_rocks
                .iter()
                .position(|x| *x == movement.index)
                .unwrap(),
        );
        self.moves -= 1;
    }

    pub fn has_free_three(&self, player: &Player) -> bool {
        let free_three_pattern: [usize; 5] = [0, 1, 1, 1, 0];
        let rocks = if player == &Player::Black {
            &self.black_rocks
        } else {
            &self.white_rocks
        };
        for rock in rocks.iter() {
            let pos = Board::index_to_coordinates(*rock);
            let (x, y): (i16, i16) = (pos.0.try_into().unwrap(), pos.1.try_into().unwrap());
            // Check all 8 directions from the rock to see if there is a free three pattern
            for (dir_x, dir_y) in DIRECTIONS {
                // Create a window of length 6 and update it on each move
                // If there is the given pattern, return true
                let mut length = 0;
                // from [? ? ? ? ?] ? ? ? ? I ? ? ? ?
                // to    ? ? ? ? ?  ? ? ? ? [I ? ? ? ?]
                let mut buf = FixedVecDeque::<[usize; 6]>::new();
                let mut mov_x = dir_x * -6;
                let mut mov_y = dir_y * -6;
                for _ in 0..12 {
                    let (new_x, new_y) = (x + mov_x, y + mov_y);
                    // Check Board boundaries
                    if new_x >= 0
                        && new_y >= 0
                        && (new_x as usize) < BOARD_SIZE
                        && (new_y as usize) < BOARD_SIZE
                    {
                        // 1 for player pawn and 0 for anything else
                        *buf.push_back() = Finder::pawn_to_pattern_pawn(
                            self,
                            new_x as usize,
                            new_y as usize,
                            player,
                        ) as usize;
                        length += 1;
                        // buf.contains([0, 1, 1, 1, 0]
                        if length >= 5 {
                            let mut i = 0;
                            for value in &buf {
                                if *value == free_three_pattern[i] {
                                    i += 1;
                                    if i == 5 {
                                        return true;
                                    }
                                } else {
                                    i = 0;
                                }
                            }
                        }
                        if length >= 6 && buf == [0, 1, 0, 1, 1, 0] {
                            return true;
                        }
                    }
                    mov_x += dir_x;
                    mov_y += dir_y;
                }
            }
        }
        false
    }

    // Pattern: [0 1 1 2] where
    // With the rock possibly in either [1] positions
    fn rock_can_be_captured(&self, index: usize) -> bool {
        let (x, y) = Board::index_to_coordinates(index);
        let self_pawn = self.get(x, y);
        let no_pawn = Rock::None;
        let other_pawn = self_pawn.opponent();

        // Horizontal
        if (x > 0
            && x < BOARD_SIZE - 2
            && ((self.get(x - 1, y) == no_pawn
                && self.get(x + 1, y) == self_pawn
                && self.get(x + 2, y) == other_pawn)
                || (self.get(x - 1, y) == other_pawn
                    && self.get(x + 1, y) == self_pawn
                    && self.get(x + 2, y) == no_pawn)))
            || (x > 1
                && x < BOARD_SIZE - 1
                && ((self.get(x - 2, y) == no_pawn
                    && self.get(x - 1, y) == self_pawn
                    && self.get(x + 1, y) == other_pawn)
                    || (self.get(x - 2, y) == other_pawn
                        && self.get(x - 1, y) == self_pawn
                        && self.get(x + 1, y) == no_pawn))) ||

        // Vertical
          (y > 0
            && y < BOARD_SIZE - 2
            && ((self.get(x, y - 1) == no_pawn
                && self.get(x, y + 1) == self_pawn
                && self.get(x, y + 2) == other_pawn)
                || (self.get(x, y - 1) == other_pawn
                    && self.get(x, y + 1) == self_pawn
                    && self.get(x, y + 2) == no_pawn)))
            || (y > 1
                && y < BOARD_SIZE - 1
                && ((self.get(x, y - 2) == no_pawn
                    && self.get(x, y - 1) == self_pawn
                    && self.get(x, y + 1) == other_pawn)
                    || (self.get(x, y - 2) == other_pawn
                        && self.get(x, y - 1) == self_pawn
                        && self.get(x, y + 1) == no_pawn))) ||
                        // Left Diagonal
                         (x > 0
                            && x < BOARD_SIZE - 2
                            && y > 0
                            && y < BOARD_SIZE - 2
                            && ((self.get(x - 1, y - 1) == no_pawn
                                && self.get(x + 1, y + 1) == self_pawn
                                && self.get(x + 2, y + 2) == other_pawn)
                                || (self.get(x - 1, y - 1) == other_pawn
                                    && self.get(x + 1, y + 1) == self_pawn
                                    && self.get(x + 2, y + 2) == no_pawn)))
                            || (x > 1
                                && x < BOARD_SIZE - 1
                                && y > 1
                                && y < BOARD_SIZE - 1
                                && ((self.get(x - 2, y - 2) == no_pawn
                                    && self.get(x - 1, y - 1) == self_pawn
                                    && self.get(x + 1, y + 1) == other_pawn)
                                    || (self.get(x - 2, y - 2) == other_pawn
                                        && self.get(x - 1, y - 1) == self_pawn
                                        && self.get(x + 1, y + 1) == no_pawn)))||
                                        // Right Diagonal
                                         (x > 1
                                            && x < BOARD_SIZE - 1
                                            && y > 0
                                            && y < BOARD_SIZE - 2
                                            && ((self.get(x + 1, y - 1) == no_pawn
                                                && self.get(x - 1, y + 1) == self_pawn
                                                && self.get(x - 2, y + 2) == other_pawn)
                                                || (self.get(x + 1, y - 1) == other_pawn
                                                    && self.get(x - 1, y + 1) == self_pawn
                                                    && self.get(x - 2, y + 2) == no_pawn)))
                                            || (x > 0
                                                && x < BOARD_SIZE - 2
                                                && y > 1
                                                && y < BOARD_SIZE - 1
                                                && ((self.get(x + 2, y - 2) == no_pawn
                                                    && self.get(x + 1, y - 1) == self_pawn
                                                    && self.get(x - 1, y + 1) == other_pawn)
                                                    || (self.get(x + 2, y - 2) == other_pawn
                                                        && self.get(x + 1, y - 1) == self_pawn
                                                        && self.get(x - 1, y + 1) == no_pawn)))
        {
            return true;
        }

        false
    }

    // Check to see if the player has a five in a row
    // -- and *then* check if this five in a row can be captured and destroyed
    // A five in a row can be captured with the following pattern in any direction
    // [0 0 0 0 0]
    // [1 0 0 0 0] with 1 in any position, but mirrored v
    // [1 1 1 1 1]                                      |
    // [2 0 0 0 0]                        in this "row" ^
    // Diagonals also need to be checked
    // [0 0 0 0 0 0]
    // [0 1 0 0 0 0] with 1 in any position, but mirrored v
    // [0 1 1 1 1 1]                                      |
    // [0 0 0 2 0 0]                        in this "row" ^
    pub fn has_uncaptured_five_in_a_row(&self, player: &Player) -> bool {
        let rocks = if player == &Player::Black {
            &self.black_rocks
        } else {
            &self.white_rocks
        };
        let mut buf = FixedVecDeque::<[u8; 5]>::new();
        let mut index_buf = FixedVecDeque::<[usize; 5]>::new();
        for rock in rocks.iter() {
            let pos = Board::index_to_coordinates(*rock);
            let (x, y): (i16, i16) = (pos.0.try_into().unwrap(), pos.1.try_into().unwrap());
            // Check all 8 directions from the rock to see if there is five in a row
            for (dir_x, dir_y) in DIRECTIONS {
                // Create a window of length 5 and update it on each move
                // If there is five in a row in the window, return true
                let mut length = 0;
                // from [? ? ? ? ?] ? ? ? ? I ? ? ? ?
                // to    ? ? ? ? ?  ? ? ? ? [I ? ? ? ?]
                let mut mov_x = dir_x * -5;
                let mut mov_y = dir_y * -5;
                for _ in 0..10 {
                    let (new_x, new_y) = (x + mov_x, y + mov_y);
                    // Check Board boundaries
                    if new_x >= 0
                        && new_y >= 0
                        && (new_x as usize) < BOARD_SIZE
                        && (new_y as usize) < BOARD_SIZE
                    {
                        // 1 for player pawn and 0 for anything else
                        *buf.push_back() = Finder::pawn_to_pattern_pawn(
                            self,
                            new_x as usize,
                            new_y as usize,
                            player,
                        );
                        *index_buf.push_back() =
                            Board::coordinates_to_index(new_x as usize, new_y as usize);
                        length += 1;
                        if length >= 5
                            && buf == [1, 1, 1, 1, 1]
                            && index_buf
                                .iter()
                                .all(|&index| !self.rock_can_be_captured(index))
                        {
                            return true;
                        }
                    }
                    mov_x += dir_x;
                    mov_y += dir_y;
                }
            }
        }

        false
    }

    pub fn has_five_in_a_row(&self, player: &Player) -> bool {
        let rocks = if player == &Player::Black {
            &self.black_rocks
        } else {
            &self.white_rocks
        };
        for rock in rocks.iter() {
            let pos = Board::index_to_coordinates(*rock);
            let (x, y): (i16, i16) = (pos.0.try_into().unwrap(), pos.1.try_into().unwrap());
            // Check all 8 directions from the rock to see if there is five in a row
            for (dir_x, dir_y) in DIRECTIONS {
                // Create a window of length 5 and update it on each move
                // If there is five in a row in the window, return true
                let mut length = 0;
                // from [? ? ? ? ?] ? ? ? ? I ? ? ? ?
                // to    ? ? ? ? ?  ? ? ? ? [I ? ? ? ?]
                let mut buf = FixedVecDeque::<[u8; 5]>::new();
                let mut mov_x = dir_x * -5;
                let mut mov_y = dir_y * -5;
                for _ in 0..10 {
                    let (new_x, new_y) = (x + mov_x, y + mov_y);
                    // Check Board boundaries
                    if new_x >= 0
                        && new_y >= 0
                        && (new_x as usize) < BOARD_SIZE
                        && (new_y as usize) < BOARD_SIZE
                    {
                        // 1 for player pawn and 0 for anything else
                        *buf.push_back() = Finder::pawn_to_pattern_pawn(
                            self,
                            new_x as usize,
                            new_y as usize,
                            player,
                        );
                        length += 1;
                        if length >= 5 && buf == [1, 1, 1, 1, 1] {
                            return true;
                        }
                    }
                    mov_x += dir_x;
                    mov_y += dir_y;
                }
            }
        }
        false
    }

    // Check if the given player is winning on the current board
    // (Has an unbreakable winning position according to the rules)
    pub fn is_winning(&self, rules: &RuleSet, player: &Player) -> bool {
        if rules.capture
            && ((player == &Player::Black && self.black_capture >= 10)
                || (player == &Player::White && self.white_capture >= 10))
        {
            return true;
        }
        // TODO >
        // if rules.game_ending_capture {
        //     self.has_uncaptured_five_in_a_row(player)
        // } else {
        //     self.has_five_in_a_row(player)
        // }
        // TODO <
        false
    }
}
