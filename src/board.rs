use crate::{
    computer::{Computer, PatternCategory, PatternCount},
    player::Player,
    rules::RuleSet,
};
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Pawn {
    None,
    Black,
    White,
}

impl ToString for Pawn {
    fn to_string(&self) -> String {
        match self {
            Pawn::None => "0".to_string(),
            Pawn::Black => "1".to_string(),
            Pawn::White => "2".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub player: Player,
    pub index: usize, // Index of the piece to place
}

pub const BOARD_SIZE: usize = 19;
pub const BOARD_PIECES: usize = BOARD_SIZE * BOARD_SIZE;

#[derive(Clone)]
pub struct Board {
    pub pieces: [Pawn; BOARD_PIECES],
    pub moves: Vec<Move>,
    pub black_capture: usize,
    pub white_capture: usize,
}

impl Default for Board {
    fn default() -> Board {
        Board {
            pieces: [Pawn::None; BOARD_PIECES],
            moves: vec![],
            black_capture: 0,
            white_capture: 0,
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..BOARD_SIZE {
            write!(
                f,
                "{}",
                self.pieces[(BOARD_SIZE * row)..(BOARD_SIZE * (row + 1))]
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(&" ")
            )?;
            if row != BOARD_SIZE - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Board {
    // Helper function to get a Board case with (x, y) coordinates
    pub fn get(&self, x: usize, y: usize) -> Option<Pawn> {
        if x >= BOARD_SIZE || y >= BOARD_SIZE {
            return None;
        }
        Some(self.pieces[Board::coordinates_to_index(x, y)].clone())
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
        if self.moves.len() == 0 {
            return vec![((BOARD_SIZE as f64 / 2.) * BOARD_SIZE as f64) as usize];
        }
        let mut intersections: Vec<usize> = vec![];
        for (existing_pawn, _) in self
            .pieces
            .iter()
            .enumerate()
            .filter(|&pawn| pawn.1 != &Pawn::None)
        {
            let (x, y) = Board::index_to_coordinates(existing_pawn);
            // for x_mov in [-2, -1, 0, 1, 2] as [i8; 5] {
            //     for y_mov in [-2, -1, 0, 1, 2] as [i8; 5] {
            for x_mov in [-1, 0, 1] as [i8; 3] {
                for y_mov in [-1, 0, 1] as [i8; 3] {
                    if x_mov == 0 && y_mov == 0 {
                        continue;
                    }
                    if (x < 2 && x_mov == -2)
                        || (x < 1 && x_mov == -1)
                        || (x_mov > 0 && x + x_mov as usize >= BOARD_SIZE)
                    {
                        continue;
                    }
                    if (y < 2 && y_mov == -2)
                        || (y < 1 && y_mov == -1)
                        || (y_mov > 0 && y + y_mov as usize >= BOARD_SIZE)
                    {
                        continue;
                    }
                    let (new_x, new_y) = (
                        usize::try_from(x as i8 + x_mov).ok().unwrap(),
                        usize::try_from(y as i8 + y_mov).ok().unwrap(),
                    );
                    if let Some(pawn) = &self.get(new_x, new_y) {
                        if *pawn == Pawn::None {
                            let index = Board::coordinates_to_index(new_x, new_y);
                            if !intersections.contains(&index) {
                                intersections.push(index);
                            }
                        }
                    }
                }
            }
        }
        intersections
    }

    // Check if a move *can* be executed according to the rules
    // TODO
    pub fn is_move_legal(&self, rules: &RuleSet, movement: &Move) -> bool {
        // Forbid movements that would create a "double three"
        if rules.no_double_three {
            // TODO
        }
        // Forbid movements that would put a pawn in a "capture" state
        if rules.capture {
            // TODO
        }
        true
    }

    // All possible movements from the intersections for a given player
    pub fn intersections_legal_moves(&self, rules: &RuleSet, player: &Player) -> Vec<Move> {
        // Analyze each intersections and check if a Pawn can be set on it
        // -- for the current player according to the rules
        let intersections = self.open_intersections();
        // println!("---\n{}\n--- intersections {:#?}", self, intersections);
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

    // Apply a movement to the current Board
    pub fn set_move(&mut self, rules: &RuleSet, movement: &Move) {
        if rules.capture {
            // TODO capture remove other pawns
            // TODO Return the number of captured pawns to increase the total (if rules.game_ending_capture)
        }
        self.pieces[movement.index] = movement.player.pawn();
        self.moves.push(movement.clone());
    }

    // Apply a movement to a new copy of the current Board
    pub fn apply_move(&self, rules: &RuleSet, movement: &Move) -> Board {
        let mut new_board = self.clone();
        new_board.set_move(rules, movement);
        new_board
    }

    pub fn has_five_in_a_row(&self, player: &Player) -> bool {
        let mut pattern_count = PatternCount::default();
        let patterns = Computer::get_patterns(&self, player);
        for pattern in patterns.iter() {
            if pattern.category == PatternCategory::FiveInRow {
                pattern_count.five_in_row += 1;
            }
        }
        pattern_count.five_in_row > 0
    }

    // Check if the given player is winning on the current board
    // (Has an unbreakable winning position according to the rules)
    pub fn is_winning(&self, rules: &RuleSet, player: &Player) -> bool {
        if rules.game_ending_capture
            && ((player == &Player::Black && self.black_capture >= 10)
                || (player == &Player::White && self.white_capture >= 10))
        {
            return true;
        }
        // TODO if game.capture -> check if five in a row can't be captured
        self.has_five_in_a_row(player)
    }
}
