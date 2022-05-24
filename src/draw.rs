use crate::{
    board::{Board, Pawn, BOARD_PIECES, BOARD_SIZE},
    game::{Game, GameMode, Winner},
    player::Player,
    BORDER_OFFSET, BUTTTON_HEIGTH, BUTTTON_LENGHT, GRID_WINDOW_SIZE, PANEL_WINDOW_SIZE,
    SQUARE_SIZE,
};
use macroquad::{
    color::Color,
    color_u8,
    prelude::{
        draw_circle, draw_circle_lines, draw_line, draw_rectangle, draw_rectangle_lines, draw_text,
        measure_text, mouse_position, Vec2, BLACK, BLUE, MAGENTA, RED, WHITE,
    },
    ui::{root_ui, widgets},
};

const TEXT_OFFSET: f32 = 20.;
const FONT_SIZE: u16 = 20;
const WIN_FONT_SIZE: u16 = 30;
const POLICE_SIZE: f32 = 20.;

const BLACK_SEMI: Color = color_u8!(0, 0, 0, 200);
const WHITE_SEMI: Color = color_u8!(255, 255, 255, 200);
const BEIGE_SEMI: Color = color_u8!(212, 176, 130, 255);

pub fn draw_goban(game: &Game) {
    let board = &game.board;

    // Draw lines
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

    // Draw circle
    let mut y = BORDER_OFFSET + 3 * SQUARE_SIZE;
    while y < (17 * SQUARE_SIZE) {
        let mut x = BORDER_OFFSET + 3 * SQUARE_SIZE;
        while x < (17 * SQUARE_SIZE) {
            draw_circle(x as f32, y as f32, 6.0, BLACK);
            x += 6 * SQUARE_SIZE;
        }
        y += 6 * SQUARE_SIZE;
    }

    // Draw movements
    let movements = game
        .board
        .intersections_all_moves(&game.rules, &game.current_player);
    for movement in movements {
        if board.pieces[movement.index] == Pawn::None {
            let (x, y) = Board::index_to_coordinates(movement.index);
            let draw_x = BORDER_OFFSET as f32 + (x * SQUARE_SIZE) as f32;
            let draw_y = BORDER_OFFSET as f32 + (y * SQUARE_SIZE) as f32;
            draw_circle(draw_x, draw_y, 4.0, if movement.legal { BLUE } else { RED });
            draw_circle_lines(draw_x, draw_y, 4., 1., BLACK);
        }
    }

    // Draw rocks
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

pub fn draw_recommended_move(game: &mut Game) {
    let movement = game.computer_recommended_move();
    if let Some(movement) = movement {
        if game.board.pieces[movement.index] == Pawn::None {
            let (x, y) = Board::index_to_coordinates(movement.index);
            let draw_x = BORDER_OFFSET as f32 + (x * SQUARE_SIZE) as f32;
            let draw_y = BORDER_OFFSET as f32 + (y * SQUARE_SIZE) as f32;
            draw_circle(draw_x, draw_y, 4.0, MAGENTA);
            draw_circle_lines(draw_x, draw_y, 4., 1., BLACK);
        }
    }
}

pub fn draw_rock_preview(game: &Game) {
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
                    BLACK_SEMI
                } else {
                    WHITE_SEMI
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
            "Time: {}ms",
            if game.winner != Winner::None {
                0
            } else {
                game.play_time.elapsed().as_millis()
            }
        )
        .as_str(),
        GRID_WINDOW_SIZE as f32 + TEXT_OFFSET,
        TEXT_OFFSET,
        POLICE_SIZE,
        BLACK,
    );
    draw_text(
        format!("Previous: {}ms", game.previous_play_time.as_millis()).as_str(),
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

const WINNER_WINDOW_WIDTH: f32 = GRID_WINDOW_SIZE as f32 / 4.;
const WINNER_WINDOW_HEIGHT: f32 = SQUARE_SIZE as f32 * 2. + 2.;

pub fn display_winner(game: &Game) {
    if game.winner != Winner::None {
        // Background
        draw_rectangle(
            (GRID_WINDOW_SIZE as f32 - WINNER_WINDOW_WIDTH) / 2.,
            GRID_WINDOW_SIZE as f32 - (WINNER_WINDOW_HEIGHT * 2.),
            WINNER_WINDOW_WIDTH,
            WINNER_WINDOW_HEIGHT,
            BEIGE_SEMI,
        );
        draw_rectangle_lines(
            (GRID_WINDOW_SIZE as f32 - WINNER_WINDOW_WIDTH) / 2.,
            GRID_WINDOW_SIZE as f32 - (WINNER_WINDOW_HEIGHT * 2.),
            WINNER_WINDOW_WIDTH,
            WINNER_WINDOW_HEIGHT,
            4.,
            BLACK,
        );
        // Winner text
        let win_text = format!(
            "{} win !",
            if game.winner == Winner::Black {
                "Black"
            } else {
                "White"
            }
        )
        .to_string();
        let text_size = measure_text(&win_text, None, WIN_FONT_SIZE, 1.);
        draw_text(
            &win_text,
            (GRID_WINDOW_SIZE as f32 / 2.) - (text_size.width / 2.),
            GRID_WINDOW_SIZE as f32 - (WINNER_WINDOW_HEIGHT * 2.)
                + ((WINNER_WINDOW_HEIGHT - text_size.height) / 1.4), // Should be 2.0 ...
            WIN_FONT_SIZE as f32,
            if game.winner == Winner::Black {
                BLACK
            } else {
                WHITE
            },
        );
    }
}
