#[macro_use]
extern crate lazy_static;

use crate::game::{Game, GameMode, Winner};

#[cfg(not(feature = "cli_ava"))]
use macroquad::prelude::*;
#[cfg(not(feature = "cli_ava"))]
use macroquad::ui::{root_ui, Skin};

mod board;
mod computer;
mod constants;
#[cfg(not(feature = "cli_ava"))]
mod draw;
mod game;
mod heuristic;
mod macros;
mod patterns;
mod player;
mod rock;
mod rules;

#[cfg(not(feature = "cli_ava"))]
fn window_conf() -> Conf {
    use crate::draw::{GRID_WINDOW_SIZE, PANEL_WINDOW_SIZE};

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
    use crate::{
        draw::{
            color_selector, display_panel_text, display_winner, draw_goban, draw_player_choices,
            draw_player_remaining_stones, draw_rock_preview, game_selector, options_selector,
            GRID_WINDOW_SIZE, SQUARE_SIZE,
        },
        game::Opening,
        macros::coord,
        rock::Rock,
    };

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
        let window_style = root_ui()
            .style_builder()
            .background(Image::from_file_with_format(
                include_bytes!("../ui/background.png"),
                None,
            ))
            .build();
        Skin {
            checkbox_style,
            window_style,
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
                display_winner(&mut game);
            } else {
                let opening = game.opening();
                // Handle openings
                if opening != Opening::None && !game.completed_opening {
                    if game.player_place_stones > 0 {
                        draw_rock_preview(&game);
                        draw_player_remaining_stones(&game);

                        // Player play
                        if is_mouse_button_released(MouseButton::Left) {
                            b_mouse_pressed = false;
                        } else if is_mouse_button_down(MouseButton::Left) && !b_mouse_pressed {
                            b_mouse_pressed = true;
                            let (mouse_x, mouse_y) = mouse_position();
                            if mouse_x < (GRID_WINDOW_SIZE - 2) as f32
                                && mouse_y < (GRID_WINDOW_SIZE - 2) as f32
                            {
                                game.play_opening(coord!(
                                    mouse_x as i16 / SQUARE_SIZE,
                                    mouse_y as i16 / SQUARE_SIZE
                                ));
                            }
                        }
                    } else if game.ask_player_choice {
                        draw_player_choices(&mut game);
                    }
                }
                // Handle Input based on current game mode
                else if game.mode != GameMode::AvA {
                    // Computer Play
                    if game.mode == GameMode::PvA && game.current_player == game.computer_play_as {
                        game.play_computer()
                    }
                    // Move preview and await input
                    else {
                        if game.generate_recommended_move && !game.computer_generated_moves {
                            game.generate_computer_recommended_moves();
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
                                game.play_player(coord!(
                                    mouse_x as i16 / SQUARE_SIZE,
                                    mouse_y as i16 / SQUARE_SIZE
                                ));
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
