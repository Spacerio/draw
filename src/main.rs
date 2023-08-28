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
}
