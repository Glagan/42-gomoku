use crate::{
    constants::{
        BOARD_PIECES_USIZE, BOARD_SIZE, BOARD_SIZE_USIZE, DIRECTIONS, OPPOSITE_DIRECTIONS,
    },
    macros::coord,
    player::Player,
    rock::Rock,
    rules::RuleSet,
};
use colored::Colorize;
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

    #[allow(clippy::manual_range_contains)]
    pub fn check_pattern(
        &self,
        coordinates: &Coordinates,
        direction: &(i16, i16),
        pattern: &[(i16, Rock)],
    ) -> bool {
        for (key, value) in pattern {
            let (check_x, check_y) = (
                coordinates.x + direction.0 * key,
                coordinates.y + direction.1 * key,
            );
            if check_x < 0 || check_x >= BOARD_SIZE || check_y < 0 || check_y >= BOARD_SIZE {
                return false;
            }
            if &self.get(check_x, check_y) != value {
                return false;
            }
        }
        true
    }

    #[allow(clippy::manual_range_contains)]
    pub fn count_repeating(
        &self,
        coordinates: &Coordinates,
        direction: &(i16, i16),
        expect: Rock,
    ) -> u8 {
        let mut i: u8 = 1;
        let (mut check_x, mut check_y) = (
            coordinates.x + direction.0 * i as i16,
            coordinates.y + direction.1 * i as i16,
        );
        while check_x >= 0
            && check_x < BOARD_SIZE
            && check_y >= 0
            && check_y < BOARD_SIZE
            && self.get(check_x, check_y) == expect
        {
            i += 1;
            check_x = coordinates.x + direction.0 * i as i16;
            check_y = coordinates.y + direction.1 * i as i16;
        }
        i - 1
    }

    // Pattern: [0 1 1 1 0]
    // Since the move rock can be in any 1 position, we need to check all possible patterns:
    // [0 ? 1 1 0], [0 1 ? 1 0], [0 1 1 ? 0]
    pub fn move_create_free_three_direct_pattern(&self, movement: &Move) -> u8 {
        let self_rock = movement.player.rock();
        let pattern = &[
            (-1, Rock::None),
            (1, self_rock),
            (2, self_rock),
            (3, Rock::None),
        ];

        let mut total = 0;
        for direction in &DIRECTIONS {
            if self.check_pattern(&movement.coordinates, direction, pattern) {
                total += 1;
            }
        }

        total
    }

    // Pattern: [0 1 1 0 1 0] and [0 1 0 1 1 0]
    pub fn move_create_free_three_secondary_pattern(&self, movement: &Move) -> u8 {
        let self_rock = movement.player.rock();
        let pattern = &[
            (-1, Rock::None),
            (1, Rock::None),
            (2, self_rock),
            (3, self_rock),
            (4, Rock::None),
        ];

        let mut total = 0;
        for direction in &DIRECTIONS {
            if self.check_pattern(&movement.coordinates, direction, pattern) {
                total += 1;
            }
        }

        total
    }

    // Pattern: [0 1 1 1 0] and [0 1 1 0 1 0] ([0 1 0 1 1 0] is just *right* and the original is left)
    // For the pattern to be considered a free-three, it strictly need to have both ends "free"
    // -- so borders does *not* count
    fn movement_create_double_free_three(&self, movement: &Move) -> bool {
        self.move_create_free_three_direct_pattern(movement)
            + self.move_create_free_three_secondary_pattern(movement)
            >= 2
    }

    // Pattern: [2 1 0 2] or [2 0 1 2] where [0] is the movement index
    fn movement_create_recursive_capture(&self, movement: &Move) -> bool {
        let player = movement.player;
        let self_rock = player.rock();
        let opponent_rock = self_rock.opponent();

        let pattern = &[(-1, opponent_rock), (1, self_rock), (2, opponent_rock)];
        for direction in &DIRECTIONS {
            if self.check_pattern(&movement.coordinates, direction, pattern) {
                return true;
            }
        }

        false
    }

    // Check if a move *can* be executed according to the rules
    pub fn is_move_legal(&self, rules: &RuleSet, movement: &Move) -> bool {
        // Forbid movements that would create a "double three"
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
        let mut captures: Vec<Coordinates> = vec![];
        let self_rock: Rock;
        let opponent_rock: Rock;
        if movement.player == Player::Black {
            self_rock = Rock::Black;
            opponent_rock = Rock::White;
        } else {
            self_rock = Rock::White;
            opponent_rock = Rock::Black;
        }
        let pattern = &[(1, opponent_rock), (2, opponent_rock), (3, self_rock)];

        // Check captures in all directions and add them to the list
        for direction in &DIRECTIONS {
            if self.check_pattern(&movement.coordinates, direction, pattern) {
                captures.push(coord!(
                    movement.coordinates.x + direction.0,
                    movement.coordinates.y + direction.1
                ));
                captures.push(coord!(
                    movement.coordinates.x + direction.0 * 2,
                    movement.coordinates.y + direction.1 * 2
                ));
            }
        }

        // Process all captues all at once
        // -- Remove rocks from the board and lists
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
        let self_rock = player.rock();
        let opponent_rock = self_rock.opponent();
        let rocks = if player == Player::Black {
            &self.black.rocks
        } else {
            &self.white.rocks
        };
        let pattern = &[(-1, Rock::None), (1, self_rock), (2, opponent_rock)];

        for rock in rocks.iter() {
            for (left, right) in &OPPOSITE_DIRECTIONS {
                if self.count_repeating(rock, left, self_rock)
                    + self.count_repeating(rock, right, self_rock)
                    >= 4
                {
                    // Pattern: [0 1 1 2] where
                    // With the rock possibly in either [1] positions
                    if DIRECTIONS
                        .iter()
                        .all(|direction| !self.check_pattern(rock, direction, pattern))
                    {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn has_five_in_a_row(&self, player: Player) -> bool {
        let self_rock = player.rock();
        let rocks = if player == Player::Black {
            &self.black.rocks
        } else {
            &self.white.rocks
        };

        for rock in rocks.iter() {
            for (left, right) in &OPPOSITE_DIRECTIONS {
                if self.count_repeating(rock, left, self_rock)
                    + self.count_repeating(rock, right, self_rock)
                    >= 4
                {
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
        if rules.game_ending_capture {
            self.has_uncaptured_five_in_a_row(player)
        } else {
            self.has_five_in_a_row(player)
        }
    }
}
