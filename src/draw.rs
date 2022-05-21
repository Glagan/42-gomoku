use crate::{
    board::{Board, Pawn, BOARD_PIECES, BOARD_SIZE},
    game::{Game, GameMode, Winner},
    player::Player,
    BORDER_OFFSET, BUTTTON_HEIGTH, BUTTTON_LENGHT, GRID_WINDOW_SIZE, PANEL_WINDOW_SIZE,
    SQUARE_SIZE,
};
use macroquad::{
    prelude::{
        draw_circle, draw_line, draw_text, measure_text, mouse_position, Vec2, BLACK, WHITE,
    },
    ui::{root_ui, widgets},
};

const TEXT_OFFSET: f32 = 20.;
const FONT_SIZE: u16 = 20;
const POLICE_SIZE: f32 = 20.;

pub fn draw_goban(game: &Game) {
    let board = &game.board;

    //Draw line
    for i in 0..BOARD_SIZE {
        draw_line(
            (i * SQUARE_SIZE + BORDER_OFFSET) as f32,
            (BORDER_OFFSET - 1) as f32,
            (i * SQUARE_SIZE + BORDER_OFFSET) as f32,
            (GRID_WINDOW_SIZE - BORDER_OFFSET + 1) as f32,
            2.,
            BLACK,
        );
    }
    for i in 0..BOARD_SIZE {
        draw_line(
            (BORDER_OFFSET - 1) as f32,
            (i * SQUARE_SIZE + BORDER_OFFSET) as f32,
            (GRID_WINDOW_SIZE - BORDER_OFFSET + 1) as f32,
            (i * SQUARE_SIZE + BORDER_OFFSET) as f32,
            2.,
            BLACK,
        );
    }
    draw_line(
        (GRID_WINDOW_SIZE + 1) as f32,
        0.,
        (GRID_WINDOW_SIZE + 1) as f32,
        GRID_WINDOW_SIZE as f32,
        4.,
        BLACK,
    );
    //Draw circle
    let mut y = BORDER_OFFSET + 3 * SQUARE_SIZE;
    while y < (17 * SQUARE_SIZE) {
        let mut x = BORDER_OFFSET + 3 * SQUARE_SIZE;
        while x < (17 * SQUARE_SIZE) {
            draw_circle(x as f32, y as f32, 6.0, BLACK);
            x += 6 * SQUARE_SIZE;
        }
        y += 6 * SQUARE_SIZE;
    }

    for i in 0..BOARD_PIECES {
        if board.pieces[i] != Pawn::None {
            let (x, y) = Board::index_to_coordinates(i);
            draw_circle(
                (x * SQUARE_SIZE + BORDER_OFFSET) as f32,
                (y * SQUARE_SIZE + BORDER_OFFSET) as f32,
                20.,
                if board.pieces[i] == Pawn::Black {
                    BLACK
                } else {
                    WHITE
                },
            );
            // Move number on top of the rock
            if let Some(move_number) = game.rock_move.iter().rposition(|&r| r == i) {
                let move_text = format!("{}", move_number + 1).to_string();
                let text_size = measure_text(&move_text, None, FONT_SIZE, 1.);
                draw_text(
                    &move_text,
                    (x * SQUARE_SIZE + BORDER_OFFSET) as f32 - text_size.width / 2.,
                    (y * SQUARE_SIZE + BORDER_OFFSET) as f32 + text_size.height / 2.,
                    POLICE_SIZE,
                    if board.pieces[i] == Pawn::Black {
                        WHITE
                    } else {
                        BLACK
                    },
                );
            }
        }
    }
}

pub fn draw_current_rock(game: &Game) {
    let (mouse_x, mouse_y) = mouse_position();
    if mouse_x < (GRID_WINDOW_SIZE - 2) as f32 && mouse_y < (GRID_WINDOW_SIZE - 2) as f32 {
        let rock_x = (mouse_x - 1.) as usize / SQUARE_SIZE;
        let rock_y = (mouse_y - 1.) as usize / SQUARE_SIZE;
        if game.board.pieces[Board::coordinates_to_index(rock_x, rock_y)] == Pawn::None {
            draw_circle(
                (rock_x * SQUARE_SIZE + BORDER_OFFSET) as f32,
                (rock_y * SQUARE_SIZE + BORDER_OFFSET) as f32,
                20.,
                if game.current_player == Player::Black {
                    BLACK
                } else {
                    WHITE
                },
            );
        }
    }
}

