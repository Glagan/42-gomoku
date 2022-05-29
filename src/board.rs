use crate::{
    pattern::Finder,
    player::Player,
    rules::RuleSet,
    transpose::{
        ANTI_DIAGONAL_TRANSPOSE, ANTI_DIAGONAL_TRANSPOSE_REV, CAPTURE_SLICES, DIAGONAL_TRANSPOSE,
        DIAGONAL_TRANSPOSE_REV, VERTICAL_TRANSPOSE, VERTICAL_TRANSPOSE_REV, WINDOW_SLICE_FIVE,
        WINDOW_SLICE_FOUR_LEFT, WINDOW_SLICE_FOUR_RIGHT,
    },
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

#[derive(Default)]
pub struct PlayerState {
    pub captures: usize,
    pub rocks: Vec<usize>,
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

pub struct Board {
    pub boards: [bitvec::array::BitArray<[usize; 6], Lsb0>; 8],
    pub moves: u16,
    pub black: PlayerState,
    pub white: PlayerState,
    pub all_rocks: Vec<usize>,
    pub moves_restore: Vec<Vec<usize>>,
}

impl Default for Board {
    fn default() -> Board {
        let mut moves_restore = vec![];
        moves_restore.reserve(360);
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
            black: PlayerState::default(),
            white: PlayerState::default(),
            all_rocks: vec![],
            moves_restore,
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
                    "{: >3} ",
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
    pub fn display_all_bitboards(&self) {
        println!("Horizontal");
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let index = col + row * BOARD_SIZE;
                print!(
                    "{: >3} ",
                    if !self.boards[Index::HORIZONTAL_WHITE][index] {
                        format!("{}", index).black().on_white()
                    } else if !self.boards[Index::HORIZONTAL_BLACK][index] {
                        format!("{}", index).white().on_bright_black()
                    } else {
                        format!("{}", index).dimmed()
                    }
                );
            }
            if row != BOARD_SIZE - 1 {
                println!();
            }
        }
        println!("\nVertical");
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let index = col + row * BOARD_SIZE;
                print!(
                    "{: >3} ",
                    if !self.boards[Index::VERTICAL_WHITE][index] {
                        format!("{}", index).black().on_white()
                    } else if !self.boards[Index::VERTICAL_BLACK][index] {
                        format!("{}", index).white().on_bright_black()
                    } else {
                        format!("{}", index).dimmed()
                    }
                );
            }
            if row != BOARD_SIZE - 1 {
                println!();
            }
        }
        println!("\nDiagonal");
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let index = col + row * BOARD_SIZE;
                print!(
                    "{: >3} ",
                    if !self.boards[Index::DIAGONAL_WHITE][index] {
                        format!("{}", index).black().on_white()
                    } else if !self.boards[Index::DIAGONAL_BLACK][index] {
                        format!("{}", index).white().on_bright_black()
                    } else {
                        format!("{}", index).dimmed()
                    }
                );
            }
            if row != BOARD_SIZE - 1 {
                println!();
            }
        }
        println!("\nAnti-Diagonal");
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let index = col + row * BOARD_SIZE;
                print!(
                    "{: >3} ",
                    if !self.boards[Index::ANTI_DIAGONAL_WHITE][index] {
                        format!("{}", index).black().on_white()
                    } else if !self.boards[Index::ANTI_DIAGONAL_BLACK][index] {
                        format!("{}", index).white().on_bright_black()
                    } else {
                        format!("{}", index).dimmed()
                    }
                );
            }
            if row != BOARD_SIZE - 1 {
                println!();
            }
        }
        println!("\n---");
    }

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

    // Iterate on each bitboards for the current player to search for the given pattern
    pub fn match_dual_pattern(
        &self,
        rock: usize,
        player: Player,
        slices: &[[(usize, usize); 361]; 4],
        pattern_1: &BitSlice,
        pattern_2: &BitSlice,
    ) -> bool {
        if player == Player::Black {
            // Iterate on each rocks to know if any of them make a five in a row
            let slice = slices[0][rock];
            if self.boards[Index::HORIZONTAL_BLACK][slice.0..=slice.1].eq(pattern_1)
                && self.boards[Index::HORIZONTAL_WHITE][slice.0..=slice.1].eq(pattern_2)
            {
                return true;
            }
            let slice = slices[1][rock];
            if self.boards[Index::VERTICAL_BLACK][slice.0..=slice.1].eq(pattern_1)
                && self.boards[Index::VERTICAL_WHITE][slice.0..=slice.1].eq(pattern_2)
            {
                return true;
            }
            let slice = slices[2][rock];
            if self.boards[Index::DIAGONAL_BLACK][slice.0..=slice.1].eq(pattern_1)
                && self.boards[Index::DIAGONAL_WHITE][slice.0..=slice.1].eq(pattern_2)
            {
                return true;
            }
            let slice = slices[3][rock];
            if self.boards[Index::ANTI_DIAGONAL_BLACK][slice.0..=slice.1].eq(pattern_1)
                && self.boards[Index::ANTI_DIAGONAL_WHITE][slice.0..=slice.1].eq(pattern_2)
            {
                return true;
            }
            false
        } else {
            // Iterate on each rocks to know if any of them make a five in a row
            let slice = slices[0][rock];
            if self.boards[Index::HORIZONTAL_WHITE][slice.0..=slice.1].eq(pattern_1)
                && self.boards[Index::HORIZONTAL_BLACK][slice.0..=slice.1].eq(pattern_2)
            {
                return true;
            }
            let slice = slices[1][rock];
            if self.boards[Index::VERTICAL_WHITE][slice.0..=slice.1].eq(pattern_1)
                && self.boards[Index::VERTICAL_BLACK][slice.0..=slice.1].eq(pattern_2)
            {
                return true;
            }
            let slice = slices[2][rock];
            if self.boards[Index::DIAGONAL_WHITE][slice.0..=slice.1].eq(pattern_1)
                && self.boards[Index::DIAGONAL_BLACK][slice.0..=slice.1].eq(pattern_2)
            {
                return true;
            }
            let slice = slices[3][rock];
            if self.boards[Index::ANTI_DIAGONAL_WHITE][slice.0..=slice.1].eq(pattern_1)
                && self.boards[Index::ANTI_DIAGONAL_BLACK][slice.0..=slice.1].eq(pattern_2)
            {
                return true;
            }
            false
        }
    }

    // Iterate on each bitboards for the current player to search for the given pattern
    pub fn match_pattern(
        &self,
        rock: usize,
        player: Player,
        slices: &[[(usize, usize); 361]; 4],
        pattern: &BitSlice,
    ) -> bool {
        if player == Player::Black {
            // Iterate on each rocks to know if any of them make a five in a row
            let slice = slices[0][rock];
            if self.boards[Index::HORIZONTAL_BLACK][slice.0..=slice.1].eq(pattern) {
                return true;
            }
            let slice = slices[1][rock];
            if self.boards[Index::VERTICAL_BLACK][slice.0..=slice.1].eq(pattern) {
                return true;
            }
            let slice = slices[2][rock];
            if self.boards[Index::DIAGONAL_BLACK][slice.0..=slice.1].eq(pattern) {
                return true;
            }
            let slice = slices[3][rock];
            if self.boards[Index::ANTI_DIAGONAL_BLACK][slice.0..=slice.1].eq(pattern) {
                return true;
            }
            false
        } else {
            // Iterate on each rocks to know if any of them make a five in a row
            let slice = slices[0][rock];
            if self.boards[Index::HORIZONTAL_WHITE][slice.0..=slice.1].eq(pattern) {
                return true;
            }
            let slice = slices[1][rock];
            if self.boards[Index::VERTICAL_WHITE][slice.0..=slice.1].eq(pattern) {
                return true;
            }
            let slice = slices[2][rock];
            if self.boards[Index::DIAGONAL_WHITE][slice.0..=slice.1].eq(pattern) {
                return true;
            }
            let slice = slices[3][rock];
            if self.boards[Index::ANTI_DIAGONAL_WHITE][slice.0..=slice.1].eq(pattern) {
                return true;
            }
            false
        }
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
        let self_pawn = player.rock();
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
        let player_pawn = player.rock();
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
                        Finder::pawn_to_pattern_pawn(self, new_x as usize, new_y as usize, player)
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
            return !self.has_free_three(movement.player);
        }
        true
    }

    // Pattern: [2 1 0 2] or [2 0 1 2] where [0] is the movement index
    fn movement_create_recursive_capture(&self, movement: &Move) -> bool {
        self.match_dual_pattern(
            movement.index,
            movement.player,
            &WINDOW_SLICE_FOUR_LEFT,
            bits![1, 1, 0, 1],
            bits![0, 1, 1, 0],
        ) || self.match_dual_pattern(
            movement.index,
            movement.player,
            &WINDOW_SLICE_FOUR_RIGHT,
            bits![1, 0, 1, 1],
            bits![0, 1, 1, 0],
        )
    }

    // Check if a move *can* be executed according to the rules
    pub fn is_move_legal(&self, rules: &RuleSet, movement: &Move) -> bool {
        // TODO >
        /*// Forbid movements that would create a "double three"
        // Pattern: [1 1 1 0 >< 0 1 1 1] where [><] means any direction change
        if rules.no_double_three && !self.is_move_legal_double_free_three(movement) {
            return false;
        }*/
        // TODO <
        // Forbid movements that would put a pawn in a "recursive capture" state
        if rules.capture && self.movement_create_recursive_capture(movement) {
            return false;
        }
        true
    }

    // All *legal* possible movements from the intersections for a given player
    pub fn intersections_legal_moves(&self, rules: &RuleSet, player: Player) -> Vec<Move> {
        // Analyze each intersections and check if a Pawn can be set on it
        // -- for the current player according to the rules
        let intersections = self.open_intersections();
        let mut moves: Vec<Move> = vec![];
        for index in intersections.iter() {
            let movement = Move {
                player,
                index: *index,
            };
            if self.is_move_legal(rules, &movement) {
                moves.push(movement);
            }
        }
        moves
    }

    // All possible movements from the intersections for a given player
    pub fn intersections_all_moves(&self, rules: &RuleSet, player: Player) -> Vec<PossibleMove> {
        // Analyze each intersections and check if a Pawn can be set on it
        // -- for the current player according to the rules
        let intersections = self.open_intersections();
        let mut moves: Vec<PossibleMove> = vec![];
        for index in intersections.iter() {
            let movement = Move {
                player,
                index: *index,
            };
            moves.push(PossibleMove {
                index: *index,
                legal: self.is_move_legal(rules, &movement),
            });
        }
        moves
    }

    fn get_movement_captures(&mut self, movement: &Move) -> Vec<usize> {
        // Check all 8 directions on a window of 4
        // -- with the movement rock on the "center" of all directions (star pattern)
        let capture_pattern_self = bits![0, 1, 1, 0];
        let capture_pattern_opponent = bits![1, 0, 0, 1];
        let index = movement.index;
        let mut captures: Vec<usize> = vec![];
        let slices = CAPTURE_SLICES[movement.index];
        if movement.player == Player::Black {
            // Left Horizontal
            if self.boards[Index::HORIZONTAL_BLACK][slices.0 .0..=index].eq(capture_pattern_self)
                && self.boards[Index::HORIZONTAL_WHITE][slices.0 .0..=index]
                    .eq(capture_pattern_opponent)
            {
                captures.push(index - 1);
                captures.push(index - 2);
            }
            // Right Horizontal
            if self.boards[Index::HORIZONTAL_BLACK][index..=slices.0 .1].eq(capture_pattern_self)
                && self.boards[Index::HORIZONTAL_WHITE][index..=slices.0 .1]
                    .eq(capture_pattern_opponent)
            {
                captures.push(index + 1);
                captures.push(index + 2);
            }
            // Top Vertical
            let transposed_index = VERTICAL_TRANSPOSE[index];
            if self.boards[Index::VERTICAL_BLACK][slices.1 .0..=transposed_index]
                .eq(capture_pattern_self)
                && self.boards[Index::VERTICAL_WHITE][slices.1 .0..=transposed_index]
                    .eq(capture_pattern_opponent)
            {
                captures.push(VERTICAL_TRANSPOSE_REV[transposed_index - 1]);
                captures.push(VERTICAL_TRANSPOSE_REV[transposed_index - 2]);
            }
            // Bottom Vertical
            if self.boards[Index::VERTICAL_BLACK][transposed_index..=slices.1 .1]
                .eq(capture_pattern_self)
                && self.boards[Index::VERTICAL_WHITE][transposed_index..=slices.1 .1]
                    .eq(capture_pattern_opponent)
            {
                captures.push(VERTICAL_TRANSPOSE_REV[transposed_index + 1]);
                captures.push(VERTICAL_TRANSPOSE_REV[transposed_index + 2]);
            }
            // Top Diagonal
            let transposed_index = DIAGONAL_TRANSPOSE[index];
            if self.boards[Index::DIAGONAL_BLACK][slices.2 .0..=transposed_index]
                .eq(capture_pattern_self)
                && self.boards[Index::DIAGONAL_WHITE][slices.2 .0..=transposed_index]
                    .eq(capture_pattern_opponent)
            {
                captures.push(DIAGONAL_TRANSPOSE_REV[transposed_index - 1]);
                captures.push(DIAGONAL_TRANSPOSE_REV[transposed_index - 2]);
            }
            // Bottom Diagonal
            if self.boards[Index::DIAGONAL_BLACK][transposed_index..=slices.2 .1]
                .eq(capture_pattern_self)
                && self.boards[Index::DIAGONAL_WHITE][transposed_index..=slices.2 .1]
                    .eq(capture_pattern_opponent)
            {
                captures.push(DIAGONAL_TRANSPOSE_REV[transposed_index + 1]);
                captures.push(DIAGONAL_TRANSPOSE_REV[transposed_index + 2]);
            }
            // Top Anti-diagonal
            let transposed_index = ANTI_DIAGONAL_TRANSPOSE[index];
            if self.boards[Index::ANTI_DIAGONAL_BLACK][slices.3 .0..=transposed_index]
                .eq(capture_pattern_self)
                && self.boards[Index::ANTI_DIAGONAL_WHITE][slices.3 .0..=transposed_index]
                    .eq(capture_pattern_opponent)
            {
                captures.push(ANTI_DIAGONAL_TRANSPOSE_REV[transposed_index - 1]);
                captures.push(ANTI_DIAGONAL_TRANSPOSE_REV[transposed_index - 2]);
            }
            // Bottom Anti-diagonal
            if self.boards[Index::ANTI_DIAGONAL_BLACK][transposed_index..=slices.3 .1]
                .eq(capture_pattern_self)
                && self.boards[Index::ANTI_DIAGONAL_WHITE][transposed_index..=slices.3 .1]
                    .eq(capture_pattern_opponent)
            {
                captures.push(ANTI_DIAGONAL_TRANSPOSE_REV[transposed_index + 1]);
                captures.push(ANTI_DIAGONAL_TRANSPOSE_REV[transposed_index + 2]);
            }
        } else {
            // Left Horizontal
            if self.boards[Index::HORIZONTAL_WHITE][slices.0 .0..=index].eq(capture_pattern_self)
                && self.boards[Index::HORIZONTAL_BLACK][slices.0 .0..=index]
                    .eq(capture_pattern_opponent)
            {
                captures.push(index - 1);
                captures.push(index - 2);
            }
            // Right Horizontal
            if self.boards[Index::HORIZONTAL_WHITE][index..=slices.0 .1].eq(capture_pattern_self)
                && self.boards[Index::HORIZONTAL_BLACK][index..=slices.0 .1]
                    .eq(capture_pattern_opponent)
            {
                captures.push(index + 1);
                captures.push(index + 2);
            }
            // Top Vertical
            let transposed_index = VERTICAL_TRANSPOSE[index];
            if self.boards[Index::VERTICAL_WHITE][slices.1 .0..=transposed_index]
                .eq(capture_pattern_self)
                && self.boards[Index::VERTICAL_BLACK][slices.1 .0..=transposed_index]
                    .eq(capture_pattern_opponent)
            {
                captures.push(VERTICAL_TRANSPOSE_REV[transposed_index - 1]);
                captures.push(VERTICAL_TRANSPOSE_REV[transposed_index - 2]);
            }
            // Bottom Vertical
            if self.boards[Index::VERTICAL_WHITE][transposed_index..=slices.1 .1]
                .eq(capture_pattern_self)
                && self.boards[Index::VERTICAL_BLACK][transposed_index..=slices.1 .1]
                    .eq(capture_pattern_opponent)
            {
                captures.push(VERTICAL_TRANSPOSE_REV[transposed_index + 1]);
                captures.push(VERTICAL_TRANSPOSE_REV[transposed_index + 2]);
            }
            // Top Diagonal
            let transposed_index = DIAGONAL_TRANSPOSE[index];
            if self.boards[Index::DIAGONAL_WHITE][slices.2 .0..=transposed_index]
                .eq(capture_pattern_self)
                && self.boards[Index::DIAGONAL_BLACK][slices.2 .0..=transposed_index]
                    .eq(capture_pattern_opponent)
            {
                captures.push(DIAGONAL_TRANSPOSE_REV[transposed_index - 1]);
                captures.push(DIAGONAL_TRANSPOSE_REV[transposed_index - 2]);
            }
            // Bottom Diagonal
            if self.boards[Index::DIAGONAL_WHITE][transposed_index..=slices.2 .1]
                .eq(capture_pattern_self)
                && self.boards[Index::DIAGONAL_BLACK][transposed_index..=slices.2 .1]
                    .eq(capture_pattern_opponent)
            {
                captures.push(DIAGONAL_TRANSPOSE_REV[transposed_index + 1]);
                captures.push(DIAGONAL_TRANSPOSE_REV[transposed_index + 2]);
            }
            // Top Anti-diagonal
            let transposed_index = ANTI_DIAGONAL_TRANSPOSE[index];
            if self.boards[Index::ANTI_DIAGONAL_WHITE][slices.3 .0..=transposed_index]
                .eq(capture_pattern_self)
                && self.boards[Index::ANTI_DIAGONAL_BLACK][slices.3 .0..=transposed_index]
                    .eq(capture_pattern_opponent)
            {
                captures.push(ANTI_DIAGONAL_TRANSPOSE_REV[transposed_index - 1]);
                captures.push(ANTI_DIAGONAL_TRANSPOSE_REV[transposed_index - 2]);
            }
            // Bottom Anti-diagonal
            if self.boards[Index::ANTI_DIAGONAL_WHITE][transposed_index..=slices.3 .1]
                .eq(capture_pattern_self)
                && self.boards[Index::ANTI_DIAGONAL_BLACK][transposed_index..=slices.3 .1]
                    .eq(capture_pattern_opponent)
            {
                captures.push(ANTI_DIAGONAL_TRANSPOSE_REV[transposed_index + 1]);
                captures.push(ANTI_DIAGONAL_TRANSPOSE_REV[transposed_index + 2]);
            }
        }
        captures
    }

    pub fn set_rock(&mut self, index: usize, rock: Rock) {
        if rock == Rock::Black {
            self.boards[Index::HORIZONTAL_BLACK].set(index, false);
            self.boards[Index::VERTICAL_BLACK].set(VERTICAL_TRANSPOSE[index], false);
            self.boards[Index::DIAGONAL_BLACK].set(DIAGONAL_TRANSPOSE[index], false);
            self.boards[Index::ANTI_DIAGONAL_BLACK].set(ANTI_DIAGONAL_TRANSPOSE[index], false);
        } else if rock == Rock::White {
            self.boards[Index::HORIZONTAL_WHITE].set(index, false);
            self.boards[Index::VERTICAL_WHITE].set(VERTICAL_TRANSPOSE[index], false);
            self.boards[Index::DIAGONAL_WHITE].set(DIAGONAL_TRANSPOSE[index], false);
            self.boards[Index::ANTI_DIAGONAL_WHITE].set(ANTI_DIAGONAL_TRANSPOSE[index], false);
        } else {
            self.boards[Index::HORIZONTAL_BLACK].set(index, true);
            self.boards[Index::HORIZONTAL_WHITE].set(index, true);
            self.boards[Index::VERTICAL_BLACK].set(VERTICAL_TRANSPOSE[index], true);
            self.boards[Index::VERTICAL_WHITE].set(VERTICAL_TRANSPOSE[index], true);
            self.boards[Index::DIAGONAL_BLACK].set(DIAGONAL_TRANSPOSE[index], true);
            self.boards[Index::DIAGONAL_WHITE].set(DIAGONAL_TRANSPOSE[index], true);
            self.boards[Index::ANTI_DIAGONAL_BLACK].set(ANTI_DIAGONAL_TRANSPOSE[index], true);
            self.boards[Index::ANTI_DIAGONAL_WHITE].set(ANTI_DIAGONAL_TRANSPOSE[index], true);
        }
    }

    // Apply a movement to the current Board
    pub fn set_move(&mut self, rules: &RuleSet, movement: &Move) {
        // Set rock
        if movement.player == Player::Black {
            self.set_rock(movement.index, Rock::Black);
            self.black.rocks.push(movement.index);
        } else {
            self.set_rock(movement.index, Rock::White);
            self.white.rocks.push(movement.index);
        }
        self.all_rocks.push(movement.index);
        // Check capture
        if rules.capture {
            let captures = self.get_movement_captures(movement);
            if movement.player == Player::Black {
                self.black.captures += captures.len();
            } else {
                self.white.captures += captures.len();
            }
            for rock in &captures {
                // Remove opponent rock from the list of rocks
                if movement.player == Player::Black {
                    self.white.rocks.swap_remove(
                        self.white
                            .rocks
                            .iter()
                            .position(|index| index == rock)
                            .unwrap(),
                    );
                } else {
                    self.black.rocks.swap_remove(
                        self.black
                            .rocks
                            .iter()
                            .position(|index| index == rock)
                            .unwrap(),
                    );
                }
                // ... and from the global list of rock
                self.all_rocks.swap_remove(
                    self.all_rocks
                        .iter()
                        .position(|index| index == rock)
                        .unwrap(),
                );
                self.set_rock(*rock, Rock::None);
            }
            // Save the list of captured rocks to restore for when undo_move is called
            self.moves_restore.push(captures);
        }
        self.moves += 1;
    }

    pub fn undo_move(&mut self, rules: &RuleSet, movement: &Move) {
        // Restored the captured rocks
        if rules.capture {
            let opponent_rock = movement.player.rock().opponent();
            let rocks = self.moves_restore.pop().unwrap();
            // Decrease capture counter
            if movement.player == Player::Black {
                self.black.captures -= rocks.len();
            } else {
                self.white.captures -= rocks.len();
            }
            // Restore the rock index in the opponent list of rocks
            for rock in rocks {
                if movement.player == Player::Black {
                    self.white.rocks.push(rock);
                } else {
                    self.black.rocks.push(rock);
                }
                self.all_rocks.push(rock);
                self.set_rock(rock, opponent_rock);
            }
        }
        // Remove rock
        self.set_rock(movement.index, Rock::None);
        if movement.player == Player::Black {
            self.black.rocks.swap_remove(
                self.black
                    .rocks
                    .iter()
                    .position(|&rock| rock == movement.index)
                    .unwrap(),
            );
        } else {
            self.white.rocks.swap_remove(
                self.white
                    .rocks
                    .iter()
                    .position(|&rock| rock == movement.index)
                    .unwrap(),
            );
        }
        self.all_rocks.swap_remove(
            self.all_rocks
                .iter()
                .position(|&rock| rock == movement.index)
                .unwrap(),
        );
        self.moves -= 1;
    }

    pub fn has_free_three(&self, player: Player) -> bool {
        let free_three_pattern: [usize; 5] = [0, 1, 1, 1, 0];
        let rocks = if player == Player::Black {
            &self.black.rocks
        } else {
            &self.white.rocks
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
    pub fn has_uncaptured_five_in_a_row(&self, player: Player) -> bool {
        let rocks = if player == Player::Black {
            &self.black.rocks
        } else {
            &self.white.rocks
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

    pub fn has_five_in_a_row(&self, player: Player) -> bool {
        let five_in_a_row = bits![0; 5];

        // Iterate on each rocks to know if any of them make a five in a row
        if player == Player::Black {
            for rock in &self.black.rocks {
                if self.match_pattern(*rock, player, &WINDOW_SLICE_FIVE, five_in_a_row) {
                    return true;
                }
            }
        } else {
            for rock in &self.white.rocks {
                if self.match_pattern(*rock, player, &WINDOW_SLICE_FIVE, five_in_a_row) {
                    return true;
                }
            }
        }
        false
    }

    // Check if the given player is winning on the current board
    // (Has an unbreakable winning position according to the rules)
    pub fn is_winning(&self, rules: &RuleSet, player: Player) -> bool {
        if rules.capture
            && ((player == Player::Black && self.black.captures >= 10)
                || (player == Player::White && self.white.captures >= 10))
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
        self.has_five_in_a_row(player)
    }
}
