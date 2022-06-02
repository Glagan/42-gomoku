use crate::{
    constants::{BOARD_PIECES_USIZE, BOARD_SIZE, BOARD_SIZE_USIZE, DIRECTIONS},
    macros::coord,
    pattern::Finder,
    player::Player,
    rock::Rock,
    rules::RuleSet,
};
use colored::Colorize;
use fixed_vec_deque::FixedVecDeque;
use std::fmt;

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy)]
pub struct Coordinates {
    pub x: i16,
    pub y: i16,
}

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub player: Player,
    pub coordinates: Coordinates, // Index of the piece to place
}

#[derive(Debug, Clone, Copy)]
pub struct PossibleMove {
    pub coordinates: Coordinates,
    pub legal: bool,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.player == Player::Black {
            write!(
                f,
                "{} {}x{}",
                "black".white().on_black(),
                self.coordinates.x,
                self.coordinates.y,
            )
        } else {
            write!(
                f,
                "{} {}x{}",
                "white".black().on_white(),
                self.coordinates.x,
                self.coordinates.y,
            )
        }
    }
}

#[derive(Clone)]
pub struct PlayerState {
    pub captures: usize,
    // Index of all of the player rocks
    pub rocks: Vec<Coordinates>,
}

impl Default for PlayerState {
    fn default() -> Self {
        let mut rocks = vec![];
        rocks.reserve(BOARD_PIECES_USIZE);
        Self { captures: 0, rocks }
    }
}

#[derive(Clone)]
pub struct Board {
    pub pieces: [[Rock; BOARD_SIZE_USIZE]; BOARD_SIZE_USIZE],
    // Number of moves executed to reach the current Board state
    pub moves: u16,
    pub black: PlayerState,
    pub white: PlayerState,
    pub all_rocks: Vec<Coordinates>,
    // Rocks to restore (to undo a capture) when undoing the last move
    pub moves_restore: Vec<Vec<Coordinates>>,
}

