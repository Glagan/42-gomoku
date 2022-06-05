use crate::{
    constants::{
        BEIGE_SEMI, BLACK_SEMI, BOARD_SIZE, BORDER_OFFSET, BUTTTON_HEIGTH, BUTTTON_LENGTH,
        FONT_SIZE, GRID_WINDOW_SIZE, PANEL_WINDOW_SIZE, POLICE_SIZE, SQUARE_SIZE, TEXT_OFFSET,
        WHITE_SEMI, WIN_FONT_SIZE,
    },
    game::{Game, GameMode, Winner},
    macros::coord,
    player::Player,
    rock::Rock,
};
use macroquad::{
    hash,
    prelude::{
        draw_circle, draw_circle_lines, draw_line, draw_rectangle, draw_rectangle_lines, draw_text,
        measure_text, mouse_position, Color, Vec2, BLACK, BLUE, MAGENTA, RED, WHITE,
    },
    ui::{root_ui, widgets},
};

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

    // Draw circles
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
        .intersections_all_moves(&game.rules, game.current_player);
    for movement in movements {
        if board.get(movement.coordinates.x, movement.coordinates.y) == Rock::None {
            let (x, y) = (movement.coordinates.x, movement.coordinates.y);
            let draw_x = BORDER_OFFSET as f32 + (x * SQUARE_SIZE) as f32;
            let draw_y = BORDER_OFFSET as f32 + (y * SQUARE_SIZE) as f32;
            draw_circle(draw_x, draw_y, 4.0, if movement.legal { BLUE } else { RED });
            draw_circle_lines(draw_x, draw_y, 4., 1., BLACK);
        }
    }

    // Draw rocks
    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            if board.get(x, y) != Rock::None {
                let coordinates = coord!(x, y);
                draw_circle(
                    (x * SQUARE_SIZE + BORDER_OFFSET) as f32,
                    (y * SQUARE_SIZE + BORDER_OFFSET) as f32,
                    20.,
                    if board.get(x, y) == Rock::Black {
                        BLACK
                    } else {
                        WHITE
                    },
                );
                // Move number on top of the rock
                if let Some(move_number) = game.rock_move.iter().rposition(|&r| r == coordinates) {
                    let move_text = format!("{}", move_number + 1).to_string();
                    let text_size = measure_text(&move_text, None, FONT_SIZE, 1.);
                    draw_text(
                        &move_text,
                        (x * SQUARE_SIZE + BORDER_OFFSET) as f32 - text_size.width / 2.,
                        (y * SQUARE_SIZE + BORDER_OFFSET) as f32 + text_size.height / 2.,
                        POLICE_SIZE,
                        if board.get(x, y) == Rock::Black {
                            WHITE
                        } else {
                            BLACK
                        },
                    );
                    // Add highlight for last move
                    if move_number == game.rock_move.len() - 1 {
                        draw_circle_lines(
                            (x * SQUARE_SIZE + BORDER_OFFSET) as f32,
                            (y * SQUARE_SIZE + BORDER_OFFSET) as f32,
                            20.,
                            2.,
                            BLUE,
                        );
                    }
                }
            }
        }
    }

    // Draw computer expected movements
    let mut black = Color::new(0.0, 0.0, 0.0, 0.7);
    let mut white = Color::new(1.0, 1.0, 1.0, 0.7);
    // let mut highlight = Color::new(0.78, 0.48, 1.00, 0.7); // PURPLE
    for (next, movement) in game.computer_expected_moves.iter().skip(1).enumerate() {
        if board.get(movement.coordinates.x, movement.coordinates.y) == Rock::None {
            let (x, y) = (movement.coordinates.x, movement.coordinates.y);
            let draw_x = (x * SQUARE_SIZE + BORDER_OFFSET) as f32;
            let draw_y = (y * SQUARE_SIZE + BORDER_OFFSET) as f32;
            draw_circle(
                draw_x,
                draw_y,
                20.,
                if movement.player == Player::Black {
                    black
                } else {
                    white
                },
            );
            let next_text = format!("{}", game.rock_move.len() + next + 1).to_string();
            let text_size = measure_text(&next_text, None, FONT_SIZE, 1.);
            draw_text(
                &next_text,
                draw_x - (text_size.width / 2.),
                draw_y + text_size.height + 6.,
                POLICE_SIZE,
                if movement.player == Player::Black {
                    white
                } else {
                    black
                },
            );
            // draw_circle_lines(draw_x, draw_y, 20., 1., highlight);
            black.a -= 0.2;
            white.a -= 0.2;
            // highlight.a -= 0.1;
        }
    }
}

