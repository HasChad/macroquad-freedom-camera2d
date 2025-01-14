use macroquad::prelude::*;

const ZOOM_DEFAULT: f32 = 2.0; // change this if you want different starting zoom level

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
    let mut zoomer = ZOOM_DEFAULT;

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

        draw_rectangle(100.0, texture_size / -2., texture_size, texture_size, BLUE);

        next_frame().await;
    }
}

fn camera_fixer(camera: &mut Camera2D, zoomer: &mut f32) {
    // resize camera width and height to window width and height
    camera.zoom = vec2(*zoomer / screen_width(), *zoomer / screen_height());

    // set min window width and heigh
    if screen_width() < 320. {
        request_new_screen_size(320., screen_height());
    }

    if screen_height() < 240. {
        request_new_screen_size(screen_width(), 240.);
    }

    // zoom in
    if mouse_wheel().1 > 0. {
        // way to always get precise .2 accuracy for floating point operation
        *zoomer = (*zoomer * 10.).round() / 10.;

        *zoomer += 0.2;

    // zoom out
    } else if mouse_wheel().1 < 0. && *zoomer > 0.2 {
        // way to always get precise .2 accuracy for floating point operation
        *zoomer = (*zoomer * 10.).round() / 10.;

        *zoomer -= 0.2;

        if *zoomer < 0.2 {
            *zoomer = 0.2;
        }
    }

    // move camera
    if is_mouse_button_down(MouseButton::Left) {
        let mouse_pos = mouse_delta_position();

        // you can change the add and sub assigments if you plan to use different camera movement
        camera.target.x += mouse_pos.x * screen_width() / *zoomer;
        camera.target.y += mouse_pos.y * screen_height() / *zoomer;
    }

    // reset position and zoom
    if is_key_pressed(KeyCode::Space) {
        camera.target = Vec2::ZERO;
        *zoomer = ZOOM_DEFAULT;
    }
}
