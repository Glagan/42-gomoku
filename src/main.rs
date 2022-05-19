use crate::{
    board::{Board, Move, Pawn},
    draw::{display_panel_text, draw_current_rock, draw_goban, game_selector},
    game::{Game, GameMode},
    player::Player,
    rules::RuleSet,
};
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
    let rules = RuleSet::default();
    let mut board = Board::default();
    // println!("{}", board);
    // println!("---");
    // let player1 = Player::Black;
    // let player2 = Player::White;
    // let computer1 = Computer::new(&rules, &player1);
    // let computer2 = Computer::new(&rules, &player2);
    // for i in 0..20 {
    //     let now = Instant::now();
    //     if i % 2 == 0 {
    //         let play_result = computer1.play(&board, 4);
    //         println!("play: {:#?}", play_result);
    //         if let Ok(play) = play_result {
    //             if let Some(movement) = play.movement {
    //                 board.set_move(&rules, &movement);
    //             }
    //         }
    //     } else {
    //         let play_result = computer2.play(&board, 4);
    //         println!("play: {:#?}", play_result);
    //         if let Ok(play) = play_result {
    //             if let Some(movement) = play.movement {
    //                 board.set_move(&rules, &movement);
    //             }
    //         }
    //     }
    //     let elapsed = now.elapsed();
    //     println!("Elapsed: {:.2?}", elapsed);
    //     println!("{}", board);
    //     println!("---");
    // }

    let mut game = Game::default();
    let mut b_mouse_pressed: bool = false;
    loop {
        clear_background(BEIGE);

        // Draw grid
        if !game.playing {
            game_selector(&mut game);
        } else {
            // Draw game
            draw_goban(&board);
            display_panel_text(&mut game);
            draw_current_rock(&game);

            // Handle Input
            if game.mode != GameMode::AvA {
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
                        if board.pieces[Board::coordinates_to_index(rock_x, rock_y)] == Pawn::None {
                            board.set_move(
                                &rules,
                                &Move {
                                    index: Board::coordinates_to_index(rock_x, rock_y),
                                    player: game.current_player,
                                },
                            );
                            game.next_player();
                        }
                    }
                }
            }
        }

        next_frame().await
    }
}
