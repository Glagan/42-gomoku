use macroquad::prelude::*;

const NB_LINES: i32 = 19;
const WINDOW_SIZE: i32 = 800;
const BORDER_OFFSET: i32 = 22;
const SQUARE_SIZE: i32 = 42;


fn window_conf() -> Conf {
    Conf {
        window_title: "Gomoku".to_owned(),
        window_height: WINDOW_SIZE,
        window_width: WINDOW_SIZE,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(BEIGE);
        //Draw Board
        {
        
            for i in 0..NB_LINES {
                draw_line((i * SQUARE_SIZE + BORDER_OFFSET) as f32, (BORDER_OFFSET - 1) as f32, (i * SQUARE_SIZE + BORDER_OFFSET) as f32, (WINDOW_SIZE - BORDER_OFFSET + 1) as f32, 2., BLACK);
            }
            for i in 0..NB_LINES {
                draw_line((BORDER_OFFSET - 1) as f32, (i * SQUARE_SIZE + BORDER_OFFSET) as f32,(WINDOW_SIZE - BORDER_OFFSET + 1) as f32,  (i * SQUARE_SIZE + BORDER_OFFSET) as f32, 2., BLACK);
            }
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
        {
            let (mouse_x, mouse_y) = mouse_position();
            let rock_x = mouse_x as i32 / SQUARE_SIZE;
            let rock_y = mouse_y as i32/ SQUARE_SIZE;

            draw_circle((rock_x * SQUARE_SIZE + BORDER_OFFSET) as f32, (rock_y * SQUARE_SIZE + BORDER_OFFSET) as f32, 20., BLACK);
        }
        next_frame().await
    }
}