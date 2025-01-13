use macroquad::prelude::*;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Macroquad Freedom Camera2D".into(),
        icon: None,
        window_width: 640,
        window_height: 480,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // setup camera variable so we can use it while the app is running
    let mut camera = Camera2D {
        zoom: vec2(2. / screen_width(), 2. / screen_height()),
        ..Default::default()
    };

    // variable for zooming in and out
    // you can remove this if you dont plan to use camera zoom
    let mut zoomer = Vec2::ZERO;

    loop {
        camera_fixer(&mut camera, &mut zoomer);

        // ! draw
        set_camera(&camera);

        clear_background(BLACK);

        let texture_size = 128.0;

        draw_rectangle(
            texture_size / -2.,
            texture_size / -2.,
            texture_size,
            texture_size,
            ORANGE,
        );

        draw_triangle(vec2(50.0, -50.0), vec2(25.0, 0.0), vec2(75.0, 0.0), BLUE);

        next_frame().await;
    }
}

fn camera_fixer(camera: &mut Camera2D, zoomer: &mut Vec2) {
    // disable stretch when window size is changed
    camera.zoom = vec2(
        2. / screen_width() + zoomer.x / screen_width(),
        2. / screen_height() + zoomer.y / screen_height(),
    );

    // set min window heigh and width
    if screen_width() < 320. {
        request_new_screen_size(320., screen_height());
    }

    if screen_height() < 240. {
        request_new_screen_size(screen_width(), 240.);
    }

    // zoom in
    if mouse_wheel().1 > 0. {
        *zoomer += 0.2

    // zoom out
    } else if mouse_wheel().1 < 0. && zoomer.x > -1. {
        *zoomer -= 0.2;
    }

    // move camera
    if is_mouse_button_down(MouseButton::Left) {
        let mouse_pos = mouse_delta_position();

        // you can change the add and sub assigments if you plan to use different camera movement
        camera.offset.x -= mouse_pos.x;
        camera.offset.y += mouse_pos.y;
    }

    // reset position and zoom
    if is_key_pressed(KeyCode::Space) {
        camera.offset = Vec2::ZERO;
        *zoomer = Vec2::ZERO;
    }
}
