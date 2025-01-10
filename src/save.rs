use macroquad::{color::hsl_to_rgb, prelude::*};

const COLUMN: usize = 5;
const ROW: usize = 5;
const GRID_SIZE: usize = COLUMN * ROW;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Freedom Grid".into(),
        icon: None,
        window_width: 640,
        window_height: 480,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut texture_size = 64.;
    let center_x = screen_width() / 2.;
    let center_y = screen_height() / 2.;

    let mut position_x = center_x - (COLUMN as f32 * texture_size) / 2.;
    let mut position_y = center_y - (ROW as f32 * texture_size) / 2.;

    let mut grid = [0.0; GRID_SIZE];

    for (num, item) in grid.iter_mut().enumerate() {
        *item += num as f32 / 25.;
    }

    loop {
        if mouse_wheel().1 < 0. {
            texture_size -= 4.
        } else if mouse_wheel().1 > 0. {
            texture_size += 4.
        }

        if is_mouse_button_down(MouseButton::Left) {
            let mouse_pos = mouse_delta_position();

            position_x -= mouse_pos.x * 100.;
            position_y -= mouse_pos.y * 100.;
        }

        if is_key_pressed(KeyCode::Space) {
            texture_size = 64.;
            position_x = center_x - (COLUMN as f32 * texture_size) / 2.;
            position_y = center_y - (ROW as f32 * texture_size) / 2.;
        }

        // ! draw
        for (index, cell) in grid.iter().enumerate() {
            let x = (index % COLUMN) as f32 * texture_size;
            let y = (index / COLUMN) as f32 * texture_size;

            draw_rectangle(
                position_x + x,
                position_y + y,
                texture_size,
                texture_size,
                hsl_to_rgb(*cell, 1.0, 0.5),
            );
        }

        next_frame().await;
    }
}