pub fn game_selector(game: &mut Game) {
    let pvp_button = widgets::Button::new("Start PvP game")
        .size(Vec2::new(BUTTTON_LENGHT, BUTTTON_HEIGTH))
        .position(Vec2::new(
            ((GRID_WINDOW_SIZE + PANEL_WINDOW_SIZE) / 2) as f32 - BUTTTON_LENGHT / 2.,
            (GRID_WINDOW_SIZE / 2) as f32 - BUTTTON_HEIGTH / 2. - 100.,
        ))
        .ui(&mut root_ui());

    let pva_button = widgets::Button::new("Start PvA game")
        .size(Vec2::new(BUTTTON_LENGHT, BUTTTON_HEIGTH))
        .position(Vec2::new(
            ((GRID_WINDOW_SIZE + PANEL_WINDOW_SIZE) / 2) as f32 - BUTTTON_LENGHT / 2.,
            (GRID_WINDOW_SIZE / 2) as f32 - BUTTTON_HEIGTH / 2.,
        ))
        .ui(&mut root_ui());

    let ava_button = widgets::Button::new("Start AvA game")
        .size(Vec2::new(BUTTTON_LENGHT, BUTTTON_HEIGTH))
        .position(Vec2::new(
            ((GRID_WINDOW_SIZE + PANEL_WINDOW_SIZE) / 2) as f32 - BUTTTON_LENGHT / 2.,
            (GRID_WINDOW_SIZE / 2) as f32 - BUTTTON_HEIGTH / 2. + 100.,
        ))
        .ui(&mut root_ui());

    if pvp_button || pva_button || ava_button {
        game.start(if pvp_button {
            GameMode::PvP
        } else if pva_button {
            GameMode::PvA
        } else {
            GameMode::AvA
        });
    }
}

pub fn display_panel_text(game: &mut Game) {
    draw_text(
        format!(
            "Elapsed time: {:.2}",
            if game.winner != Winner::None {
                0.
            } else {
                game.play_time.elapsed().as_secs_f32()
            }
        )
        .as_str(),
        GRID_WINDOW_SIZE as f32 + TEXT_OFFSET,
        TEXT_OFFSET,
        POLICE_SIZE,
        BLACK,
    );
    draw_text(
        format!("Elapsed time: {:.2}", game.previous_play_time.as_secs_f32()).as_str(),
        GRID_WINDOW_SIZE as f32 + TEXT_OFFSET,
        TEXT_OFFSET * 2.,
        POLICE_SIZE,
        BLACK,
    );

    draw_text(
        format!("Black capture: {}", game.board.black_capture).as_str(),
        GRID_WINDOW_SIZE as f32 + TEXT_OFFSET,
        TEXT_OFFSET * 3.,
        POLICE_SIZE,
        BLACK,
    );
    draw_text(
        format!("White capture: {}", game.board.white_capture).as_str(),
        GRID_WINDOW_SIZE as f32 + TEXT_OFFSET,
        TEXT_OFFSET * 4.,
        POLICE_SIZE,
        BLACK,
    );

    draw_text(
        format!(
            "Player: {}",
            if game.current_player == Player::Black {
                "Black"
            } else {
                "White"
            }
        )
        .as_str(),
        GRID_WINDOW_SIZE as f32 + TEXT_OFFSET,
        TEXT_OFFSET * 5.,
        POLICE_SIZE,
        BLACK,
    );
    let surrender_button = widgets::Button::new(if game.mode == GameMode::AvA {
        "Back"
    } else {
        if game.winner != Winner::None {
            "Back"
        } else {
            "Surrender"
        }
    })
    .size(Vec2::new(BUTTTON_LENGHT - 30., BUTTTON_HEIGTH - 30.))
    .position(Vec2::new(
        (GRID_WINDOW_SIZE + PANEL_WINDOW_SIZE / 2) as f32 - (BUTTTON_LENGHT - 30.) / 2.,
        GRID_WINDOW_SIZE as f32 - 70.,
    ))
    .ui(&mut root_ui());

    if surrender_button {
        game.playing = false;
    }
}
