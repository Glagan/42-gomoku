use crate::{
    board::{Board, Move, Pawn},
    draw::{display_panel_text, draw_current_rock, draw_goban},
    player::Player,
    rules::RuleSet,
};
use macroquad::{
    prelude::*,
    ui::{root_ui, widgets},
};
use std::time::{Duration, Instant};

const GRID_WINDOW_SIZE: usize = 800;
const PANEL_WINDOW_SIZE: usize = 200;
const BORDER_OFFSET: usize = 22;
const SQUARE_SIZE: usize = 42;
const BUTTTON_HEIGTH: f32 = 70.;
const BUTTTON_LENGHT: f32 = 200.;

mod board;
mod computer;
mod draw;
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

    let mut now: Instant = Instant::now();
    let mut prev_play_time: Duration = now - now;
    let p1: Player = Player::Black;
    let p2: Player = Player::White;
    let mut current_player: &Player = &p1;
    let mut b_mouse_pressed: bool = false;
    let mut in_game: bool = false;

    let mut p1_captured: usize = 0;
    let mut p2_captured: usize = 0;
    loop {
        clear_background(BEIGE);
        //Draw grid
        if !in_game {
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
                in_game = true;
                now = Instant::now();
                prev_play_time = now - now;
            }
        }

        if in_game {
            draw_goban(&board);
            draw_current_rock(current_player, &board);
            display_panel_text(
                now,
                &mut in_game,
                &p1_captured,
                &p2_captured,
                current_player,
                &prev_play_time,
            );
            if !in_game {
                board = Board::default();
                current_player = &p1;
            }
            if is_mouse_button_released(MouseButton::Left) {
                b_mouse_pressed = false;
            }
            if is_mouse_button_down(MouseButton::Left) && !b_mouse_pressed {
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
                                player: *current_player,
                            },
                        );
                        if current_player == &p1 {
                            current_player = &p2;
                        } else {
                            current_player = &p1;
                        }
                        prev_play_time = now.elapsed();
                        now = Instant::now();
                    }
                }
            }
        }

        next_frame().await
    }
}
