#[macro_use]
extern crate lazy_static;

use crate::{
    draw::{
        color_selector, display_panel_text, display_winner, draw_goban, draw_recommended_move,
        draw_rock_preview, game_selector, options_selector,
    },
    game::{Game, GameMode, Winner},
    rock::Rock,
};
use macroquad::prelude::*;
use macroquad::ui::{root_ui, Skin};

const GRID_WINDOW_SIZE: usize = 800;
const PANEL_WINDOW_SIZE: usize = 200;
const BORDER_OFFSET: usize = 22;
const SQUARE_SIZE: usize = 42;
const BUTTTON_HEIGTH: f32 = 70.;
const BUTTTON_LENGTH: f32 = 200.;

mod board;
mod computer;
mod draw;
mod game;
mod options;
mod pattern;
mod player;
mod rock;
mod rules;

fn window_conf() -> Conf {
    Conf {
        window_title: "Gomoku".to_owned(),
        window_height: GRID_WINDOW_SIZE as i32,
        window_width: (GRID_WINDOW_SIZE + PANEL_WINDOW_SIZE) as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[cfg(not(feature = "cli_ava"))]
#[macroquad::main(window_conf)]
async fn main() {
    // Add skin for checkboxes
    let default_skin = {
        let checkbox_style = root_ui()
            .style_builder()
            .color_selected(GREEN)
            .color_hovered(RED)
            .color_clicked(BLUE)
            .color_selected_hovered(GREEN)
            .color(RED)
            .build();
        Skin {
            checkbox_style,
            ..root_ui().default_skin()
        }
    };
    root_ui().push_skin(&default_skin);

    let mut game = Game::default();
    let mut b_mouse_pressed: bool = false;

    loop {
        clear_background(BEIGE);

        // Options
        if game.in_options {
            options_selector(&mut game);
        }
        // Game mode selector
        else if !game.playing {
            if game_selector(&mut game) {
                b_mouse_pressed = true;
            }
        }
        // Color selector in PvA
        else if game.mode == GameMode::PvA && game.player_color == Rock::None {
            if color_selector(&mut game) {
                b_mouse_pressed = true;
            }
        }
        // Draw game
        else {
            draw_goban(&game);
            display_panel_text(&mut game);

            // Winner
            if game.winner != Winner::None {
                display_winner(&game);
            } else {
                // Handle Input based on current game mode
                if game.mode != GameMode::AvA {
                    // Computer Play
                    if game.mode == GameMode::PvA && game.current_player == game.computer_play_as {
                        game.play_computer()
                    }
                    // Move preview and await input
                    else {
                        if game.generate_recommended_move {
                            draw_recommended_move(&mut game);
                        }
                        draw_rock_preview(&game);

                        // Player play
                        if is_mouse_button_released(MouseButton::Left) {
                            b_mouse_pressed = false;
                        } else if is_mouse_button_down(MouseButton::Left)
                            && !b_mouse_pressed
                            && (game.mode == GameMode::PvP
                                || (game.mode == GameMode::PvA
                                    && game.current_player != game.computer_play_as))
                        {
                            b_mouse_pressed = true;
                            let (mouse_x, mouse_y) = mouse_position();
                            if mouse_x < (GRID_WINDOW_SIZE - 2) as f32
                                && mouse_y < (GRID_WINDOW_SIZE - 2) as f32
                            {
                                game.play_player(
                                    mouse_x as usize / SQUARE_SIZE,
                                    mouse_y as usize / SQUARE_SIZE,
                                );
                            }
                        }
                    }
                }
                // AvA just play in turn, no input
                else {
                    game.play_computer()
                }
            }
        }

        next_frame().await
    }
}

#[cfg(feature = "cli_ava")]
fn main() {
    let mut game = Game::default();
    game.start(GameMode::AvA);
    while game.winner == Winner::None {
        game.play_computer()
    }
}
