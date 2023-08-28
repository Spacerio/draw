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
    let list = create_list();
    let mut n = 0;
    loop {
        clear_background(BEIGE);
        if is_key_down(KeyCode::Q) {
            break;
        }
        draw_board();
        if is_key_pressed(KeyCode::Space) {
            n += 1;
        }
        if is_key_pressed(KeyCode::P) {
            n -= 1;
        }

        // draw_piece([1, 1, 2, 3]);
        draw_piece(list.get(n).unwrap().to_owned());

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
            match (i % 2, j % 2) {
                (1, 1) | (0, 0) => draw_rectangle(x, y, w, h, BROWN),
                _ => ()
            }
        }
    }
}

fn draw_piece(l: [i32;4]) {
    let r = 25.0;
    let x = l[0] as f32 * (screen_width() / 8.0) - r * 2.0;
    let y = l[1] as f32 * (screen_height() / 8.0) - r * 2.0;
    draw_circle(x, y, r, WHITE);
    let x = l[2] as f32 * (screen_width() / 8.0) - r * 2.0;
    let y = l[3] as f32 * (screen_height() / 8.0) - r * 2.0;
    draw_circle(x, y, r, BLACK);
    println!("{:?}", l);
}

fn create_list() -> Vec<[i32;4]> {
    let mut l: Vec<[i32;4]> = Vec::new();
    let n = 8;
    for wx in 1..=n {
        for wy in 1..=n {
            for bx in 1..=n {
                for by in 1..=n {
                    if wx != bx && wy != by {
                        if (bx as i32 - by as i32).abs() != (wy as i32 - wx as i32).abs() {
                            if (bx as i32 + by as i32).abs() != (wy as i32 + wy as i32).abs() {
                                l.push([wx, wy, bx, by]);
                                println!("{}", l.len());
                            }
                        }
                    }
                }
            }
        }
    }
    l
}
