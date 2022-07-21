use crate::rock::PlayerRock;

// * Static patterns

// -- [0 {1} 1 {1} 0]
pub const FREE_THREE_DIRECT_PATTERN: [(i16, PlayerRock); 4] = [
    (-1, PlayerRock::None),
    (1, PlayerRock::Player),
    (2, PlayerRock::Player),
    (3, PlayerRock::None),
];

// -- [0 1 {1} 1 0]
pub const FREE_THREE_DIRECT_CENTER_PATTERN: [(i16, PlayerRock); 4] = [
    (-2, PlayerRock::None),
    (-1, PlayerRock::Player),
    (1, PlayerRock::Player),
    (2, PlayerRock::None),
];

// Missing [0 {1} 1 0 1 0] ?
// -- [0 {1} 0 1 1 0]
pub const FREE_THREE_SECONDARY_PATTERN: [(i16, PlayerRock); 5] = [
    (-1, PlayerRock::None),
    (1, PlayerRock::None),
    (2, PlayerRock::Player),
    (3, PlayerRock::Player),
    (4, PlayerRock::None),
];

// -- [0 1 {1} 0 1 0]
pub const FREE_THREE_SECONDARY_CENTER_PATTERN: [(i16, PlayerRock); 5] = [
    (-2, PlayerRock::None),
    (-1, PlayerRock::Player),
    (1, PlayerRock::None),
    (2, PlayerRock::Player),
    (3, PlayerRock::None),
];

pub const DIRECT_FOUR_PATTERNS: [&[(i16, PlayerRock); 5]; 2] = [
    &[
        (-1, PlayerRock::None),
        (1, PlayerRock::Player),
        (2, PlayerRock::Player),
        (3, PlayerRock::Player),
        (4, PlayerRock::None),
    ],
    &[
        (-2, PlayerRock::None),
        (-1, PlayerRock::Player),
        (1, PlayerRock::Player),
        (2, PlayerRock::Player),
        (3, PlayerRock::None),
    ],
];

pub const SECONDARY_FOUR_PATTERNS: [&[(i16, PlayerRock); 6]; 3] = [
    &[
        (1, PlayerRock::None),
        (2, PlayerRock::Player),
        (3, PlayerRock::Player),
        (4, PlayerRock::Player),
        (5, PlayerRock::None),
        (6, PlayerRock::Player),
    ],
    &[
        (-2, PlayerRock::Player),
        (-1, PlayerRock::None),
        (1, PlayerRock::Player),
        (2, PlayerRock::Player),
        (3, PlayerRock::None),
        (4, PlayerRock::Player),
    ],
    &[
        (-3, PlayerRock::Player),
        (-2, PlayerRock::None),
        (-1, PlayerRock::Player),
        (1, PlayerRock::Player),
        (2, PlayerRock::None),
        (3, PlayerRock::Player),
    ],
];

pub const FIVE_PATTERNS: [&[(i16, PlayerRock); 4]; 3] = [
    &[
        (1, PlayerRock::Player),
        (2, PlayerRock::Player),
        (3, PlayerRock::Player),
        (4, PlayerRock::Player),
    ],
    &[
        (-1, PlayerRock::Player),
        (1, PlayerRock::Player),
        (2, PlayerRock::Player),
        (3, PlayerRock::Player),
    ],
    &[
        (-2, PlayerRock::Player),
        (-1, PlayerRock::Player),
        (1, PlayerRock::Player),
        (2, PlayerRock::Player),
    ],
];

// -- [0, {1}, {1}, 2]
// -+ [2, {1}, {1}, 0]
pub const UNDER_CAPTURE_PATTERNS: [[(i16, PlayerRock); 3]; 2] = [
    [
        (-1, PlayerRock::None),
        (1, PlayerRock::Player),
        (2, PlayerRock::Opponent),
    ],
    [
        (-2, PlayerRock::None),
        (-1, PlayerRock::Player),
        (1, PlayerRock::Opponent),
    ],
];

// 0 is used as the reference with this pattern
// -- [2 {0} 1 2]
// -+ [2  1 {0} 2]
pub const RECURSIVE_CAPTURE_PATTERN: &[(i16, PlayerRock); 3] = &[
    (-1, PlayerRock::Opponent),
    (1, PlayerRock::Player),
    (2, PlayerRock::Opponent),
];

// -- [{1}, 2, 2, {1}]
pub const CAPTURE_PATTERN: &[(i16, PlayerRock); 3] = &[
    (1, PlayerRock::Opponent),
    (2, PlayerRock::Opponent),
    (3, PlayerRock::Player),
];

