use crate::{
    constants::{
        BOARD_PIECES_USIZE, BOARD_SIZE, BOARD_SIZE_USIZE, DIRECTIONS, OPPOSITE_DIRECTIONS,
    },
    macros::coord,
    patterns::{
        CAPTURE_PATTERN, FIVE_PATTERNS, FREE_THREE_DIRECT_CENTER_PATTERN,
        FREE_THREE_DIRECT_PATTERN, FREE_THREE_SECONDARY_CENTER_PATTERN,
        FREE_THREE_SECONDARY_PATTERN, RECURSIVE_CAPTURE_PATTERN, UNDER_CAPTURE_PATTERNS,
    },
    player::Player,
    rock::{PlayerRock, Rock},
    rules::RuleSet,
};
use colored::Colorize;
use std::{collections::HashSet, fmt};

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct Coordinates {
    pub x: i16,
    pub y: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
                "{} {}x{} ({})",
                "black".white().on_black(),
                self.coordinates.y,
                self.coordinates.x,
                self.coordinates.y + self.coordinates.x * BOARD_SIZE
            )
        } else {
            write!(
                f,
                "{} {}x{} ({})",
                "white".black().on_white(),
                self.coordinates.y,
                self.coordinates.x,
                self.coordinates.y + self.coordinates.x * BOARD_SIZE
            )
        }
    }
}

#[derive(Clone)]
pub struct PlayerState {
    pub captures: u8,
    // Index of all of the player rocks
    pub rocks: HashSet<Coordinates>,
}

impl Default for PlayerState {
    fn default() -> Self {
        let mut rocks = HashSet::new();
        rocks.reserve(BOARD_PIECES_USIZE);
        Self { captures: 0, rocks }
    }
}

#[derive(Clone)]
pub struct Board {
    pub pieces: [[Rock; BOARD_SIZE_USIZE]; BOARD_SIZE_USIZE],
    pub player_pieces: [[[PlayerRock; BOARD_SIZE_USIZE]; BOARD_SIZE_USIZE]; 2],
    // Number of moves executed to reach the current Board state
    pub moves: u16,
    pub black: PlayerState,
    pub white: PlayerState,
    pub all_rocks: HashSet<Coordinates>,
    // Rocks to restore (to undo a capture) when undoing the last move
    pub moves_restore: Vec<Vec<Coordinates>>,
}

