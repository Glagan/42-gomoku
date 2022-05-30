use crate::{
    pattern::Finder,
    player::Player,
    rules::RuleSet,
    transpose::{
        ANTI_DIAGONAL_TRANSPOSE, ANTI_DIAGONAL_TRANSPOSE_REV, CAPTURE_SLICES, DIAGONAL_TRANSPOSE,
        DIAGONAL_TRANSPOSE_REV, VERTICAL_TRANSPOSE, VERTICAL_TRANSPOSE_REV, WINDOW_SLICE_FIVE_1,
        WINDOW_SLICE_FIVE_2, WINDOW_SLICE_FIVE_3, WINDOW_SLICE_FOUR_LEFT, WINDOW_SLICE_FOUR_RIGHT,
        WINDOW_SLICE_SIX_1, WINDOW_SLICE_SIX_2, WINDOW_SLICE_SIX_3, WINDOW_SLICE_SIX_4,
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
    pub const BLACK: usize = 0;
    pub const WHITE: usize = 1;

    pub const HORIZONTAL: usize = 0;
    pub const VERTICAL: usize = 1;
    pub const DIAGONAL: usize = 2;
    pub const ANTI_DIAGONAL: usize = 3;
}

#[derive(Default)]
pub struct PlayerState {
    pub captures: usize,
    // Number of free threes the player has on the board
    pub free_three: u8,
    // Index which removes a free three
    pub remove_free_three: Vec<usize>,
    // Index of all of the player rocks
    pub rocks: Vec<usize>,
}

#[derive(Default)]
pub struct CreatedPatterns {
    // Number of free threes to restore
    pub black_free_three: u8,
    pub white_free_three: u8,
    // Index of free three indexes that the move restore
    pub black_free_three_indexes: Vec<usize>,
    pub white_free_three_indexes: Vec<usize>,
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
    pub boards: [[bitvec::array::BitArray<[usize; 6], Lsb0>; 4]; 2],
    pub moves: u16,
    pub black: PlayerState,
    pub white: PlayerState,
    pub all_rocks: Vec<usize>,
    pub patterns_restore: Vec<CreatedPatterns>,
    pub moves_restore: Vec<Vec<usize>>,
}

impl Default for Board {
    fn default() -> Board {
        let mut patterns_restore = vec![];
        patterns_restore.reserve(360);
        let mut moves_restore = vec![];
        moves_restore.reserve(360);
        let mut board = Board {
            boards: [
                [
                    bitarr![0; 361],
                    bitarr![0; 361],
                    bitarr![0; 361],
                    bitarr![0; 361],
                ],
                [
                    bitarr![0; 361],
                    bitarr![0; 361],
                    bitarr![0; 361],
                    bitarr![0; 361],
                ],
            ],
            moves: 0,
            black: PlayerState::default(),
            white: PlayerState::default(),
            all_rocks: vec![],
            patterns_restore,
            moves_restore,
        };
        for player_bitboards in board.boards.iter_mut() {
            for bitboard in player_bitboards {
                bitboard.fill(true);
            }
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
                    if !self.boards[Index::BLACK][Index::HORIZONTAL][col + row * BOARD_SIZE] {
                        format!("{}", col + row * BOARD_SIZE)
                            .white()
                            .on_bright_black()
                    } else if !self.boards[Index::WHITE][Index::HORIZONTAL][col + row * BOARD_SIZE]
                    {
                        format!("{}", col + row * BOARD_SIZE).black().on_white()
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
                    if !self.boards[Index::BLACK][Index::HORIZONTAL][index] {
                        format!("{}", index).white().on_bright_black()
                    } else if !self.boards[Index::WHITE][Index::HORIZONTAL][index] {
                        format!("{}", index).black().on_white()
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
                    if !self.boards[Index::BLACK][Index::VERTICAL][index] {
                        format!("{}", index).white().on_bright_black()
                    } else if !self.boards[Index::WHITE][Index::VERTICAL][index] {
                        format!("{}", index).black().on_white()
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
                    if !self.boards[Index::BLACK][Index::DIAGONAL][index] {
                        format!("{}", index).white().on_bright_black()
                    } else if !self.boards[Index::WHITE][Index::DIAGONAL][index] {
                        format!("{}", index).black().on_white()
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
                    if !self.boards[Index::BLACK][Index::ANTI_DIAGONAL][index] {
                        format!("{}", index).white().on_bright_black()
                    } else if !self.boards[Index::WHITE][Index::ANTI_DIAGONAL][index] {
                        format!("{}", index).black().on_white()
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
        if !self.boards[Index::BLACK][Index::HORIZONTAL][index] {
            Rock::Black
        } else if !self.boards[Index::WHITE][Index::HORIZONTAL][index] {
            Rock::White
        } else {
            Rock::None
        }
    }

    pub fn at(&self, index: usize) -> Rock {
        if !self.boards[Index::BLACK][Index::HORIZONTAL][index] {
            Rock::Black
        } else if !self.boards[Index::WHITE][Index::HORIZONTAL][index] {
            Rock::White
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
    // -- given the current slice around the rock
    pub fn match_dual_pattern(
        &self,
        rock: usize,
        player: Player,
        slices: &[[(usize, usize); 361]; 4],
        pattern_1: &BitSlice,
        pattern_2: &BitSlice,
    ) -> bool {
        let boards: &[BitArray<[usize; 6]>; 4];
        let opponent_boards: &[BitArray<[usize; 6]>; 4];
        if player == Player::Black {
            boards = &self.boards[Index::BLACK];
            opponent_boards = &self.boards[Index::WHITE];
        } else {
            boards = &self.boards[Index::WHITE];
            opponent_boards = &self.boards[Index::BLACK];
        };
        // Iterate on each rocks to know if any of them make a five in a row
        let slice = slices[0][rock];
        if boards[Index::HORIZONTAL][slice.0..=slice.1].eq(pattern_1)
            && opponent_boards[Index::HORIZONTAL][slice.0..=slice.1].eq(pattern_2)
        {
            return true;
        }
        let slice = slices[1][rock];
        if boards[Index::VERTICAL][slice.0..=slice.1].eq(pattern_1)
            && opponent_boards[Index::VERTICAL][slice.0..=slice.1].eq(pattern_2)
        {
            return true;
        }
        let slice = slices[2][rock];
        if boards[Index::DIAGONAL][slice.0..=slice.1].eq(pattern_1)
            && opponent_boards[Index::DIAGONAL][slice.0..=slice.1].eq(pattern_2)
        {
            return true;
        }
        let slice = slices[3][rock];
        if boards[Index::ANTI_DIAGONAL][slice.0..=slice.1].eq(pattern_1)
            && opponent_boards[Index::ANTI_DIAGONAL][slice.0..=slice.1].eq(pattern_2)
        {
            return true;
        }
        false
    }

    // Iterate on each bitboards for the current player to search for the given pattern
    // -- given the current slice around the rock
    // -- and count the number of occurences
    pub fn count_dual_pattern(
        &self,
        rock: usize,
        player: Player,
        slices: &[[(usize, usize); 361]; 4],
        pattern_1: &BitSlice,
        pattern_2: &BitSlice,
    ) -> u8 {
        let boards: &[BitArray<[usize; 6]>; 4];
        let opponent_boards: &[BitArray<[usize; 6]>; 4];
        if player == Player::Black {
            boards = &self.boards[Index::BLACK];
            opponent_boards = &self.boards[Index::WHITE];
        } else {
            boards = &self.boards[Index::WHITE];
            opponent_boards = &self.boards[Index::BLACK];
        };
        let mut total = 0;
        // Iterate on each rocks to know if any of them make a five in a row
        let slice = slices[0][rock];
        if boards[Index::HORIZONTAL][slice.0..=slice.1].eq(pattern_1)
            && opponent_boards[Index::HORIZONTAL][slice.0..=slice.1].eq(pattern_2)
        {
            total += 1;
        }
        let slice = slices[1][rock];
        if boards[Index::VERTICAL][slice.0..=slice.1].eq(pattern_1)
            && opponent_boards[Index::VERTICAL][slice.0..=slice.1].eq(pattern_2)
        {
            total += 1;
        }
        let slice = slices[2][rock];
        if boards[Index::DIAGONAL][slice.0..=slice.1].eq(pattern_1)
            && opponent_boards[Index::DIAGONAL][slice.0..=slice.1].eq(pattern_2)
        {
            total += 1;
        }
        let slice = slices[3][rock];
        if boards[Index::ANTI_DIAGONAL][slice.0..=slice.1].eq(pattern_1)
            && opponent_boards[Index::ANTI_DIAGONAL][slice.0..=slice.1].eq(pattern_2)
        {
            total += 1;
        }
        total
    }

    // Iterate on each bitboards for the current player to search for the given pattern
    // -- given the current slice around the rock
    pub fn match_pattern(
        &self,
        rock: usize,
        player: Player,
        slices: &[[(usize, usize); 361]; 4],
        pattern: &BitSlice,
    ) -> bool {
        let boards = if player == Player::Black {
            &self.boards[Index::BLACK]
        } else {
            &self.boards[Index::WHITE]
        };
        // Iterate on each rocks to know if any of them make a five in a row
        let slice = slices[0][rock];
        if boards[Index::HORIZONTAL][slice.0..=slice.1].eq(pattern) {
            return true;
        }
        let slice = slices[1][rock];
        if boards[Index::VERTICAL][slice.0..=slice.1].eq(pattern) {
            return true;
        }
        let slice = slices[2][rock];
        if boards[Index::DIAGONAL][slice.0..=slice.1].eq(pattern) {
            return true;
        }
        let slice = slices[3][rock];
        if boards[Index::ANTI_DIAGONAL][slice.0..=slice.1].eq(pattern) {
            return true;
        }
        false
    }

    // All open intersections for the current Board
    // -- Empty cases within other pieces
    // TODO Use bitboards
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

    fn get_free_threes_break_indexes(&mut self, movement: &Move) -> Vec<usize> {
        let mut indexes: Vec<usize> = vec![];
        let index = movement.index;
        let boards: &[BitArray<[usize; 6]>; 4];
        let opponent_boards: &[BitArray<[usize; 6]>; 4];
        if movement.player == Player::Black {
            boards = &self.boards[Index::BLACK];
            opponent_boards = &self.boards[Index::WHITE];
        } else {
            boards = &self.boards[Index::WHITE];
            opponent_boards = &self.boards[Index::BLACK];
        };
        // Check the [0 1 1 1 0] pattern match in any sliding window
        // -- and add the border index to the list of breakable free three index
        let patterns = [
            (&WINDOW_SLICE_FIVE_1, bits![1, 1, 0, 0, 1]),
            (&WINDOW_SLICE_FIVE_2, bits![1, 0, 1, 0, 1]),
            (&WINDOW_SLICE_FIVE_3, bits![1, 0, 0, 1, 1]),
        ];
        let opponent_pattern = bits![1, 1, 1, 1, 1];
        for (slices, pattern) in patterns {
            // Horizontal
            let slice = slices[0][index];
            if boards[Index::HORIZONTAL][slice.0..=slice.1].eq(pattern)
                && opponent_boards[Index::HORIZONTAL][slice.0..=slice.1].eq(opponent_pattern)
            {
                indexes.push(slice.0);
                indexes.push(slice.1);
                return indexes;
            }
            // Vertical
            let slice = slices[1][index];
            if boards[Index::VERTICAL][slice.0..=slice.1].eq(pattern)
                && opponent_boards[Index::VERTICAL][slice.0..=slice.1].eq(opponent_pattern)
            {
                indexes.push(VERTICAL_TRANSPOSE_REV[slice.0]);
                indexes.push(VERTICAL_TRANSPOSE_REV[slice.1]);
                return indexes;
            }
            // Diagonal
            let slice = slices[2][index];
            if boards[Index::DIAGONAL][slice.0..=slice.1].eq(pattern)
                && opponent_boards[Index::DIAGONAL][slice.0..=slice.1].eq(opponent_pattern)
            {
                indexes.push(DIAGONAL_TRANSPOSE_REV[slice.0]);
                indexes.push(DIAGONAL_TRANSPOSE_REV[slice.1]);
                return indexes;
            }
            // Anti-diagonal
            let slice = slices[3][index];
            if boards[Index::ANTI_DIAGONAL][slice.0..=slice.1].eq(pattern)
                && opponent_boards[Index::ANTI_DIAGONAL][slice.0..=slice.1].eq(opponent_pattern)
            {
                indexes.push(ANTI_DIAGONAL_TRANSPOSE_REV[slice.0]);
                indexes.push(ANTI_DIAGONAL_TRANSPOSE_REV[slice.1]);
                return indexes;
            }
        }
        // ... same with the [0 1 1 1 0] pattern
        let patterns = [
            // Central bit                 v--v-----v
            (&WINDOW_SLICE_SIX_1, bits![1, 1, 0, 1, 0, 1], 3),
            (&WINDOW_SLICE_SIX_1, bits![1, 1, 1, 0, 0, 1], 2),
            (&WINDOW_SLICE_SIX_2, bits![1, 0, 1, 1, 0, 1], 3),
            (&WINDOW_SLICE_SIX_3, bits![1, 0, 1, 1, 0, 1], 2),
            (&WINDOW_SLICE_SIX_4, bits![1, 0, 0, 1, 1, 1], 3),
            (&WINDOW_SLICE_SIX_4, bits![1, 0, 1, 0, 1, 1], 2),
        ];
        let opponent_pattern = bits![1, 1, 1, 1, 1, 1];
        for (slices, pattern, extract) in patterns {
            // Horizontal
            let slice = slices[0][index];
            if boards[Index::HORIZONTAL][slice.0..=slice.1].eq(pattern)
                && opponent_boards[Index::HORIZONTAL][slice.0..=slice.1].eq(opponent_pattern)
            {
                indexes.push(slice.0);
                indexes.push(slice.0 + extract);
                indexes.push(slice.0 + 5);
                return indexes;
            }
            // Vertical
            let slice = slices[1][index];
            if boards[Index::VERTICAL][slice.0..=slice.1].eq(pattern)
                && opponent_boards[Index::VERTICAL][slice.0..=slice.1].eq(opponent_pattern)
            {
                indexes.push(VERTICAL_TRANSPOSE_REV[slice.0]);
                indexes.push(VERTICAL_TRANSPOSE_REV[slice.0 + extract]);
                indexes.push(VERTICAL_TRANSPOSE_REV[slice.0 + 5]);
                return indexes;
            }
            // Diagonal
            let slice = slices[2][index];
            if boards[Index::DIAGONAL][slice.0..=slice.1].eq(pattern)
                && opponent_boards[Index::DIAGONAL][slice.0..=slice.1].eq(opponent_pattern)
            {
                indexes.push(DIAGONAL_TRANSPOSE_REV[slice.0]);
                indexes.push(DIAGONAL_TRANSPOSE_REV[slice.0 + extract]);
                indexes.push(DIAGONAL_TRANSPOSE_REV[slice.0 + 5]);
                return indexes;
            }
            // Anti-diagonal
            let slice = slices[3][index];
            if boards[Index::ANTI_DIAGONAL][slice.0..=slice.1].eq(pattern)
                && opponent_boards[Index::ANTI_DIAGONAL][slice.0..=slice.1].eq(opponent_pattern)
            {
                indexes.push(ANTI_DIAGONAL_TRANSPOSE_REV[slice.0]);
                indexes.push(ANTI_DIAGONAL_TRANSPOSE_REV[slice.0 + extract]);
                indexes.push(ANTI_DIAGONAL_TRANSPOSE_REV[slice.0 + 5]);
                return indexes;
            }
        }
        indexes
    }

    // Pattern: [0 1 1 1 0] and [0 1 1 0 1 0]
    // Three variations per pattern to create a "sliding window"
    pub fn count_created_free_threes(&self, movement: &Move) -> u8 {
        self.count_dual_pattern(
            movement.index,
            movement.player,
            &WINDOW_SLICE_FIVE_1,
            // Central bit     v
            bits![1, 1, 0, 0, 1],
            bits![1, 1, 1, 1, 1],
        ) + self.count_dual_pattern(
            movement.index,
            movement.player,
            &WINDOW_SLICE_FIVE_2,
            // Central bit        v
            bits![1, 0, 1, 0, 1],
            bits![1, 1, 1, 1, 1],
        ) + self.count_dual_pattern(
            movement.index,
            movement.player,
            &WINDOW_SLICE_FIVE_3,
            // Central bit           v
            bits![1, 0, 0, 1, 1],
            bits![1, 1, 1, 1, 1],
        ) + self.count_dual_pattern(
            movement.index,
            movement.player,
            &WINDOW_SLICE_SIX_1,
            // Central bit     v
            bits![1, 1, 0, 1, 0, 1],
            bits![1, 1, 1, 1, 1, 1],
        ) + self.count_dual_pattern(
            movement.index,
            movement.player,
            &WINDOW_SLICE_SIX_1,
            // Central bit     v
            bits![1, 1, 1, 0, 0, 1],
            bits![1, 1, 1, 1, 1, 1],
        ) + self.count_dual_pattern(
            movement.index,
            movement.player,
            &WINDOW_SLICE_SIX_2,
            // Central bit        v
            bits![1, 0, 1, 1, 0, 1],
            bits![1, 1, 1, 1, 1, 1],
        ) + self.count_dual_pattern(
            movement.index,
            movement.player,
            &WINDOW_SLICE_SIX_3,
            // Central bit           v
            bits![1, 0, 1, 1, 0, 1],
            bits![1, 1, 1, 1, 1, 1],
        ) + self.count_dual_pattern(
            movement.index,
            movement.player,
            &WINDOW_SLICE_SIX_4,
            // Central bit              v
            bits![1, 0, 0, 1, 1, 1],
            bits![1, 1, 1, 1, 1, 1],
        ) + self.count_dual_pattern(
            movement.index,
            movement.player,
            &WINDOW_SLICE_SIX_4,
            // Central bit              v
            bits![1, 0, 1, 0, 1, 1],
            bits![1, 1, 1, 1, 1, 1],
        )
    }

    fn movement_create_double_free_three(&self, movement: &Move) -> bool {
        let created_free_three = self.count_created_free_threes(movement);
        let existing_free_threes = if movement.player == Player::Black {
            self.black.free_three
        } else {
            self.white.free_three
        };
        // Handle moves that break a free three while creating one
        let break_current_free_three = if movement.player == Player::Black {
            self.black.remove_free_three.contains(&movement.index)
        } else {
            self.white.remove_free_three.contains(&movement.index)
        };
        let break_free_three = if break_current_free_three { 1 } else { 0 };
        // Beware of negative usize
        if created_free_three > 0 {
            created_free_three + (existing_free_threes - break_free_three) >= 2
        } else {
            false
        }
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
        // Forbid movements that would create a "double three"
        if rules.no_double_three && self.movement_create_double_free_three(movement) {
            return false;
        }
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

    // TODO Update to use the sliding window 4 slices (two patterns in a loop instead of top/bottom)
    // TODO like in ``movement_create_recursive_capture``
    fn get_movement_captures(&mut self, movement: &Move) -> Vec<usize> {
        // Check all 8 directions on a window of 4
        // -- with the movement rock on the "center" of all directions (star pattern)
        let capture_pattern_self = bits![0, 1, 1, 0];
        let capture_pattern_opponent = bits![1, 0, 0, 1];
        let index = movement.index;
        let mut captures: Vec<usize> = vec![];
        let slices = CAPTURE_SLICES[movement.index];
        let boards: [BitArray<[usize; 6]>; 4];
        let opponent_boards: [BitArray<[usize; 6]>; 4];
        if movement.player == Player::Black {
            boards = self.boards[Index::BLACK];
            opponent_boards = self.boards[Index::WHITE];
        } else {
            boards = self.boards[Index::WHITE];
            opponent_boards = self.boards[Index::BLACK];
        };
        // Left Horizontal
        if boards[Index::HORIZONTAL][slices.0 .0..=index].eq(capture_pattern_self)
            && opponent_boards[Index::HORIZONTAL][slices.0 .0..=index].eq(capture_pattern_opponent)
        {
            captures.push(index - 1);
            captures.push(index - 2);
        }
        // Right Horizontal
        if boards[Index::HORIZONTAL][index..=slices.0 .1].eq(capture_pattern_self)
            && opponent_boards[Index::HORIZONTAL][index..=slices.0 .1].eq(capture_pattern_opponent)
        {
            captures.push(index + 1);
            captures.push(index + 2);
        }
        // Top Vertical
        let transposed_index = VERTICAL_TRANSPOSE[index];
        if boards[Index::VERTICAL][slices.1 .0..=transposed_index].eq(capture_pattern_self)
            && opponent_boards[Index::VERTICAL][slices.1 .0..=transposed_index]
                .eq(capture_pattern_opponent)
        {
            captures.push(VERTICAL_TRANSPOSE_REV[transposed_index - 1]);
            captures.push(VERTICAL_TRANSPOSE_REV[transposed_index - 2]);
        }
        // Bottom Vertical
        if boards[Index::VERTICAL][transposed_index..=slices.1 .1].eq(capture_pattern_self)
            && opponent_boards[Index::VERTICAL][transposed_index..=slices.1 .1]
                .eq(capture_pattern_opponent)
        {
            captures.push(VERTICAL_TRANSPOSE_REV[transposed_index + 1]);
            captures.push(VERTICAL_TRANSPOSE_REV[transposed_index + 2]);
        }
        // Top Diagonal
        let transposed_index = DIAGONAL_TRANSPOSE[index];
        if boards[Index::DIAGONAL][slices.2 .0..=transposed_index].eq(capture_pattern_self)
            && opponent_boards[Index::DIAGONAL][slices.2 .0..=transposed_index]
                .eq(capture_pattern_opponent)
        {
            captures.push(DIAGONAL_TRANSPOSE_REV[transposed_index - 1]);
            captures.push(DIAGONAL_TRANSPOSE_REV[transposed_index - 2]);
        }
        // Bottom Diagonal
        if boards[Index::DIAGONAL][transposed_index..=slices.2 .1].eq(capture_pattern_self)
            && opponent_boards[Index::DIAGONAL][transposed_index..=slices.2 .1]
                .eq(capture_pattern_opponent)
        {
            captures.push(DIAGONAL_TRANSPOSE_REV[transposed_index + 1]);
            captures.push(DIAGONAL_TRANSPOSE_REV[transposed_index + 2]);
        }
        // Top Anti-diagonal
        let transposed_index = ANTI_DIAGONAL_TRANSPOSE[index];
        if boards[Index::ANTI_DIAGONAL][slices.3 .0..=transposed_index].eq(capture_pattern_self)
            && opponent_boards[Index::ANTI_DIAGONAL][slices.3 .0..=transposed_index]
                .eq(capture_pattern_opponent)
        {
            captures.push(ANTI_DIAGONAL_TRANSPOSE_REV[transposed_index - 1]);
            captures.push(ANTI_DIAGONAL_TRANSPOSE_REV[transposed_index - 2]);
        }
        // Bottom Anti-diagonal
        if boards[Index::ANTI_DIAGONAL][transposed_index..=slices.3 .1].eq(capture_pattern_self)
            && opponent_boards[Index::ANTI_DIAGONAL][transposed_index..=slices.3 .1]
                .eq(capture_pattern_opponent)
        {
            captures.push(ANTI_DIAGONAL_TRANSPOSE_REV[transposed_index + 1]);
            captures.push(ANTI_DIAGONAL_TRANSPOSE_REV[transposed_index + 2]);
        }
        captures
    }

    pub fn set_rock(&mut self, index: usize, rock: Rock) {
        if rock == Rock::Black {
            self.boards[Index::BLACK][Index::HORIZONTAL].set(index, false);
            self.boards[Index::BLACK][Index::VERTICAL].set(VERTICAL_TRANSPOSE[index], false);
            self.boards[Index::BLACK][Index::DIAGONAL].set(DIAGONAL_TRANSPOSE[index], false);
            self.boards[Index::BLACK][Index::ANTI_DIAGONAL]
                .set(ANTI_DIAGONAL_TRANSPOSE[index], false);
        } else if rock == Rock::White {
            self.boards[Index::WHITE][Index::HORIZONTAL].set(index, false);
            self.boards[Index::WHITE][Index::VERTICAL].set(VERTICAL_TRANSPOSE[index], false);
            self.boards[Index::WHITE][Index::DIAGONAL].set(DIAGONAL_TRANSPOSE[index], false);
            self.boards[Index::WHITE][Index::ANTI_DIAGONAL]
                .set(ANTI_DIAGONAL_TRANSPOSE[index], false);
        } else {
            self.boards[Index::BLACK][Index::HORIZONTAL].set(index, true);
            self.boards[Index::BLACK][Index::VERTICAL].set(VERTICAL_TRANSPOSE[index], true);
            self.boards[Index::BLACK][Index::DIAGONAL].set(DIAGONAL_TRANSPOSE[index], true);
            self.boards[Index::BLACK][Index::ANTI_DIAGONAL]
                .set(ANTI_DIAGONAL_TRANSPOSE[index], true);
            self.boards[Index::WHITE][Index::HORIZONTAL].set(index, true);
            self.boards[Index::WHITE][Index::VERTICAL].set(VERTICAL_TRANSPOSE[index], true);
            self.boards[Index::WHITE][Index::DIAGONAL].set(DIAGONAL_TRANSPOSE[index], true);
            self.boards[Index::WHITE][Index::ANTI_DIAGONAL]
                .set(ANTI_DIAGONAL_TRANSPOSE[index], true);
        }
    }

    // Apply a movement to the current Board
    pub fn set_move(&mut self, rules: &RuleSet, movement: &Move) {
        // Add free threes
        if rules.no_double_three {
            // Save the current free three state to restore on undo move
            let black_free_three_save = self.black.free_three;
            let white_free_three_save = self.white.free_three;
            let black_remove_free_three_save = self.black.remove_free_three.clone();
            let white_remove_free_three_save = self.white.remove_free_three.clone();
            // Check if the move remove a free three for any player
            // -- and clean free three indexes if a free three was deleted
            if self.black.remove_free_three.contains(&movement.index) {
                self.black.free_three -= 1;
                self.black.remove_free_three.clear();
            }
            if self.white.remove_free_three.contains(&movement.index) {
                self.white.free_three -= 1;
                self.white.remove_free_three.clear();
            }
            // Count the number of created free threes and add that to the total
            let created_free_threes = self.count_created_free_threes(movement);
            // Check the new indexes that break the created free three to remove it if a rock is placed
            if movement.player == Player::Black {
                self.black.free_three += created_free_threes;
                if created_free_threes > 0 {
                    let break_indexes = self.get_free_threes_break_indexes(movement);
                    self.black.remove_free_three = break_indexes;
                }
            } else {
                self.white.free_three += created_free_threes;
                if created_free_threes > 0 {
                    let break_indexes = self.get_free_threes_break_indexes(movement);
                    self.white.remove_free_three = break_indexes;
                }
            }
            // Keep track of what index to restore to keep track of free threes on undo move
            self.patterns_restore.push(CreatedPatterns {
                black_free_three: black_free_three_save,
                white_free_three: white_free_three_save,
                black_free_three_indexes: black_remove_free_three_save,
                white_free_three_indexes: white_remove_free_three_save,
            });
        }
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
        // Remove free threes patterns count
        if rules.no_double_three {
            let created_patterns = self.patterns_restore.pop().unwrap();
            self.black.free_three = created_patterns.black_free_three;
            self.black.remove_free_three = created_patterns.black_free_three_indexes;
            self.white.free_three = created_patterns.white_free_three;
            self.white.remove_free_three = created_patterns.white_free_three_indexes;
        }
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
        let rocks = if player == Player::Black {
            &self.black.rocks
        } else {
            &self.white.rocks
        };
        for rock in rocks {
            if self.match_pattern(*rock, player, &WINDOW_SLICE_FIVE_2, five_in_a_row) {
                return true;
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
