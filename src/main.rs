use macroquad::{prelude::*, ui::{widgets, root_ui}, hash};

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
    let mut n = 8;
    let mut pos = 0;
    let mut setup = true;
    let mut data = String::new();
    loop {
        clear_background(BEIGE);
        match get_last_key_pressed() {
            Some(KeyCode::P) => pos -= 1,
            Some(KeyCode::Space) => pos += 1,
            Some(KeyCode::S) => pos += 10,
            Some(KeyCode::Up) => n += 1,
            Some(KeyCode::Down) => n -= 1,
            Some(KeyCode::Q) => break,
            _ => ()
        }

        if n >= 3 && is_key_pressed(KeyCode::Enter) {
            setup = !setup
        }
        if setup {
            widgets::Window::new(hash!(), vec2(470., 50.), vec2(300., 60.))
                .label("Input n")
                .ui(&mut *root_ui(), |ui| {
                    ui.input_text(hash!(), "Input n", &mut data);
                    ui.label(None, &n.to_string());
                });
            if let Ok(s) = data.parse() {
                n = s;
            }
        } else {
            let list = create_list(n);
            draw_board(n);
            draw_piece(n, list.get(pos).unwrap().to_owned());
        }
        next_frame().await
    }
}

fn take_input() -> i32 {
    let mut data = String::new();
    widgets::Window::new(hash!(), vec2(470., 50.), vec2(300., 300.))
        .label("Input n")
        .ui(&mut *root_ui(), |ui| {
            ui.input_text(hash!(), "Input n", &mut data)
        });
    data.parse().unwrap_or_default()
}


fn draw_board(n: i32) {
    let w = screen_width() / n as f32;
    let h = screen_height() / n as f32;
    for i in 0..n {
        for j in 0..n {
            let x = i as f32 * w;
            let y = j as f32 * h;
            match (i % 2, j % 2) {
                (1, 1) | (0, 0) => draw_rectangle(x, y, w, h, BROWN),
                _ => ()
            }
        }
    }
}

fn draw_piece(n: i32, l: [i32;4]) {
    let tile_size = screen_height() / n as f32;
    let r = tile_size / 3.0;
    let x = l[0] as f32 * tile_size - tile_size / 2.0;
    let y = l[1] as f32 * tile_size - tile_size / 2.0;
    draw_circle(x, y, r, WHITE);
    let x = l[2] as f32 * tile_size - tile_size / 2.0;
    let y = l[3] as f32 * tile_size - tile_size / 2.0;
    draw_circle(x, y, r, BLACK);
}

fn create_list(n: i32) -> Vec<[i32;4]> {
    let mut l: Vec<[i32;4]> = Vec::new();
    for wx in 1..=n {
        for wy in 1..=n {
            for bx in 1..=n {
                for by in 1..=n {
                    if wx != bx && wy != by {
                        if (bx - by).abs() != (wy - wx).abs() {
                            if (bx + by) != (wx + wy) {
                                l.push([wx, wy, bx, by]);
                                // println!("{}", l.len());
                            }
                        }
                    }
                }
            }
        }
    }
    l
}
