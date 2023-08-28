use macroquad::prelude::*;

fn window_config() -> Conf {
    Conf {
        window_title: "Chess".to_string(),
        window_width: 800,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    loop {
        clear_background(WHITE);
        if is_key_down(KeyCode::Q) {
            break;
        }
        // Draw chess board
        draw_board();
        // Draw pieces from coordinates
        next_frame().await
    }
}

fn draw_board() {
    let w = screen_width() / 8.0;
    let h = screen_height() / 8.0;
    for i in 0..8 {
        for j in 0..8 {
            let x = i as f32 * w;
            let y = j as f32 * h;
            if i % 2 == 0 {
                if j % 2 == 0 {
                    draw_rectangle(x, y, w, h, BLACK)
                }
            } else if j % 2 == 1 {
                draw_rectangle(x, y, w, h, BLACK)
            }
        }
    }
}
