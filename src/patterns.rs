use crate::rock::PlayerRock;

// * Static patterns

pub const FREE_THREE_DIRECT_PATTERN: [(i16, PlayerRock); 4] = [
    (-1, PlayerRock::None),
    (1, PlayerRock::Player),
    (2, PlayerRock::Player),
    (3, PlayerRock::None),
];

pub const FREE_THREE_DIRECT_CENTER_PATTERN: [(i16, PlayerRock); 4] = [
    (-2, PlayerRock::None),
    (-1, PlayerRock::Player),
    (1, PlayerRock::Player),
    (2, PlayerRock::None),
];

pub const FREE_THREE_SECONDARY_PATTERN: [(i16, PlayerRock); 5] = [
    (-1, PlayerRock::None),
    (1, PlayerRock::None),
    (2, PlayerRock::Player),
    (3, PlayerRock::Player),
    (4, PlayerRock::None),
];

pub const FREE_THREE_SECONDARY_CENTER_PATTERN: [(i16, PlayerRock); 5] = [
    (-2, PlayerRock::None),
    (-1, PlayerRock::Player),
    (1, PlayerRock::None),
    (2, PlayerRock::Player),
    (3, PlayerRock::None),
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

pub const UNDER_CAPTURE_PATTERNS: [[(i16, PlayerRock); 3]; 2] = [
    [
        (-1, PlayerRock::None),
        (1, PlayerRock::Player),
        (2, PlayerRock::Opponent),
    ],
    [
        (-1, PlayerRock::Opponent),
        (1, PlayerRock::Player),
        (2, PlayerRock::None),
    ],
];

pub const RECURSIVE_CAPTURE_PATTERN: &[(i16, PlayerRock); 3] = &[
    (-1, PlayerRock::Opponent),
    (1, PlayerRock::Player),
    (2, PlayerRock::Opponent),
];

pub const CAPTURE_PATTERN: &[(i16, PlayerRock); 3] = &[
    (1, PlayerRock::Opponent),
    (2, PlayerRock::Opponent),
    (3, PlayerRock::Player),
];

#[repr(u8)]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Category {
    FiveInRow,
    CapturedFiveInRow,
    KilledFive,
    OpenFour,
    KilledFour,
    DeadFour,
    KilledThree,
    BlockedCapture,
    OpenThree,
    CutThree,
    DeadThree,
    OpenTwo,
    DeadTwo,
}

#[derive(Default, Debug, Clone)]
pub struct PatternCount {
    pub five_in_row: u8,
    pub captured_five_in_row: u8,
    pub killed_five: u8,
    pub live_four: u8,
    pub killed_four: u8,
    pub dead_four: u8,
    pub killed_three: u8,
    pub blocked_capture: u8,
    pub live_three: u8,
    pub cut_three: u8,
    pub dead_three: u8,
    pub live_two: u8,
    pub dead_two: u8,
    pub captures: u8,
}

impl PatternCount {
    // Order by which to sort the generated moves
    // Gives priority to moves that save the game or end the game
    pub fn best_pattern(&self) -> u8 {
        if self.five_in_row > 0 {
            13
        } else if self.killed_five > 0 {
            12
        } else if self.live_four > 0 {
            11
        } else if self.killed_four > 0 {
            10
        } else if self.killed_three > 0 {
            9
        } else if self.blocked_capture > 0 {
            8
        } else if self.dead_four > 0 {
            7
        } else if self.live_three > 0 {
            6
        } else if self.cut_three > 0 {
            5
        } else if self.captured_five_in_row > 0 {
            4
        } else if self.dead_three > 0 {
            3
        } else if self.live_two > 0 {
            2
        } else if self.dead_two > 0 {
            1
        } else {
            0
        }
    }

    pub fn from_patterns(patterns: &Vec<Category>) -> Self {
        let mut pattern_count = PatternCount::default();
        for &pattern in patterns {
            if pattern == Category::FiveInRow {
                pattern_count.five_in_row += 1;
            } else if pattern == Category::KilledFive {
                pattern_count.killed_five += 1;
            } else if pattern == Category::OpenFour {
                pattern_count.live_four += 1;
            } else if pattern == Category::KilledFour {
                pattern_count.killed_four += 1;
            } else if pattern == Category::DeadFour {
                pattern_count.dead_four += 1;
            } else if pattern == Category::BlockedCapture {
                pattern_count.blocked_capture += 1;
            } else if pattern == Category::KilledThree {
                pattern_count.killed_three += 1;
            } else if pattern == Category::CutThree {
                pattern_count.cut_three += 1;
            } else if pattern == Category::OpenThree {
                pattern_count.live_three += 1;
            } else if pattern == Category::CapturedFiveInRow {
                pattern_count.captured_five_in_row += 1;
            } else if pattern == Category::DeadThree {
                pattern_count.dead_three += 1;
            } else if pattern == Category::OpenTwo {
                pattern_count.live_two += 1;
            } else {
                pattern_count.dead_two += 1;
            }
        }
        pattern_count
    }
}

