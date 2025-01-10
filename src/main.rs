use macroquad::{color::hsl_to_rgb, prelude::*};
use yakui_macroquad::*;

pub const GAME_WIDTH: f32 = 640.0;
pub const GAME_HEIGHT: f32 = 480.0;
const COLUMN: usize = 10;
const ROW: usize = 10;
const GRID_SIZE: usize = COLUMN * ROW;
const TEXTURE_SIZE: f32 = 32.;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Freedom Grid".into(),
        icon: None,
        window_width: GAME_WIDTH as i32,
        window_height: GAME_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut camera = Camera2D {
        zoom: vec2(2. / GAME_WIDTH, 2. / GAME_HEIGHT),
        target: vec2((GAME_WIDTH * 0.5).floor(), (GAME_HEIGHT * 0.5).floor()),
        ..Default::default()
    };

    let mut grid = [0.0; GRID_SIZE];

    for (num, item) in grid.iter_mut().enumerate() {
        *item += num as f32 / GRID_SIZE as f32;
    }

    let pos_x = GAME_WIDTH / 2. - ((COLUMN as f32 * TEXTURE_SIZE) / 2.);
    let pos_y = GAME_HEIGHT / 2. - ((ROW as f32 * TEXTURE_SIZE) / 2.);

    loop {
        for (num, item) in grid.iter_mut().enumerate() {
            *item += 0.0001;
            if *item > 1.0 {
                *item = 0.0;
            }
        }

        // ! controller
        if mouse_wheel().1 < 0. && TEXTURE_SIZE > 4. {
            camera.zoom += Vec2::new(0.1 / GAME_WIDTH, 0.1 / GAME_HEIGHT)
        } else if mouse_wheel().1 > 0. {
            camera.zoom -= Vec2::new(0.1 / GAME_WIDTH, 0.1 / GAME_HEIGHT)
        }

        if camera.zoom.x < 0. {
            camera.zoom += Vec2::new(0.1 / GAME_WIDTH, 0.1 / GAME_HEIGHT)
        }

        if is_mouse_button_down(MouseButton::Left) {
            let mouse_pos = mouse_delta_position();

            camera.offset.x -= mouse_pos.x;
            camera.offset.y += mouse_pos.y;
        }

        if is_key_pressed(KeyCode::Space) {
            camera.offset = Vec2::ZERO;
            camera.zoom = Vec2::new(2. / GAME_WIDTH, 2. / GAME_HEIGHT);
        }

        // ! ui
        yakui_macroquad::ui(|_| {
            yakui::center(|| {
                yakui::colored_box_container(yakui::Color::REBECCA_PURPLE, || {
                    yakui::pad(yakui::widgets::Pad::all(4.0), || {
                        yakui::text(16.0, "hello, world!");
                    });
                });
            });
        });

        // ! draw
        set_camera(&camera);

        clear_background(BLACK);

        for (index, cell) in grid.iter().enumerate() {
            let x = (index % COLUMN) as f32 * TEXTURE_SIZE;
            let y = (index / COLUMN) as f32 * TEXTURE_SIZE;

            draw_rectangle(
                pos_x + x,
                pos_y + y,
                TEXTURE_SIZE,
                TEXTURE_SIZE,
                hsl_to_rgb(*cell, 1.0, 0.5),
            );
        }

        yakui_macroquad::draw();

        next_frame().await;
    }
}
