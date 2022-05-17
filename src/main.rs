use board::{BOARD_SIZE, Board};
use macroquad::{prelude::*};
use player::Player;

use crate::board::Move;

const WINDOW_SIZE: usize = 800;
const BORDER_OFFSET: usize = 22;
const SQUARE_SIZE: usize = 42;

mod board;
mod computer;
mod player;

fn window_conf() -> Conf {
    Conf {
        window_title: "Gomoku".to_owned(),
        window_height: WINDOW_SIZE as i32,
        window_width: WINDOW_SIZE as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut board = Board::default();
    println!("{}", board);
    board.set_move(&Move {
        index: 0,
        player: Player::Black,
    });
    board.set_move(&Move {
        index: 19,
        player: Player::White,
    });
    board.set_move(&Move {
        index: 20,
        player: Player::Black,
    });
    board.set_move(&Move {
        index: 360,
        player: Player::White,
    });
    board.set_move(&Move {
        index: 361,
        player: Player::Black,
    });
    
    let p1 = Player::Black;
    let p2 = Player::White;
    let mut current_player: &Player = &p1;
    let mut b_mouse_pressed: bool = false;

    loop {
        clear_background(BEIGE);
        //Draw Board
        {
            //Draw line
            for i in 0..BOARD_SIZE {
                draw_line((i * SQUARE_SIZE + BORDER_OFFSET) as f32, (BORDER_OFFSET - 1) as f32, (i * SQUARE_SIZE + BORDER_OFFSET) as f32, (WINDOW_SIZE - BORDER_OFFSET + 1) as f32, 2., BLACK);
            }
            for i in 0..BOARD_SIZE {
                draw_line((BORDER_OFFSET - 1) as f32, (i * SQUARE_SIZE + BORDER_OFFSET) as f32,(WINDOW_SIZE - BORDER_OFFSET + 1) as f32,  (i * SQUARE_SIZE + BORDER_OFFSET) as f32, 2., BLACK);
            }
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
        }
        //Draw board
        if is_mouse_button_down(MouseButton::Left) && !b_mouse_pressed {
            b_mouse_pressed = true;
            
        }
        if is_mouse_button_released(MouseButton::Left){
            b_mouse_pressed = false;
            if current_player == &p1 {
                current_player = &p2;
            }
            else {
                current_player = &p1;
            }
        }

    
        //Draw current rock
        {
            let (mouse_x, mouse_y) = mouse_position();
            if mouse_x < WINDOW_SIZE as f32 && mouse_y < WINDOW_SIZE as f32 {

                let rock_x = mouse_x as usize / SQUARE_SIZE;
                let rock_y = mouse_y as usize/ SQUARE_SIZE;
                draw_circle((rock_x * SQUARE_SIZE + BORDER_OFFSET) as f32, (rock_y * SQUARE_SIZE + BORDER_OFFSET) as f32, 20., if current_player == &p1 {BLACK} else {WHITE});
            }
        }
        next_frame().await
    }
}