#[repr(u8)]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Category {
    // Uncaptured five in a row
    FiveInRow,
    // Kills an opponent four in a row
    KillFour,
    // Four in a row that can be upgraded with one or more moves
    OpenFour,
    // Kills an opponent open three in a row (reduce to closed three in a row)
    ReduceThree,
    // Four in a row that can be upgraded on a single side
    CloseFour,
    // Three in a row that can be upgraded to open four with multiple moves
    OpenThree,
    // Kills an opponent closed three in a row (no moves on any sides)
    KillThree,
    // Block a capture
    BlockedCapture,
    // Create a possible capture
    CreateCapture,
    // Create a capture for the opponent
    RevealCapture,
    // Create a captured five in a row
    // Interesting since it might be upgradable with BlockedCapture
    CapturedFiveInRow,
    // Blocked three in a row
    CloseThree,
    // Two in a row
    OpenTwo,
    // Kills an opponent open two (reduce to closed two in a row)
    ReduceTwo,
    // Two in a row that can only be upgraded by a single move
    // CloseTwo,
}

#[derive(Default, Debug, Clone)]
pub struct PatternCount {
    pub five_in_row: u8,
    pub kill_four: u8,
    pub open_four: u8,
    pub reduce_three: u8,
    pub close_four: u8,
    pub open_three: u8,
    pub kill_three: u8,
    pub blocked_capture: u8,
    pub captured_five_in_row: u8,
    pub close_three: u8,
    pub open_two: u8,
    pub reduce_two: u8,
    pub close_two: u8,
    pub created_captures: u8,
    pub reveal_capture: u8,
    pub total_captures: u8,
    pub opponent_captures: u8,
    pub inc_captures: u8,
}

impl PatternCount {
    // Order by which to sort the generated moves
    // Gives priority to moves that save the game or end the game
    pub fn best_pattern(&self) -> u8 {
        (if self.total_captures >= 10 || self.five_in_row > 0 {
            200
        } else if self.kill_four > 0 {
            100
        } else if self.open_four > 0 {
            50
        } else if self.reduce_three > 0 {
            20
        } else if self.close_four > 0 {
            11
        } else if self.open_three > 0 {
            8
        } else if self.kill_three > 0 || self.inc_captures > 0 || self.created_captures > 0 {
            7
        } else if self.blocked_capture > 0 {
            6
        } else if self.captured_five_in_row > 0 {
            5
        } else if self.close_three > 0 {
            4
        } else if self.open_two > 0 {
            3
        } else if self.reduce_two > 0 {
            2
        } else if self.close_two > 0 {
            1
        } else {
            0
        }) + if self.inc_captures > 0 {
            self.inc_captures + self.total_captures
        } else {
            0
        }
    }

    pub fn from_patterns(patterns: &Vec<Category>) -> Self {
        let mut pattern_count = PatternCount::default();
        for &pattern in patterns {
            match pattern {
                Category::FiveInRow => pattern_count.five_in_row += 1,
                Category::KillFour => pattern_count.kill_four += 1,
                Category::OpenFour => pattern_count.open_four += 1,
                Category::ReduceThree => pattern_count.reduce_three += 1,
                Category::CloseFour => pattern_count.close_four += 1,
                Category::OpenThree => pattern_count.open_three += 1,
                Category::KillThree => pattern_count.kill_three += 1,
                Category::BlockedCapture => pattern_count.blocked_capture += 1,
                Category::RevealCapture => pattern_count.reveal_capture += 1,
                Category::CapturedFiveInRow => pattern_count.captured_five_in_row += 1,
                Category::CloseThree => pattern_count.close_three += 1,
                Category::OpenTwo => pattern_count.open_two += 1,
                Category::ReduceTwo => pattern_count.reduce_two += 1,
                Category::CreateCapture => pattern_count.created_captures += 1,
                // Category::CloseTwo => pattern_count.close_two += 1,
            }
        }
        pattern_count
    }
}