lazy_static! {
    pub static ref PATTERNS: Vec<(Vec<(i16, u8)>, Category)> = vec![
        // Five in a row
        // Only half of the patterns are required since it will check all directions
        // -- [1, 1, 1, 1, 1]
        (vec![(1, 1), (2, 1), (3, 1), (4, 1)], Category::FiveInRow),
        (vec![(-1, 1), (1, 1), (2, 1), (3, 1)], Category::FiveInRow),
        (vec![(-2, 1), (-1, 1), (1, 1), (2, 1)], Category::FiveInRow),
        // -- [1, 2, 2, 2, 2, 1]
        (
            vec![(1, 2), (2, 2), (3, 2), (4, 2), (5, 1)],
            Category::KilledFive,
        ),
        (
            vec![(-5, 1), (-4, 2), (-3, 2), (-2, 2), (-1, 2)],
            Category::KilledFive,
        ),
        // -- [2, 1, 2, 2, 2, 1, 2]
        (
            vec![(-1, 2), (1, 2), (2, 2), (3, 2), (4, 1), (5, 2)],
            Category::KilledFive,
        ),
        // -- [2, 1, 2, 2, 2]
        // -- [2, 2, 2, 1, 2]
        (vec![(-1, 2), (1, 2), (2, 2), (3, 2)], Category::KilledFive),
        // -- [2, 2, 1, 2, 2]
        (vec![(-2, 2), (-1, 2), (1, 2), (2, 2)], Category::KilledFive),
        // -- [1, 2, 2, 2, 2]
        (vec![(1, 2), (2, 2), (3, 2), (4, 2)], Category::KilledFive),
        // -- [1, 2, 2, 2, 1]
        (vec![(1, 2), (2, 2), (3, 2), (4, 1)], Category::KilledFour),
        // -- [1, 2, 2, 2]
        // -- [2, 2, 2, 1]
        (vec![(1, 2), (2, 2), (3, 2)], Category::KilledFour),
        // -- [2, 1, 2, 2]
        // -- [2, 2, 1, 2]
        (vec![(-1, 2), (1, 2), (2, 2)], Category::KilledFour),
        (vec![(-2, 2), (-1, 2), (1, 2)], Category::KilledFour),
        // -- [0, 1, 1, 0, 1, 1]
        (
            vec![(-1, 0), (1, 1), (2, 0), (3, 1), (4, 1)],
            Category::OpenFour,
        ),
        (
            vec![(-2, 0), (-1, 1), (1, 0), (2, 1), (3, 1)],
            Category::OpenFour,
        ),
        (
            vec![(-4, 0), (-3, 1), (-2, 1), (-1, 0), (1, 1), (2, 1)],
            Category::OpenFour,
        ),
        // -- [1, 1, 0, 1, 1, 0]
        (
            vec![(1, 1), (2, 0), (3, 1), (4, 1), (5, 0)],
            Category::OpenFour,
        ),
        (
            vec![(-1, 1), (1, 0), (2, 1), (3, 1), (4, 0)],
            Category::OpenFour,
        ),
        (
            vec![(-3, 1), (-2, 1), (-1, 0), (1, 1), (2, 0)],
            Category::OpenFour,
        ),
        (
            vec![(-4, 1), (-3, 1), (-2, 0), (-1, 1), (1, 0)],
            Category::OpenFour,
        ),
        // -- [0, 1, 1, 1, 1]
        // -- [1, 1, 1, 1, 0]
        (vec![(-1, 0), (1, 1), (2, 1), (3, 1)], Category::OpenFour),
        (vec![(-2, 0), (-1, 1), (1, 1), (2, 1)], Category::OpenFour),
        // -- [2, 1, 1, 1, 1, 0]
        // -- [0, 1, 1, 1, 1, 2]
        (
            vec![(-1, 2), (1, 1), (2, 1), (3, 1), (4, 0)],
            Category::OpenFour,
        ),
        (
            vec![(-2, 2), (-1, 1), (1, 1), (2, 1), (3, 0)],
            Category::OpenFour,
        ),
        (
            vec![(-3, 2), (-2, 1), (-1, 1), (1, 1), (2, 0)],
            Category::OpenFour,
        ),
        // -- [1, 0, 1, 1, 1]
        // -- [1, 1, 1, 0, 1]
        (vec![(1, 0), (2, 1), (3, 1), (4, 1)], Category::OpenFour),
        (vec![(-2, 1), (-1, 0), (1, 1), (2, 1)], Category::OpenFour),
        // -- [1, 1, 0, 1, 1]
        (vec![(1, 1), (2, 0), (3, 1), (4, 1)], Category::OpenFour),
        (vec![(-1, 1), (1, 0), (2, 1), (3, 1)], Category::OpenFour),
        // -- [2, 0, 1, 1, 1, 0, 2]
        (
            vec![(-2, 2), (-1, 0), (1, 1), (2, 1), (3, 0), (4, 2)],
            Category::OpenThree,
        ),
        (
            vec![(-3, 2), (-2, 0), (-1, 1), (1, 1), (2, 0), (3, 2)],
            Category::OpenThree,
        ),
        // -- [2, 1, 1, 1]
        // -- [1, 1, 1, 2]
        (vec![(1, 1), (2, 1), (3, 2)], Category::BlockedCapture),
        // -- [0, 1, 1, 1, 0]
        (vec![(-1, 0), (1, 1), (2, 1), (3, 0)], Category::OpenThree),
        (vec![(-2, 0), (-1, 1), (1, 1), (2, 0)], Category::OpenThree),
        // -- [1, 1, 1]
        (vec![(1, 1), (2, 1)], Category::OpenThree),
        (vec![(-1, 1), (1, 1)], Category::OpenThree),
        // -- [1, 0, 1, 0, 1]
        (vec![(1, 0), (2, 1), (3, 0), (4, 1)], Category::OpenThree),
        (vec![(-2, 1), (-1, 0), (1, 0), (2, 1)], Category::OpenThree),
        // -- [1, 0, 1, 1]
        // -- [1, 1, 0, 1]
        (vec![(1, 0), (2, 1), (3, 1)], Category::OpenThree),
        (vec![(-2, 1), (-1, 0), (1, 1)], Category::OpenThree),
        // -- [1, 0, 0, 1, 1]
        // -- [1, 1, 0, 0, 1]
        (vec![(1, 0), (2, 0), (3, 1), (4, 1)], Category::OpenThree),
        (vec![(-3, 1), (-2, 0), (-1, 0), (1, 1)], Category::OpenThree),
        // -- [1, 0, 1, 1, 2]
        // -- [2, 1, 1, 0, 1]
        (vec![(1, 0), (2, 1), (3, 1), (4, 2)], Category::DeadThree),
        (vec![(-2, 1), (-1, 0), (1, 1), (2, 2)], Category::DeadThree),
        (vec![(-3, 1), (-2, 0), (-1, 1), (1, 2)], Category::DeadThree),
        // -- [2, 1, 0, 1, 1]
        (vec![(-1, 2), (1, 0), (2, 1), (3, 1)], Category::DeadThree),
        (vec![(-3, 2), (-2, 1), (-1, 0), (1, 1)], Category::DeadThree),
        (
            vec![(-4, 2), (-3, 1), (-2, 0), (-1, 1)],
            Category::DeadThree,
        ),
        // -- [1, 1, 0, 1, 2, 0]
        (
            vec![(1, 1), (2, 0), (3, 1), (4, 2), (5, 0)],
            Category::DeadThree,
        ),
        (
            vec![(-1, 1), (1, 0), (2, 1), (3, 2), (4, 0)],
            Category::DeadThree,
        ),
        (
            vec![(-3, 1), (-2, 1), (-1, 0), (1, 2), (2, 0)],
            Category::DeadThree,
        ),
        // -- [0, 1, 1, 1, 2]
        // -- [2, 1, 1, 1, 0]
        (vec![(-1, 0), (1, 1), (2, 1), (3, 2)], Category::DeadThree),
        (vec![(-2, 0), (-1, 1), (1, 1), (2, 2)], Category::DeadThree),
        (vec![(-3, 0), (-2, 1), (-1, 1), (1, 2)], Category::DeadThree),
        // -- [1, 0, 0, 1, 2]
        // -- [2, 1, 0, 0, 1]
        (vec![(1, 0), (2, 0), (3, 1), (4, 2)], Category::OpenTwo),
        (vec![(-1, 2), (1, 0), (2, 0), (3, 1)], Category::OpenTwo),
        // -- [2, 1, 0, 1]
        // -- [1, 0, 1, 2]
        (vec![(-1, 2), (1, 0), (2, 1)], Category::OpenTwo),
        (vec![(1, 0), (2, 1), (3, 2)], Category::OpenTwo),
        // -- [1, 0, 0, 1]
        (vec![(1, 0), (2, 0), (3, 1)], Category::OpenTwo),
        // -- [0, 1, 1, 0]
        (vec![(-1, 0), (1, 1), (2, 0)], Category::OpenTwo),
        // -- [1, 0, 0, 0, 1]
        (vec![(1, 0), (2, 0), (3, 0), (4, 1)], Category::DeadTwo),
        // -- [1, 0, 1]
        (vec![(1, 0), (2, 1)], Category::OpenTwo),
        // -- [1, 0, 0, 0, 1]
        (vec![(1, 0), (2, 0), (3, 0), (4, 1)], Category::DeadTwo),
        // -- [0, 1, 1, 2]
        // -- [2, 1, 1, 0]
        (vec![(-1, 0), (1, 1), (2, 1)], Category::DeadTwo),
        (vec![(-1, 2), (1, 1), (2, 0)], Category::DeadTwo),
        // -- [2, 1, 1]
        // -- [1, 1, 2]
        (vec![(-1, 2), (1, 1)], Category::DeadTwo),
        (vec![(-2, 2), (-1, 1)], Category::DeadTwo),
        // -- [1, 1]
        (vec![(1, 1)], Category::DeadTwo),
    ];
}
