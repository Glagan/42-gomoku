use std::time::{Instant, Duration};

const TEXT_OFFSET: f32 = 20.;
const POLICE_SIZE: f32 = 20.;

use macroquad::{
    prelude::{draw_line, BLACK, draw_circle, WHITE, mouse_position, draw_text, Vec2}, 
    ui::{widgets, root_ui}
};

use crate::{
    board::{BOARD_SIZE, Board, Pawn, BOARD_PIECES}, 
    BORDER_OFFSET, SQUARE_SIZE, GRID_WINDOW_SIZE, 
    player::{Player}, BUTTTON_LENGHT, PANEL_WINDOW_SIZE, BUTTTON_HEIGTH
};

pub fn draw_goban(board: &Board)
{
    //Draw line
    for i in 0..BOARD_SIZE {
        draw_line((i * SQUARE_SIZE + BORDER_OFFSET) as f32, (BORDER_OFFSET - 1) as f32, (i * SQUARE_SIZE + BORDER_OFFSET) as f32, (GRID_WINDOW_SIZE - BORDER_OFFSET + 1) as f32, 2., BLACK);
    }
    for i in 0..BOARD_SIZE {
        draw_line((BORDER_OFFSET - 1) as f32, (i * SQUARE_SIZE + BORDER_OFFSET) as f32,(GRID_WINDOW_SIZE - BORDER_OFFSET + 1) as f32,  (i * SQUARE_SIZE + BORDER_OFFSET) as f32, 2., BLACK);
    }
    draw_line((GRID_WINDOW_SIZE + 1) as f32, 0., (GRID_WINDOW_SIZE + 1) as f32, GRID_WINDOW_SIZE as f32, 4., BLACK);
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
            draw_circle((x * SQUARE_SIZE + BORDER_OFFSET) as f32, (y * SQUARE_SIZE + BORDER_OFFSET) as f32, 20., if board.pieces[i] == Pawn::Black {BLACK} else {WHITE});
        }
    }

}

pub fn draw_current_rock(current_player: &Player, board: &Board)
{
    let (mouse_x, mouse_y) = mouse_position();
    if mouse_x < (GRID_WINDOW_SIZE - 2) as f32 && mouse_y < (GRID_WINDOW_SIZE - 2) as f32 {
        let rock_x = (mouse_x - 1.) as usize / SQUARE_SIZE;
        let rock_y = (mouse_y - 1.) as usize/ SQUARE_SIZE;
        if board.pieces[Board::coordinates_to_index(rock_x, rock_y)] == Pawn::None {
            draw_circle((rock_x * SQUARE_SIZE + BORDER_OFFSET) as f32, (rock_y * SQUARE_SIZE + BORDER_OFFSET) as f32, 20., if *current_player == Player::Black {BLACK} else {WHITE});
        }
    }
}

pub fn display_panel_text(last_move_time: Instant, in_game: &mut bool, p1_captured: &usize, p2_captured: &usize, current_player: &Player, prev_time_play: &Duration) {
    draw_text(format!("Elapsed time: {:.2}", last_move_time.elapsed().as_secs_f32()).as_str(), GRID_WINDOW_SIZE as f32 + TEXT_OFFSET, TEXT_OFFSET, POLICE_SIZE, BLACK);
    draw_text(format!("Elapsed time: {:.2}", prev_time_play.as_secs_f32()).as_str(), GRID_WINDOW_SIZE as f32 + TEXT_OFFSET, TEXT_OFFSET * 2., POLICE_SIZE, BLACK);
    
    draw_text(format!("P1 captured: {}", p1_captured).as_str(), GRID_WINDOW_SIZE as f32 + TEXT_OFFSET, TEXT_OFFSET * 3., POLICE_SIZE, BLACK);
    draw_text(format!("P2 captured: {}", p2_captured).as_str(), GRID_WINDOW_SIZE as f32 + TEXT_OFFSET, TEXT_OFFSET * 4., POLICE_SIZE, BLACK);
    
    draw_text(format!("Turn: {}", if *current_player == Player::Black {"Player 1"} else {"Player 2"}).as_str(), GRID_WINDOW_SIZE as f32 + TEXT_OFFSET, TEXT_OFFSET * 5., POLICE_SIZE, BLACK);
    


    let surrender_button = widgets::Button::new("Surrender")
            .size(Vec2::new(BUTTTON_LENGHT - 30., BUTTTON_HEIGTH - 30.))
            .position(Vec2::new((GRID_WINDOW_SIZE + PANEL_WINDOW_SIZE / 2) as f32 - (BUTTTON_LENGHT - 30.) / 2., GRID_WINDOW_SIZE as f32 - 70.))
            .ui(&mut root_ui());

            if surrender_button{
                *in_game = false;
            }
}