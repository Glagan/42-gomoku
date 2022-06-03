use macroquad::{color_u8, prelude::Color};

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

// Interface
pub const GRID_WINDOW_SIZE: i16 = 800;
pub const PANEL_WINDOW_SIZE: i16 = 200;
pub const BORDER_OFFSET: i16 = 22;
pub const SQUARE_SIZE: i16 = 42;
pub const BUTTTON_HEIGTH: f32 = 70.;
pub const BUTTTON_LENGTH: f32 = 200.;
pub const TEXT_OFFSET: f32 = 20.;
pub const FONT_SIZE: u16 = 20;
pub const WIN_FONT_SIZE: u16 = 30;
pub const POLICE_SIZE: f32 = 20.;
pub const DEPTH: usize = 4;

// Colors
pub const BLACK_SEMI: Color = color_u8!(0, 0, 0, 200);
pub const WHITE_SEMI: Color = color_u8!(255, 255, 255, 200);
pub const BEIGE_SEMI: Color = color_u8!(212, 176, 130, 255);