pub fn draw_recommended_move(game: &mut Game) {
    let movement = game.computer_recommended_move();
    if let Some(movement) = movement {
        if game
            .board
            .get(movement.coordinates.x, movement.coordinates.y)
            == Rock::None
        {
            let (x, y) = (movement.coordinates.x, movement.coordinates.y);
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
        let rock_x = (mouse_x - 1.) as i16 / SQUARE_SIZE;
        let rock_y = (mouse_y - 1.) as i16 / SQUARE_SIZE;
        if game.board.get(rock_x, rock_y) == Rock::None {
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

pub fn options_selector(game: &mut Game) {
    let ui = &mut root_ui();
    ui.checkbox(hash!(), "Enable Capture", &mut game.rules.capture);
    ui.checkbox(
        hash!(),
        "Enable game ending capture",
        &mut game.rules.game_ending_capture,
    );
    ui.checkbox(
        hash!(),
        "Disallow double free three",
        &mut game.rules.no_double_three,
    );
    ui.checkbox(
        hash!(),
        "Generate recommended moves",
        &mut game.generate_recommended_move,
    );

    let back_button = widgets::Button::new("Back")
        .size(Vec2::new(BUTTTON_LENGTH - 30., BUTTTON_HEIGTH - 30.))
        .position(Vec2::new(
            (GRID_WINDOW_SIZE + PANEL_WINDOW_SIZE / 2) as f32 - (BUTTTON_LENGTH - 30.) / 2.,
            GRID_WINDOW_SIZE as f32 - 70.,
        ))
        .ui(ui);
    if back_button {
        game.in_options = false;
    }
}

pub fn game_selector(game: &mut Game) -> bool {
    let options_button = widgets::Button::new("Options")
        .size(Vec2::new(BUTTTON_LENGTH - 30., BUTTTON_HEIGTH - 30.))
        .position(Vec2::new(
            (GRID_WINDOW_SIZE + PANEL_WINDOW_SIZE / 2) as f32 - (BUTTTON_LENGTH - 30.) / 2.,
            GRID_WINDOW_SIZE as f32 - 70.,
        ))
        .ui(&mut root_ui());
    if options_button {
        game.in_options = true;
        return false;
    }

    let pvp_button = widgets::Button::new("Start PvP game")
        .size(Vec2::new(BUTTTON_LENGTH, BUTTTON_HEIGTH))
        .position(Vec2::new(
            ((GRID_WINDOW_SIZE + PANEL_WINDOW_SIZE) / 2) as f32 - BUTTTON_LENGTH / 2.,
            (GRID_WINDOW_SIZE / 2) as f32 - BUTTTON_HEIGTH / 2. - 100.,
        ))
        .ui(&mut root_ui());

    let pva_button = widgets::Button::new("Start PvA game")
        .size(Vec2::new(BUTTTON_LENGTH, BUTTTON_HEIGTH))
        .position(Vec2::new(
            ((GRID_WINDOW_SIZE + PANEL_WINDOW_SIZE) / 2) as f32 - BUTTTON_LENGTH / 2.,
            (GRID_WINDOW_SIZE / 2) as f32 - BUTTTON_HEIGTH / 2.,
        ))
        .ui(&mut root_ui());

    let ava_button = widgets::Button::new("Start AvA game")
        .size(Vec2::new(BUTTTON_LENGTH, BUTTTON_HEIGTH))
        .position(Vec2::new(
            ((GRID_WINDOW_SIZE + PANEL_WINDOW_SIZE) / 2) as f32 - BUTTTON_LENGTH / 2.,
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
        true
    } else {
        false
    }
}

pub fn color_selector(game: &mut Game) -> bool {
    let back_button = widgets::Button::new("Back")
        .size(Vec2::new(BUTTTON_LENGTH - 30., BUTTTON_HEIGTH - 30.))
        .position(Vec2::new(
            (GRID_WINDOW_SIZE + PANEL_WINDOW_SIZE / 2) as f32 - (BUTTTON_LENGTH - 30.) / 2.,
            GRID_WINDOW_SIZE as f32 - 70.,
        ))
        .ui(&mut root_ui());
    if back_button {
        game.reset();
        return false;
    }

    draw_text(
        "Play as",
        ((GRID_WINDOW_SIZE + PANEL_WINDOW_SIZE) / 2) as f32 - BUTTTON_LENGTH - BUTTTON_LENGTH / 3.,
        (GRID_WINDOW_SIZE / 2) as f32 - BUTTTON_HEIGTH + BUTTTON_HEIGTH / 3.,
        POLICE_SIZE,
        BLACK,
    );

    let black_button = widgets::Button::new("Black")
        .size(Vec2::new(BUTTTON_LENGTH, BUTTTON_HEIGTH))
        .position(Vec2::new(
            ((GRID_WINDOW_SIZE + PANEL_WINDOW_SIZE) / 2) as f32
                - BUTTTON_LENGTH
                - BUTTTON_LENGTH / 3.,
            (GRID_WINDOW_SIZE / 2) as f32 - BUTTTON_HEIGTH / 2.,
        ))
        .ui(&mut root_ui());
    if black_button {
        game.start_pva(Rock::Black);
        return true;
    }

    let white_button = widgets::Button::new("White")
        .size(Vec2::new(BUTTTON_LENGTH, BUTTTON_HEIGTH))
        .position(Vec2::new(
            ((GRID_WINDOW_SIZE + PANEL_WINDOW_SIZE) / 2) as f32 + BUTTTON_LENGTH / 3.,
            (GRID_WINDOW_SIZE / 2) as f32 - BUTTTON_HEIGTH / 2.,
        ))
        .ui(&mut root_ui());
    if white_button {
        game.start_pva(Rock::White);
        return true;
    }

    false
}

pub fn display_panel_text(game: &mut Game) {
    let mut y_offset = TEXT_OFFSET;
    let play_time = game.play_time.elapsed().as_millis();
    draw_text(
        if play_time > 1000 {
            format!(
                "Time: {:.2}s",
                if game.winner != Winner::None {
                    0.
                } else {
                    game.play_time.elapsed().as_secs_f32()
                },
            )
        } else {
            format!(
                "Time: {}ms",
                if game.winner != Winner::None {
                    0
                } else {
                    play_time
                },
            )
        }
        .as_str(),
        GRID_WINDOW_SIZE as f32 + TEXT_OFFSET,
        y_offset,
        POLICE_SIZE,
        BLACK,
    );

    y_offset += TEXT_OFFSET;
    let previous_play_time = game.previous_play_time.as_millis();
    draw_text(
        if previous_play_time > 1000 {
            format!("Previous: {:.2}s", game.previous_play_time.as_secs_f32())
        } else {
            format!("Previous: {}ms", previous_play_time)
        }
        .as_str(),
        GRID_WINDOW_SIZE as f32 + TEXT_OFFSET,
        y_offset,
        POLICE_SIZE,
        BLACK,
    );

    if game.mode != GameMode::PvP && game.computer_average_play_time != 0. {
        y_offset += TEXT_OFFSET;
        let highest_play_time = game.computer_highest_play_time.as_millis();
        draw_text(
            if highest_play_time > 1000 {
                format!(
                    "Highest: {:.2}s",
                    game.computer_highest_play_time.as_secs_f32()
                )
            } else {
                format!("Highest: {:.0}ms", highest_play_time)
            }
            .as_str(),
            GRID_WINDOW_SIZE as f32 + TEXT_OFFSET,
            y_offset,
            POLICE_SIZE,
            BLACK,
        );

        y_offset += TEXT_OFFSET;
        draw_text(
            if game.computer_average_play_time > 1000. {
                format!("Average: {:.2}s", game.computer_average_play_time / 1000.)
            } else {
                format!("Average: {:.0}ms", game.computer_average_play_time)
            }
            .as_str(),
            GRID_WINDOW_SIZE as f32 + TEXT_OFFSET,
            y_offset,
            POLICE_SIZE,
            BLACK,
        );

        y_offset += TEXT_OFFSET;
        let lowest_play_time = game.computer_lowest_play_time.as_millis();
        draw_text(
            if lowest_play_time > 1000 {
                format!(
                    "Lowest: {:.2}s",
                    game.computer_lowest_play_time.as_secs_f32()
                )
            } else {
                format!("Lowest: {:.0}ms", lowest_play_time)
            }
            .as_str(),
            GRID_WINDOW_SIZE as f32 + TEXT_OFFSET,
            y_offset,
            POLICE_SIZE,
            BLACK,
        );
    }

    if game.rules.capture {
        y_offset += TEXT_OFFSET;
        draw_text(
            format!("Black capture: {}", game.board.black.captures).as_str(),
            GRID_WINDOW_SIZE as f32 + TEXT_OFFSET,
            y_offset,
            POLICE_SIZE,
            BLACK,
        );

        y_offset += TEXT_OFFSET;
        draw_text(
            format!("White capture: {}", game.board.white.captures).as_str(),
            GRID_WINDOW_SIZE as f32 + TEXT_OFFSET,
            y_offset,
            POLICE_SIZE,
            BLACK,
        );
    }

    y_offset += TEXT_OFFSET;
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
        y_offset,
        POLICE_SIZE,
        BLACK,
    );

    let surrender_button = widgets::Button::new(
        if game.mode == GameMode::AvA || game.winner != Winner::None {
            "Back"
        } else {
            "Surrender"
        },
    )
    .size(Vec2::new(BUTTTON_LENGTH - 30., BUTTTON_HEIGTH - 30.))
    .position(Vec2::new(
        (GRID_WINDOW_SIZE + PANEL_WINDOW_SIZE / 2) as f32 - (BUTTTON_LENGTH - 30.) / 2.,
        GRID_WINDOW_SIZE as f32 - 70.,
    ))
    .ui(&mut root_ui());

    if surrender_button {
        if game.winner == Winner::None {
            game.winner = if game.current_player == Player::Black {
                Winner::White
            } else {
                Winner::Black
            };
        } else {
            game.playing = false;
        }
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
        );
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
