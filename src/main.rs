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
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut camera = Camera2D {
        zoom: vec2(2. / screen_width(), 2. / screen_height()),
        ..Default::default()
    };
    let mut zoomer = Vec2::ZERO;

    let mut grid = [0.0; GRID_SIZE];

    for (num, item) in grid.iter_mut().enumerate() {
        *item += num as f32 / GRID_SIZE as f32;
    }

    loop {
        for item in grid.iter_mut() {
            *item += 0.2 * get_frame_time();
            if *item > 1.0 {
                *item = 0.0;
            }
        }

        camera_fixer(&mut camera, &mut zoomer);

        // ! draw
        set_camera(&camera);

        clear_background(BLACK);

        let pos_x = (COLUMN as f32 * TEXTURE_SIZE) / 2.;
        let pos_y = (ROW as f32 * TEXTURE_SIZE) / 2.;

        for (index, cell) in grid.iter().enumerate() {
            let x = (index % COLUMN) as f32 * TEXTURE_SIZE;
            let y = (index / COLUMN) as f32 * TEXTURE_SIZE;

            draw_rectangle(
                x - pos_x,
                y - pos_y,
                TEXTURE_SIZE,
                TEXTURE_SIZE,
                hsl_to_rgb(*cell, 1.0, 0.5),
            );
        }

        next_frame().await;
    }
}

fn camera_fixer(camera: &mut Camera2D, zoomer: &mut Vec2) {
    // ! window res
    camera.zoom = vec2(
        2. / screen_width() + zoomer.x / screen_width(),
        2. / screen_height() + zoomer.y / screen_height(),
    );
    camera.target = Vec2::ZERO;

    if screen_width() < 320. {
        request_new_screen_size(320., screen_height());
    }

    if screen_height() < 240. {
        request_new_screen_size(screen_width(), 240.);
    }

    // ! controller
    if mouse_wheel().1 > 0. {
        *zoomer += 0.2
    } else if mouse_wheel().1 < 0. && zoomer.x > -2. {
        *zoomer -= 0.2;
    }

    if camera.zoom.x < 0. {
        camera.zoom += Vec2::new(0.1 / screen_width(), 0.1 / screen_height())
    }

    if is_mouse_button_down(MouseButton::Left) {
        let mouse_pos = mouse_delta_position();

        camera.offset.x -= mouse_pos.x;
        camera.offset.y += mouse_pos.y;
    }

    if is_key_pressed(KeyCode::Space) {
        camera.offset = Vec2::ZERO;
        *zoomer = Vec2::ZERO;
    }
}

/*
// ui things
yakui_macroquad::ui(|_| {
    yakui::center(|| {
        yakui::colored_box_container(yakui::Color::REBECCA_PURPLE, || {
            yakui::pad(yakui::widgets::Pad::all(4.0), || {
                yakui::text(16.0, "hello, world!");
            });
        });
    });
});

yakui_macroquad::draw();
*/