impl Default for Board {
    fn default() -> Board {
        let mut moves_restore = vec![];
        moves_restore.reserve(BOARD_PIECES_USIZE);
        let mut all_rocks = HashSet::new();
        all_rocks.reserve(BOARD_PIECES_USIZE);
        Board {
            pieces: [[Rock::None; BOARD_SIZE_USIZE]; BOARD_SIZE_USIZE],
            player_pieces: [[[PlayerRock::None; BOARD_SIZE_USIZE]; BOARD_SIZE_USIZE]; 2],
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

    // Helper function to get a Board case for a player with (x, y) coordinates
    #[inline(always)]
    pub fn get_for_player(&self, x: i16, y: i16, index: usize) -> PlayerRock {
        self.player_pieces[index][y as usize][x as usize]
    }

    #[inline(always)]
    pub fn get_for_player_mut(&mut self, x: i16, y: i16, index: usize) -> &mut PlayerRock {
        &mut self.player_pieces[index][y as usize][x as usize]
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
        pattern: &[(i16, PlayerRock)],
        player: Player,
    ) -> bool {
        for (key, value) in pattern {
            let (check_x, check_y) = (
                coordinates.x + direction.0 * key,
                coordinates.y + direction.1 * key,
            );
            if check_x < 0 || check_x >= BOARD_SIZE || check_y < 0 || check_y >= BOARD_SIZE {
                return false;
            }
            if &self.get_for_player(
                check_x,
                check_y,
                if player == Player::Black { 0 } else { 1 },
            ) != value
            {
                return false;
            }
        }
        true
    }

    // Pattern: [0 1 1 1 0]
    // Since the move rock can be in any 1 position, we need to check all possible patterns:
    // [0 ? 1 1 0], [0 1 ? 1 0], [0 1 1 ? 0]
    pub fn move_create_free_three_direct_pattern(&self, movement: &Move) -> u8 {
        let mut total = 0;

        // Handle the [0 {1} 1 {1} 0] patterns
        for direction in &DIRECTIONS {
            if self.check_pattern(
                &movement.coordinates,
                direction,
                &FREE_THREE_DIRECT_PATTERN,
                movement.player,
            ) {
                total += 1;
            }
        }

        // Handle the [0 1 {1} 1 0] pattern to only count it once for a global direction
        for (left, right) in &OPPOSITE_DIRECTIONS {
            if self.check_pattern(
                &movement.coordinates,
                left,
                &FREE_THREE_DIRECT_CENTER_PATTERN,
                movement.player,
            ) || self.check_pattern(
                &movement.coordinates,
                right,
                &FREE_THREE_DIRECT_CENTER_PATTERN,
                movement.player,
            ) {
                total += 1;
            }
        }

        total
    }

    // Pattern: [0 1 1 0 1 0] and [0 1 0 1 1 0]
    pub fn move_create_free_three_secondary_pattern(&self, movement: &Move) -> u8 {
        let mut total = 0;

        // Handle [0 {1} 1 0 {1} 0]
        for direction in &DIRECTIONS {
            if self.check_pattern(
                &movement.coordinates,
                direction,
                &FREE_THREE_SECONDARY_PATTERN,
                movement.player,
            ) {
                total += 1;
            }
        }

        // Handle the [0 1 {1} 0 1 0] pattern to only count it once for a global direction
        for (left, right) in &OPPOSITE_DIRECTIONS {
            if self.check_pattern(
                &movement.coordinates,
                left,
                &FREE_THREE_SECONDARY_CENTER_PATTERN,
                movement.player,
            ) || self.check_pattern(
                &movement.coordinates,
                right,
                &FREE_THREE_SECONDARY_CENTER_PATTERN,
                movement.player,
            ) {
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
        for direction in &DIRECTIONS {
            if self.check_pattern(
                &movement.coordinates,
                direction,
                RECURSIVE_CAPTURE_PATTERN,
                movement.player,
            ) {
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

    fn check_capture(&mut self, movement: &Move) -> u8 {
        // Check captures in all directions and add them to the list
        let mut captures: Vec<Coordinates> = vec![];
        for direction in &DIRECTIONS {
            if self.check_pattern(
                &movement.coordinates,
                direction,
                CAPTURE_PATTERN,
                movement.player,
            ) {
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
            self.remove_from_boards(&coordinates);
            if movement.player == Player::Black {
                self.black.captures += 1;
                self.white.rocks.remove(&coordinates);
            } else {
                self.white.captures += 1;
                self.black.rocks.remove(&coordinates);
            }
            self.all_rocks.remove(&coordinates);
        }
        let captures_len = captures.len() as u8;
        self.moves_restore.push(captures);
        captures_len
    }

    // Update all boards to update for the given movement
    #[inline(always)]
    pub fn set_on_boards(&mut self, coordinates: &Coordinates, player: Player) {
        if player == Player::Black {
            *self.get_mut(coordinates.x, coordinates.y) = Rock::Black;
            *self.get_for_player_mut(coordinates.x, coordinates.y, 0) = PlayerRock::Player;
            *self.get_for_player_mut(coordinates.x, coordinates.y, 1) = PlayerRock::Opponent;
        } else {
            *self.get_mut(coordinates.x, coordinates.y) = Rock::White;
            *self.get_for_player_mut(coordinates.x, coordinates.y, 0) = PlayerRock::Opponent;
            *self.get_for_player_mut(coordinates.x, coordinates.y, 1) = PlayerRock::Player;
        }
    }

    // Apply a movement to the current Board
    pub fn set_move(&mut self, rules: &RuleSet, movement: &Move) -> u8 {
        let mut captures: u8 = 0;
        self.set_on_boards(&movement.coordinates, movement.player);
        if rules.capture {
            captures = self.check_capture(movement);
        }
        if movement.player == Player::Black {
            self.black.rocks.insert(movement.coordinates);
        } else {
            self.white.rocks.insert(movement.coordinates);
        }
        self.all_rocks.insert(movement.coordinates);
        self.moves += 1;
        captures
    }

    #[inline(always)]
    pub fn remove_from_boards(&mut self, coordinates: &Coordinates) {
        *self.get_mut(coordinates.x, coordinates.y) = Rock::None;
        *self.get_for_player_mut(coordinates.x, coordinates.y, 0) = PlayerRock::None;
        *self.get_for_player_mut(coordinates.x, coordinates.y, 1) = PlayerRock::None;
    }

    pub fn undo_move(&mut self, rules: &RuleSet, movement: &Move) {
        // Restored the captured rocks
        if rules.capture {
            let opponent = movement.player.opponent();
            let rocks = self.moves_restore.pop().unwrap();
            // Decrease capture counter
            if movement.player == Player::Black {
                self.black.captures -= rocks.len() as u8;
            } else {
                self.white.captures -= rocks.len() as u8;
            }
            // Restore the rock index in the opponent list of rocks
            for rock in rocks {
                self.set_on_boards(&rock, opponent);
                if movement.player == Player::Black {
                    self.white.rocks.insert(rock);
                } else {
                    self.black.rocks.insert(rock);
                }
                self.all_rocks.insert(rock);
            }
        }
        // Restore rock
        self.remove_from_boards(&movement.coordinates);
        if movement.player == Player::Black {
            self.black.rocks.remove(&movement.coordinates);
        } else {
            self.white.rocks.remove(&movement.coordinates);
        }
        self.all_rocks.remove(&movement.coordinates);
        self.moves -= 1;
    }

    // Check if a five in a row (as a pattern) is not under capture
    pub fn five_in_a_row_is_under_capture(
        &self,
        rules: &RuleSet,
        coordinates: &Coordinates,
        five_in_a_row_direction: &(i16, i16),
        pattern: &[(i16, PlayerRock)],
        player: Player,
    ) -> bool {
        let opponent = player.opponent();
        // Check if the rock that initiated the five in a row is not under capture...
        // Pattern: [0 {1} {1} 2]
        UNDER_CAPTURE_PATTERNS.iter().enumerate().all(|(index, capture_pattern)| {
            DIRECTIONS.iter().all(|direction| {
                // Check that the pattern *doesn't* match ...
                !self.check_pattern(coordinates, direction, capture_pattern, player)
                // ... or that the move in [0] is illegal for the other player
                || (index == 0 && !self.is_move_legal(
                    rules,
                    &Move {
                        player: opponent,
                        coordinates: coord!(
                            coordinates.x + -direction.0,
                            coordinates.y + -direction.1
                        ),
                    },
                )) || (index == 1 && !self.is_move_legal(
                    rules,
                    &Move {
                        player: opponent,
                        coordinates: coord!(
                            coordinates.x + direction.0 * -2,
                            coordinates.y + direction.1 * -2
                        ),
                    },
                ))
            })
        })
        // ... and check if each other rock in the five in a row are not under capture
        && pattern.iter().all(|(mov, _)| {
            // The checked rock is the another rock in the current five in a row pattern
            let other_rock_coords = coord!(coordinates.x + five_in_a_row_direction.0 * mov, coordinates.y + five_in_a_row_direction.1 * mov);
            // Pattern: [0 {1} {1} 2]
            UNDER_CAPTURE_PATTERNS.iter().enumerate().all(|(index, capture_pattern)| {
                DIRECTIONS.iter().all(|direction| {
                    // Check that the pattern *doesn't* match ...
                    !self.check_pattern(&other_rock_coords, direction, capture_pattern, player)
                    // ... or that the move in [0] is illegal for the other player
                    || (index == 0 && !self.is_move_legal(
                        rules,
                        &Move {
                            player: opponent,
                            coordinates: coord!(
                                other_rock_coords.x + -direction.0,
                                other_rock_coords.y + -direction.1
                            ),
                        },
                    )) || (index == 1 && !self.is_move_legal(
                        rules,
                        &Move {
                            player: opponent,
                            coordinates: coord!(
                                other_rock_coords.x + direction.0 * -2,
                                other_rock_coords.y + direction.1 * -2
                            ),
                        },
                    ))
                })
            })
        })
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
    pub fn has_uncaptured_five_in_a_row(&self, rules: &RuleSet, player: Player) -> bool {
        // We need to check all rocks since a capture movement can unlock a totally unrelated
        // -- captured five in a row thas is now legal and trigger a win
        let rocks = if player == Player::Black {
            &self.black.rocks
        } else {
            &self.white.rocks
        };
        for rock in rocks {
            for five_in_a_row_direction in &DIRECTIONS {
                for pattern in FIVE_PATTERNS {
                    if self.check_pattern(rock, five_in_a_row_direction, pattern, player)
                        && self.five_in_a_row_is_under_capture(
                            rules,
                            rock,
                            five_in_a_row_direction,
                            pattern,
                            player,
                        )
                    {
                        return true;
                    }
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
        for rock in rocks {
            for direction in &DIRECTIONS {
                for pattern in FIVE_PATTERNS {
                    if self.check_pattern(rock, direction, pattern, player) {
                        return true;
                    }
                }
            }
        }
        false
    }

    // Check if the given player is winning with the current board state
    // (Has an unbreakable winning position according to the rules)
    // This function is called *after* a move is made, so the [0] is already on the board
    pub fn is_winning(&self, rules: &RuleSet, player: Player) -> bool {
        if rules.capture
            && ((player == Player::Black && self.black.captures >= 10)
                || (player == Player::White && self.white.captures >= 10))
        {
            return true;
        }
        if rules.game_ending_capture {
            self.has_uncaptured_five_in_a_row(rules, player)
        } else {
            self.has_five_in_a_row(player)
        }
    }
}