impl Default for Board {
    fn default() -> Board {
        let mut moves_restore = vec![];
        moves_restore.reserve(BOARD_PIECES_USIZE);
        let mut all_rocks = vec![];
        all_rocks.reserve(BOARD_PIECES_USIZE);
        Board {
            pieces: [[Rock::None; BOARD_SIZE_USIZE]; BOARD_SIZE_USIZE],
            moves: 0,
            black: PlayerState::default(),
            white: PlayerState::default(),
            all_rocks,
            moves_restore,
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (x_index, row) in self.pieces.iter().enumerate() {
            write!(
                f,
                "{}",
                row.iter()
                    .enumerate()
                    .map(|(y_index, p)| format!(
                        "{: >3}",
                        if p == &Rock::Black {
                            format!("{}", x_index + y_index * BOARD_SIZE_USIZE)
                                .white()
                                .on_bright_black()
                        } else if p == &Rock::White {
                            format!("{}", x_index + y_index * BOARD_SIZE_USIZE)
                                .black()
                                .on_white()
                        } else {
                            format!("{}", x_index + y_index * BOARD_SIZE_USIZE).dimmed()
                        }
                    ))
                    .collect::<Vec<String>>()
                    .join(" ")
            )?;
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Board {
    // Helper function to get a Board case with (x, y) coordinates
    #[inline(always)]
    pub fn get(&self, x: i16, y: i16) -> Rock {
        self.pieces[y as usize][x as usize]
    }

    #[inline(always)]
    pub fn get_mut(&mut self, x: i16, y: i16) -> &mut Rock {
        &mut self.pieces[y as usize][x as usize]
    }

    // All open intersections for the current Board
    // -- Empty cases within other pieces
    pub fn open_intersections(&self) -> Vec<Coordinates> {
        // Only the center intersection is available if there is no previous moves
        if self.moves == 0 {
            return vec![coord!(BOARD_SIZE / 2, BOARD_SIZE / 2)];
        }
        let mut intersections: Vec<Coordinates> = vec![];
        for existing_rock in self.all_rocks.iter() {
            for (mov_x, mov_y) in DIRECTIONS {
                let new_coords = coord!(existing_rock.x + mov_x, existing_rock.y + mov_y);
                // Check Board boundaries
                if new_coords.x >= 0
                    && new_coords.y >= 0
                    && new_coords.x < BOARD_SIZE
                    && new_coords.y < BOARD_SIZE
                {
                    let rock = self.get(new_coords.x, new_coords.y);
                    if rock == Rock::None && !intersections.contains(&new_coords) {
                        intersections.push(new_coords);
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
        let self_rock = player.rock();
        let no_rock = Rock::None;
        let (x, y) = (movement.coordinates.x, movement.coordinates.y);

        // Horizontal
        let mut total = 0;
        if (x > 0
            && x < BOARD_SIZE - 3
            && self.get(x - 1, y) == no_rock
            && self.get(x + 1, y) == self_rock
            && self.get(x + 2, y) == self_rock
            && self.get(x + 3, y) == no_rock)
            || (x > 1
                && x < BOARD_SIZE - 2
                && self.get(x - 2, y) == no_rock
                && self.get(x - 1, y) == self_rock
                && self.get(x + 1, y) == self_rock
                && self.get(x + 2, y) == no_rock)
            || (x > 2
                && x < BOARD_SIZE - 1
                && self.get(x - 3, y) == no_rock
                && self.get(x - 2, y) == self_rock
                && self.get(x - 1, y) == self_rock
                && self.get(x + 1, y) == no_rock)
        {
            total += 1;
        }

        // Vertical
        if (y > 0
            && y < BOARD_SIZE - 3
            && self.get(x, y - 1) == no_rock
            && self.get(x, y + 1) == self_rock
            && self.get(x, y + 2) == self_rock
            && self.get(x, y + 3) == no_rock)
            || (y > 1
                && y < BOARD_SIZE - 2
                && self.get(x, y - 2) == no_rock
                && self.get(x, y - 1) == self_rock
                && self.get(x, y + 1) == self_rock
                && self.get(x, y + 2) == no_rock)
            || (y > 2
                && y < BOARD_SIZE - 1
                && self.get(x, y - 3) == no_rock
                && self.get(x, y - 2) == self_rock
                && self.get(x, y - 1) == self_rock
                && self.get(x, y + 1) == no_rock)
        {
            total += 1;
        }

        // Left Diagonal
        if (x > 0
            && x < BOARD_SIZE - 3
            && y > 0
            && y < BOARD_SIZE - 3
            && self.get(x - 1, y - 1) == no_rock
            && self.get(x + 1, y + 1) == self_rock
            && self.get(x + 2, y + 2) == self_rock
            && self.get(x + 3, y + 3) == no_rock)
            || (x > 1
                && x < BOARD_SIZE - 2
                && y > 1
                && y < BOARD_SIZE - 2
                && self.get(x - 2, y - 2) == no_rock
                && self.get(x - 1, y - 1) == self_rock
                && self.get(x + 1, y + 1) == self_rock
                && self.get(x + 2, y + 2) == no_rock)
            || (x > 2
                && x < BOARD_SIZE - 1
                && y > 2
                && y < BOARD_SIZE - 1
                && self.get(x - 3, y - 3) == no_rock
                && self.get(x - 2, y - 2) == self_rock
                && self.get(x - 1, y - 1) == self_rock
                && self.get(x + 1, y + 1) == no_rock)
        {
            total += 1;
        }

        // Right Diagonal
        if (x > 2
            && x < BOARD_SIZE - 1
            && y > 0
            && y < BOARD_SIZE - 3
            && self.get(x + 1, y - 1) == no_rock
            && self.get(x - 1, y + 1) == self_rock
            && self.get(x - 2, y + 2) == self_rock
            && self.get(x - 3, y + 3) == no_rock)
            || (x > 1
                && x < BOARD_SIZE - 2
                && y > 1
                && y < BOARD_SIZE - 2
                && self.get(x + 2, y - 2) == no_rock
                && self.get(x + 1, y - 1) == self_rock
                && self.get(x - 1, y + 1) == self_rock
                && self.get(x - 2, y + 2) == no_rock)
            || (x > 0
                && x < BOARD_SIZE - 3
                && y > 2
                && y < BOARD_SIZE - 1
                && self.get(x + 3, y - 3) == no_rock
                && self.get(x + 2, y - 2) == self_rock
                && self.get(x + 1, y - 1) == self_rock
                && self.get(x - 1, y + 1) == no_rock)
        {
            total += 1;
        }

        total
    }

    // Pattern: [0 1 1 0 1 0] and [0 1 0 1 1 0]
    pub fn move_create_free_three_secondary_pattern(&self, movement: &Move) -> u8 {
        let player = movement.player;
        let player_rock = player.rock();
        let (x, y) = (movement.coordinates.x, movement.coordinates.y);
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
                let new_coords = coord!(x + mov_x, y + mov_y);
                // Check Board boundaries
                if new_coords.x >= 0
                    && new_coords.y >= 0
                    && new_coords.x < BOARD_SIZE
                    && new_coords.y < BOARD_SIZE
                {
                    // 1 for player rock and 0 for anything else
                    *buf.push_back() = if new_coords.x == x && new_coords.y == y
                        || self.get(new_coords.x, new_coords.y) == player_rock
                    {
                        1
                    } else {
                        Finder::rock_to_pattern_rock(self, &new_coords, player)
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

    fn movement_create_double_free_three(&self, movement: &Move) -> bool {
        self.move_create_free_three(movement) >= 2
    }

    // Pattern: [2 1 0 2] or [2 0 1 2] where [0] is the movement index
    fn movement_create_recursive_capture(&self, movement: &Move) -> bool {
        let player = movement.player;
        let (x, y) = (movement.coordinates.x, movement.coordinates.y);
        let self_rock = player.rock();
        let other_rock = self_rock.opponent();

        // Left
        if (x > 1
            && x < BOARD_SIZE - 1
            && self.get(x - 1, y) == self_rock
            && self.get(x - 2, y) == other_rock
            && self.get(x + 1, y) == other_rock)
            // Right
            || (x > 0
                && x < BOARD_SIZE - 2
                && self.get(x - 1, y) == other_rock
                && self.get(x + 1, y) == self_rock
                && self.get(x + 2, y) == other_rock)
            // Top
            || (y > 1
                && y < BOARD_SIZE - 1
                && self.get(x, y - 1) == self_rock
                && self.get(x, y - 2) == other_rock
                && self.get(x, y + 1) == other_rock)
            // Bottom
            || (y > 0
                && y < BOARD_SIZE - 2
                && self.get(x, y - 1) == other_rock
                && self.get(x, y + 1) == self_rock
                && self.get(x, y + 2) == other_rock)
            // Top-Left
            || (x > 1
                && y > 1
                && x < BOARD_SIZE - 1
                && y < BOARD_SIZE - 1
                && self.get(x - 1, y - 1) == self_rock
                && self.get(x - 2, y - 2) == other_rock
                && self.get(x + 1, y + 1) == other_rock)
            // Top-Right
            || (x > 0
                && y > 1
                && x < BOARD_SIZE - 2
                && y < BOARD_SIZE - 1
                && self.get(x + 1, y - 1) == self_rock
                && self.get(x + 2, y - 2) == other_rock
                && self.get(x - 1, y + 1) == other_rock)
            // Bottom-Left
            || (x > 1
                && y > 0
                && x < BOARD_SIZE - 1
                && y < BOARD_SIZE - 2
                && self.get(x - 1, y + 1) == self_rock
                && self.get(x - 2, y + 2) == other_rock
                && self.get(x + 1, y - 1) == other_rock)
            // Bottom-Right
            || (x > 0
                && y > 0
                && x < BOARD_SIZE - 2
                && y < BOARD_SIZE - 2
                && self.get(x + 1, y + 1) == self_rock
                && self.get(x + 2, y + 2) == other_rock
                && self.get(x - 1, y - 1) == other_rock)
        {
            return true;
        }

        false
    }

    // Check if a move *can* be executed according to the rules
    pub fn is_move_legal(&self, rules: &RuleSet, movement: &Move) -> bool {
        // Forbid movements that would create a "double three"
        // Pattern: [1 1 1 0 >< 0 1 1 1] where [><] means any direction change
        if rules.no_double_three && self.movement_create_double_free_three(movement) {
            return false;
        }
        // Forbid movements that would put a rock in a "recursive capture" state
        if rules.capture && self.movement_create_recursive_capture(movement) {
            return false;
        }
        true
    }

    // All *legal* possible movements from the intersections for a given player
    // TODO self.open_intersections().iter().filter(|index| self.is_move_index_legal(rule, index, player));
    pub fn intersections_legal_moves(&self, rules: &RuleSet, player: Player) -> Vec<Move> {
        // Analyze each intersections and check if a Rock can be set on it
        // -- for the current player according to the rules
        let intersections = self.open_intersections();
        let mut moves: Vec<Move> = vec![];
        for coordinates in intersections.iter() {
            let movement = Move {
                player,
                coordinates: *coordinates,
            };
            if self.is_move_legal(rules, &movement) {
                moves.push(movement);
            }
        }
        moves
    }

    // All possible movements from the intersections for a given player
    pub fn intersections_all_moves(&self, rules: &RuleSet, player: Player) -> Vec<PossibleMove> {
        // Analyze each intersections and check if a Rock can be set on it
        // -- for the current player according to the rules
        let intersections = self.open_intersections();
        let mut moves: Vec<PossibleMove> = vec![];
        for coordinates in intersections.iter() {
            let movement = Move {
                player,
                coordinates: *coordinates,
            };
            moves.push(PossibleMove {
                legal: self.is_move_legal(rules, &movement),
                coordinates: *coordinates,
            });
        }
        moves
    }

    fn check_capture(&mut self, movement: &Move) {
        let player_rock: Rock;
        let opponent_rock: Rock;
        let (x, y) = (movement.coordinates.x, movement.coordinates.y);
        let mut captures: Vec<Coordinates> = vec![];

        if movement.player == Player::Black {
            player_rock = Rock::Black;
            opponent_rock = Rock::White;
        } else {
            player_rock = Rock::White;
            opponent_rock = Rock::Black;
        }

        if x >= 3
            && self.get(x - 1, y) == opponent_rock
            && self.get(x - 2, y) == opponent_rock
            && self.get(x - 3, y) == player_rock
        {
            captures.push(coord!(x - 1, y));
            captures.push(coord!(x - 2, y));
        }
        if x + 3 < BOARD_SIZE
            && self.get(x + 1, y) == opponent_rock
            && self.get(x + 2, y) == opponent_rock
            && self.get(x + 3, y) == player_rock
        {
            captures.push(coord!(x + 1, y));
            captures.push(coord!(x + 2, y));
        }
        if y >= 3
            && self.get(x, y - 1) == opponent_rock
            && self.get(x, y - 2) == opponent_rock
            && self.get(x, y - 3) == player_rock
        {
            captures.push(coord!(x, y - 1));
            captures.push(coord!(x, y - 2));
        }
        if y + 3 < BOARD_SIZE
            && self.get(x, y + 1) == opponent_rock
            && self.get(x, y + 2) == opponent_rock
            && self.get(x, y + 3) == player_rock
        {
            captures.push(coord!(x, y + 1));
            captures.push(coord!(x, y + 2));
        }
        if y >= 3
            && x >= 3
            && self.get(x - 1, y - 1) == opponent_rock
            && self.get(x - 2, y - 2) == opponent_rock
            && self.get(x - 3, y - 3) == player_rock
        {
            captures.push(coord!(x - 1, y - 1));
            captures.push(coord!(x - 2, y - 2));
        }
        if y + 3 < BOARD_SIZE
            && x >= 3
            && self.get(x - 1, y + 1) == opponent_rock
            && self.get(x - 2, y + 2) == opponent_rock
            && self.get(x - 3, y + 3) == player_rock
        {
            captures.push(coord!(x - 1, y + 1));
            captures.push(coord!(x - 2, y + 2));
        }
        if y + 3 < BOARD_SIZE
            && x + 3 < BOARD_SIZE
            && self.get(x + 1, y + 1) == opponent_rock
            && self.get(x + 2, y + 2) == opponent_rock
            && self.get(x + 3, y + 3) == player_rock
        {
            captures.push(coord!(x + 1, y + 1));
            captures.push(coord!(x + 2, y + 2));
        }
        if y >= 3
            && x + 3 < BOARD_SIZE
            && self.get(x + 1, y - 1) == opponent_rock
            && self.get(x + 2, y - 2) == opponent_rock
            && self.get(x + 3, y - 3) == player_rock
        {
            captures.push(coord!(x + 1, y - 1));
            captures.push(coord!(x + 2, y - 2));
        }

        for &coordinates in &captures {
            *self.get_mut(coordinates.x, coordinates.y) = Rock::None;
            if movement.player == Player::Black {
                self.black.captures += 1;
                self.white.rocks.swap_remove(
                    self.white
                        .rocks
                        .iter()
                        .position(|&rock| rock == coordinates)
                        .unwrap(),
                );
            } else {
                self.white.captures += 1;
                self.black.rocks.swap_remove(
                    self.black
                        .rocks
                        .iter()
                        .position(|&rock| rock == coordinates)
                        .unwrap(),
                );
            }
            self.all_rocks.swap_remove(
                self.all_rocks
                    .iter()
                    .position(|&rock| rock == coordinates)
                    .unwrap(),
            );
        }
        self.moves_restore.push(captures);
    }

    // Apply a movement to the current Board
    pub fn set_move(&mut self, rules: &RuleSet, movement: &Move) {
        *self.get_mut(movement.coordinates.x, movement.coordinates.y) = movement.player.rock();
        if rules.capture {
            self.check_capture(movement);
        }
        if movement.player == Player::Black {
            self.black.rocks.push(movement.coordinates);
        } else {
            self.white.rocks.push(movement.coordinates);
        }
        self.all_rocks.push(movement.coordinates);
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
                *self.get_mut(rock.x, rock.y) = opponent_rock;
            }
        }
        // Restore rock
        *self.get_mut(movement.coordinates.x, movement.coordinates.y) = Rock::None;
        if movement.player == Player::Black {
            self.black.rocks.swap_remove(
                self.black
                    .rocks
                    .iter()
                    .position(|&rock| rock == movement.coordinates)
                    .unwrap(),
            );
        } else {
            self.white.rocks.swap_remove(
                self.white
                    .rocks
                    .iter()
                    .position(|&rock| rock == movement.coordinates)
                    .unwrap(),
            );
        }
        self.all_rocks.swap_remove(
            self.all_rocks
                .iter()
                .position(|&rock| rock == movement.coordinates)
                .unwrap(),
        );
        self.moves -= 1;
    }

    // Pattern: [0 1 1 2] where
    // With the rock possibly in either [1] positions
    fn rock_can_be_captured(&self, coordinates: &Coordinates) -> bool {
        let (x, y) = (coordinates.x, coordinates.y);
        let self_rock = self.get(x, y);
        let no_rock = Rock::None;
        let other_rock = self_rock.opponent();

        // Horizontal
        if (x > 0
            && x < BOARD_SIZE - 2
            && ((self.get(x - 1, y) == no_rock
                && self.get(x + 1, y) == self_rock
                && self.get(x + 2, y) == other_rock)
                || (self.get(x - 1, y) == other_rock
                    && self.get(x + 1, y) == self_rock
                    && self.get(x + 2, y) == no_rock)))
            || (x > 1
                && x < BOARD_SIZE - 1
                && ((self.get(x - 2, y) == no_rock
                    && self.get(x - 1, y) == self_rock
                    && self.get(x + 1, y) == other_rock)
                    || (self.get(x - 2, y) == other_rock
                        && self.get(x - 1, y) == self_rock
                        && self.get(x + 1, y) == no_rock))) ||

        // Vertical
          (y > 0
            && y < BOARD_SIZE - 2
            && ((self.get(x, y - 1) == no_rock
                && self.get(x, y + 1) == self_rock
                && self.get(x, y + 2) == other_rock)
                || (self.get(x, y - 1) == other_rock
                    && self.get(x, y + 1) == self_rock
                    && self.get(x, y + 2) == no_rock)))
            || (y > 1
                && y < BOARD_SIZE - 1
                && ((self.get(x, y - 2) == no_rock
                    && self.get(x, y - 1) == self_rock
                    && self.get(x, y + 1) == other_rock)
                    || (self.get(x, y - 2) == other_rock
                        && self.get(x, y - 1) == self_rock
                        && self.get(x, y + 1) == no_rock))) ||
                        // Left Diagonal
                         (x > 0
                            && x < BOARD_SIZE - 2
                            && y > 0
                            && y < BOARD_SIZE - 2
                            && ((self.get(x - 1, y - 1) == no_rock
                                && self.get(x + 1, y + 1) == self_rock
                                && self.get(x + 2, y + 2) == other_rock)
                                || (self.get(x - 1, y - 1) == other_rock
                                    && self.get(x + 1, y + 1) == self_rock
                                    && self.get(x + 2, y + 2) == no_rock)))
                            || (x > 1
                                && x < BOARD_SIZE - 1
                                && y > 1
                                && y < BOARD_SIZE - 1
                                && ((self.get(x - 2, y - 2) == no_rock
                                    && self.get(x - 1, y - 1) == self_rock
                                    && self.get(x + 1, y + 1) == other_rock)
                                    || (self.get(x - 2, y - 2) == other_rock
                                        && self.get(x - 1, y - 1) == self_rock
                                        && self.get(x + 1, y + 1) == no_rock)))||
                                        // Right Diagonal
                                         (x > 1
                                            && x < BOARD_SIZE - 1
                                            && y > 0
                                            && y < BOARD_SIZE - 2
                                            && ((self.get(x + 1, y - 1) == no_rock
                                                && self.get(x - 1, y + 1) == self_rock
                                                && self.get(x - 2, y + 2) == other_rock)
                                                || (self.get(x + 1, y - 1) == other_rock
                                                    && self.get(x - 1, y + 1) == self_rock
                                                    && self.get(x - 2, y + 2) == no_rock)))
                                            || (x > 0
                                                && x < BOARD_SIZE - 2
                                                && y > 1
                                                && y < BOARD_SIZE - 1
                                                && ((self.get(x + 2, y - 2) == no_rock
                                                    && self.get(x + 1, y - 1) == self_rock
                                                    && self.get(x - 1, y + 1) == other_rock)
                                                    || (self.get(x + 2, y - 2) == other_rock
                                                        && self.get(x + 1, y - 1) == self_rock
                                                        && self.get(x - 1, y + 1) == no_rock)))
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
        let mut index_buf = FixedVecDeque::<[Coordinates; 5]>::new();
        for rock in rocks.iter() {
            let (x, y) = (rock.x, rock.y);
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
                    let new_coords = coord!(x + mov_x, y + mov_y);
                    // Check Board boundaries
                    if new_coords.x >= 0
                        && new_coords.y >= 0
                        && new_coords.x < BOARD_SIZE
                        && new_coords.y < BOARD_SIZE
                    {
                        // 1 for player rock and 0 for anything else
                        *buf.push_back() = Finder::rock_to_pattern_rock(self, &new_coords, player);
                        *index_buf.push_back() = new_coords;
                        length += 1;
                        if length >= 5
                            && buf == [1, 1, 1, 1, 1]
                            && index_buf
                                .iter()
                                .all(|&coords| !self.rock_can_be_captured(&coords))
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
        let rocks = if player == Player::Black {
            &self.black.rocks
        } else {
            &self.white.rocks
        };
        for rock in rocks.iter() {
            let (x, y) = (rock.x, rock.y);
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
                    let new_coords = coord!(x + mov_x, y + mov_y);
                    // Check Board boundaries
                    if new_coords.x >= 0
                        && new_coords.y >= 0
                        && (new_coords.x) < BOARD_SIZE
                        && (new_coords.y) < BOARD_SIZE
                    {
                        // 1 for player rock and 0 for anything else
                        *buf.push_back() = Finder::rock_to_pattern_rock(self, &new_coords, player);
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
    pub fn is_winning(&self, rules: &RuleSet, player: Player) -> bool {
        if rules.capture
            && ((player == Player::Black && self.black.captures >= 10)
                || (player == Player::White && self.white.captures >= 10))
        {
            return true;
        }
        if rules.game_ending_capture {
            self.has_uncaptured_five_in_a_row(player)
        } else {
            self.has_five_in_a_row(player)
        }
    }
}
