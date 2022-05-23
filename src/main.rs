#[macro_use]
extern crate lazy_static;

use crate::{
    draw::{display_panel_text, draw_goban, draw_rock_preview, game_selector},
    game::{Game, GameMode, Winner},
    player::Player,
};
use draw::display_winner;
use macroquad::prelude::*;

const GRID_WINDOW_SIZE: usize = 800;
const PANEL_WINDOW_SIZE: usize = 200;
const BORDER_OFFSET: usize = 22;
const SQUARE_SIZE: usize = 42;
const BUTTTON_HEIGTH: f32 = 70.;
const BUTTTON_LENGHT: f32 = 200.;

mod board;
mod computer;
mod draw;
mod game;
mod pattern;
mod player;
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

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::default();
    let mut b_mouse_pressed: bool = false;
    loop {
        clear_background(BEIGE);

        // Draw grid
        if !game.playing {
            game_selector(&mut game);
        } else {
            // Draw game
            draw_goban(&game);
            display_panel_text(&mut game);

            // Winner
            if game.winner != Winner::None {
                display_winner(&game);
            } else {
                // Handle Input based on current game mode
                if game.mode != GameMode::AvA {
                    // Computer Play
                    if game.mode == GameMode::PvA && game.current_player == Player::Black {
                        game.play_computer()
                    }
                    // Move preview and await input
                    else {
                        draw_rock_preview(&game);

                        // Player play
                        if is_mouse_button_released(MouseButton::Left) {
                            b_mouse_pressed = false;
                        } else if is_mouse_button_down(MouseButton::Left)
                            && !b_mouse_pressed
                            && (game.mode == GameMode::PvP
                                || (game.mode == GameMode::PvA
                                    && game.current_player == Player::White))
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