lazy_static! {
    pub static ref PATTERNS: Vec<(Vec<(i16, u8)>, Category)> = vec![
        // Only half of some patterns are required since all directions will be checked
        // * FiveInRow
        // -- [{1}, {1}, {1}, 1, 1]
        (vec![(1, 1), (2, 1), (3, 1), (4, 1)], Category::FiveInRow),
        (vec![(-1, 1), (1, 1), (2, 1), (3, 1)], Category::FiveInRow),
        (vec![(-2, 1), (-1, 1), (1, 1), (2, 1)], Category::FiveInRow),
        // * KillFour
        // -- [{1}, 2, 2, 2, 2, 1]
        // -+ [1, 2, 2, 2, 2, {1}]
        (
            vec![(1, 2), (2, 2), (3, 2), (4, 2), (5, 1)],
            Category::KillFour,
        ),
        // -- [2, 2, {1}, 2, 2]
        (vec![(-2, 2), (-1, 2), (1, 2), (2, 2)], Category::KillFour),
        // -- [2, {1}, 2, 2, 2]
        // -+ [2, 2, 2, {1}, 2]
        (vec![(-1, 2), (1, 2), (2, 2), (3, 2)], Category::KillFour),
        // -- [{1}, 2, 2, 2, 2]
        // -+ [2, 2, 2, 2, {1}]
        (vec![(1, 2), (2, 2), (3, 2), (4, 2)], Category::KillFour),
        // * OpenFour
        // -- [0, {1}, {1}, 1, 1, 0]
        (
            vec![(-1, 0), (1, 1), (2, 1), (3, 1), (4, 0)],
            Category::OpenFour,
        ),
        (
            vec![(-2, 0), (-1, 1), (1, 1), (2, 1), (3, 0)],
            Category::OpenFour,
        ),
        // -- [{1}, 0, {1}, {1}, 1, 0, 1]
        (
            vec![(1, 0), (2, 1), (3, 1), (4, 1), (5, 0), (6, 1)],
            Category::OpenFour,
        ),
        (
            vec![(-2, 1), (-1, 0), (1, 1), (2, 1), (3, 0), (4, 1)],
            Category::OpenFour,
        ),
        (
            vec![(-3, 1), (-2, 0), (-1, 1), (1, 1), (2, 0), (3, 1)],
            Category::OpenFour,
        ),
        // * ReduceThree
        // -- [{1}, 2, 2, 2, 0]
        // -+ [0, 2, 2, 2, {1}]
        (
            vec![(1, 2), (2, 2), (3, 2), (4, 0)],
            Category::ReduceThree,
        ),
        // -- [2, 2, {1}, 2, 0]
        // -+ [0, 2, {1}, 2, 2]
        (
            vec![(-2, 2), (-1, 2), (1, 2), (2, 0)],
            Category::ReduceThree,
        ),
        // -- [2, {1}, 2, 2, 0]
        // -+ [0, 2, 2, {1}, 2]
        (
            vec![(-1, 2), (1, 2), (2, 2), (3, 0)],
            Category::ReduceThree,
        ),
        // * CloseFour
        // -- [2, {1}, {1}, {1}, {1}, 0]
        // -+ [0, {1}, {1}, {1}, {1}, 2]
        (
            vec![(-1, 2), (1, 1), (2, 1), (3, 1), (4, 0)],
            Category::CloseFour,
        ),
        (
            vec![(-2, 2), (-1, 1), (1, 1), (2, 1), (3, 0)],
            Category::CloseFour,
        ),
        (
            vec![(-3, 2), (-2, 1), (-1, 1), (1, 1), (2, 0)],
            Category::CloseFour,
        ),
        (
            vec![(-4, 2), (-3, 1), (-2, 1), (-1, 1), (1, 0)],
            Category::CloseFour,
        ),
        // -- [{1}, 0, {1}, {1}, {1}, 2]
        // -+ [2, {1}, {1}, {1}, 0, {1}]
        (
            vec![(1, 0), (2, 1), (3, 1), (4, 1), (5, 2)],
            Category::CloseFour,
        ),
        (
            vec![(-2, 1), (-1, 0), (1, 1), (2, 1), (3, 2)],
            Category::CloseFour,
        ),
        (
            vec![(-3, 1), (-2, 0), (-1, 1), (1, 1), (2, 2)],
            Category::CloseFour,
        ),
        (
            vec![(-4, 1), (-3, 0), (-2, 1), (-1, 1), (1, 2)],
            Category::CloseFour,
        ),
        // -- [2, {1}, {1}, 0, 1, 1, 2]
        // -+ [2, 1, 1, 0, {1}, {1}, 2]
        // (
        //     vec![(-1, 2), (1, 1), (2, 0), (3, 1), (4, 1), (5, 2)],
        //     Category::CloseFour,
        // ),
        // (
        //     vec![(-2, 2), (-1, 1), (1, 0), (2, 1), (3, 1), (4, 2)],
        //     Category::CloseFour,
        // ),
        // * OpenThree
        // -- [0, {1}, {1}, 1, 0]
        (
            vec![(-1, 0), (1,1), (2, 1), (3, 0)],
            Category::OpenThree,
        ),
        (
            vec![(-2, 0), (-1,1), (1, 1), (2, 0)],
            Category::OpenThree,
        ),
        // -- [0, {1}, 0, {1}, {1}, 0]
        // -+ [0, {1}, {1}, 0, {1}, 0]
        (
            vec![(-1, 0), (1,0), (2, 1), (3, 1), (4, 0)],
            Category::OpenThree,
        ),
        (
            vec![(-3, 0), (-2,1), (-1, 0), (1, 1), (2, 0)],
            Category::OpenThree,
        ),
        (
            vec![(-4, 0), (-3,1), (-2, 0), (-1, 1), (1, 0)],
            Category::OpenThree,
        ),
        // * KillThree
        // -- [{1}, 2, 2, 2, 1]
        // -+ [1, 2, 2, 2, {1}]
        (
            vec![(1, 2), (2, 2), (3, 2), (4, 1)],
            Category::KillThree,
        ),
        // -- [{1}, 2, {1}, 2, 2]
        // -+ [2, 2, {1}, 2, {1}]
        (
            vec![(1, 2), (2, 1), (3, 2), (4, 2)],
            Category::KillThree,
        ),
        (
            vec![(-2, 1), (-1, 2), (1, 2), (2, 2)],
            Category::KillThree,
        ),
        // * BlockedCapture
        // -- [{1}, 1, 1, 2]
        // -+ [2, 1, 1, {1}]
        (vec![(1, 1), (2, 1), (3, 2)], Category::BlockedCapture),
        // * CreateCapture
        // -- [{1}, 2, 2, 0]
        // -+ [0, 2, 2, {1}]
        (vec![(1, 2), (2, 2), (3, 0)], Category::CreateCapture),
        // * RevealCapture
        // -- [2, {1}, {1}, 0]
        // -+ [0, {1}, {1}, 2]
        (vec![(-1, 2), (1, 1), (2, 0)], Category::RevealCapture),
        (vec![(-2, 2), (-1, 1), (1, 0)], Category::RevealCapture),
        // * CapturedFiveInRow
        // > Computed after five in a row
        // * CloseThree
        // -- [2, {1}, {1}, {1}, 0, 0]
        // -+ [0, 0, {1}, {1}, {1}, 2]
        (
            vec![(-1, 2), (1, 1), (2, 1), (3, 0), (4, 0)],
            Category::CloseThree,
        ),
        (
            vec![(-2, 2), (-1, 1), (1, 1), (2, 0)],
            Category::CloseThree,
        ),
        (
            vec![(-3, 2), (-2, 1), (-1, 1), (1, 0)],
            Category::CloseThree,
        ),
        // -- [2, {1}, {1}, 0, {1}, 0]
        // -+ [0, 0, {1}, {1}, {1}, 2]
        (
            vec![(-1, 2), (1, 1), (2, 0), (3, 1), (4, 0)],
            Category::CloseThree,
        ),
        (
            vec![(-2, 2), (-1, 1), (1, 0), (2, 1), (3, 0)],
            Category::CloseThree,
        ),
        (
            vec![(-4, 2), (-3, 1), (-2, 1), (-1, 0), (1, 0)],
            Category::CloseThree,
        ),
        // * OpenTwo
        // -- [0, 0, {1}, {1}, 0]
        // -+ [0, {1}, {1}, 0, 0]
        (
            vec![(-2, 0), (-1, 0), (1, 1), (2, 0)],
            Category::OpenTwo,
        ),
        (
            vec![(-3, 0), (-2, 0), (-1, 1), (1, 0)],
            Category::OpenTwo,
        ),
        // -- [0, {1}, 0, 1, 0]
        (
            vec![(-1, 0), (1, 0), (2, 1), (3, 0)],
            Category::OpenTwo,
        ),
        // * ReduceTwo
        // -- [{1}, 2, 2, 0]
        // -+ [0, 2, 2, {1}]
        (
            vec![(1, 2), (2, 2), (3, 0)],
            Category::ReduceTwo,
        ),
        // -- [2, {1}, 2]
        (
            vec![(-1, 2), (1, 2)],
            Category::ReduceTwo,
        ),
        // -- [0, 2, 0, {1}, 2, 0]
        // -+ [0, 2, {1}, 0, 2, 0]
        (
            vec![(-3, 0), (-2, 2), (-1, 0), (1, 2), (2, 0)],
            Category::ReduceTwo,
        ),
        // * CloseTwo
        // -- [2, {1}, {1}, 0, 0]
        // -+ [0, 0, {1}, {1}, 2]
        // (
        //     vec![(-1, 2), (1, 1), (2, 0), (3, 0)],
        //     Category::CloseTwo,
        // ),
        // (
        //     vec![(-2, 2),(-1, 1), (1, 0), (2, 0)],
        //     Category::CloseTwo,
        // ),
        // -- [2, {1}, 0, {1}, 0, 0]
        // -+ [0, 0, {1}, 0, {1}, 2]
        // (
        //     vec![(-1, 2), (1, 0), (2, 1), (3, 0), (4, 0)],
        //     Category::CloseTwo,
        // ),
        // (
        //     vec![(-3, 2), (-2, 1), (-1, 0), (1, 0), (2, 0)],
        //     Category::CloseTwo,
        // ),
    ];
}
