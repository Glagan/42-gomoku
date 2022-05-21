use crate::{
    board::{Board, Move, Pawn},
    draw::{display_panel_text, draw_current_rock, draw_goban, game_selector},
    game::{Game, GameMode, Winner},
    player::Player,
};
use colored::Colorize;
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
            draw_goban(&game.board);
            display_panel_text(&mut game);

            // Winner
            if game.winner != Winner::None {
                // TODO "Winner"
            } else {
                // Handle Input based on current game mode
                if game.mode != GameMode::AvA {
                    draw_current_rock(&game);

                    // Player play
                    if is_mouse_button_released(MouseButton::Left) {
                        b_mouse_pressed = false;
                    } else if is_mouse_button_down(MouseButton::Left)
                        && !b_mouse_pressed
                        && (game.mode == GameMode::PvP
                            || (game.mode == GameMode::PvA && game.current_player == Player::White))
                    {
                        b_mouse_pressed = true;
                        let (mouse_x, mouse_y) = mouse_position();
                        if mouse_x < (GRID_WINDOW_SIZE - 2) as f32
                            && mouse_y < (GRID_WINDOW_SIZE - 2) as f32
                        {
                            let rock_x = mouse_x as usize / SQUARE_SIZE;
                            let rock_y = mouse_y as usize / SQUARE_SIZE;
                            if game.board.pieces[Board::coordinates_to_index(rock_x, rock_y)]
                                == Pawn::None
                            {
                                game.board.set_move(
                                    &game.rules,
                                    &Move {
                                        index: Board::coordinates_to_index(rock_x, rock_y),
                                        player: game.current_player,
                                    },
                                );
                                if game.board.is_winning(&game.rules, &game.current_player) {
                                    game.player_won();
                                } else {
                                    game.next_player();
                                }
                            }
                        }
                    }
                    // Computer Play
                    if game.mode == GameMode::PvA && game.current_player == Player::Black {
                        let play_result =
                            game.computer
                                .play(&game.rules, &game.board, 5, &game.current_player);
                        if let Ok(play) = play_result {
                            println!(
                                "computer play: {} in {}ms",
                                play,
                                game.play_time.elapsed().as_millis()
                            );
                            if let Some(movement) = play.movement {
                                game.board.set_move(&game.rules, &movement);
                                if game.board.is_winning(&game.rules, &game.current_player) {
                                    game.player_won();
                                } else {
                                    game.next_player();
                                }
                            }
                        } else {
                            println!("{}", "computer returned an empty play result".red());
                        }
                    }
                }
                // AvA just play in turn, no input
                else {
                    let play_result =
                        game.computer
                            .play(&game.rules, &game.board, 5, &game.current_player);
                    if let Ok(play) = play_result {
                        println!(
                            "computer play: {} in {}ms",
                            play,
                            game.play_time.elapsed().as_millis()
                        );
                        if let Some(movement) = play.movement {
                            game.board.set_move(&game.rules, &movement);
                            if game.board.is_winning(&game.rules, &game.current_player) {
                                game.player_won();
                            } else {
                                game.next_player();
                            }
                        }
                    } else {
                        println!("{}", "computer returned an empty play result".red());
                    }
                }
            }
        }

        next_frame().await
    }
}
