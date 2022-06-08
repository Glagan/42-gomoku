// Board
pub const BOARD_SIZE: i16 = 19;
pub const BOARD_SIZE_USIZE: usize = BOARD_SIZE as usize;
pub const BOARD_PIECES_USIZE: usize = BOARD_SIZE_USIZE * BOARD_SIZE_USIZE;
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
pub const OPPOSITE_DIRECTIONS: [((i16, i16), (i16, i16)); 4] = [
    ((-1, -1), (1, 1)),
    ((-1, 0), (1, 0)),
    ((-1, 1), (1, -1)),
    ((0, -1), (0, 1)),
];
pub const DEPTH: usize = 4;
