use macroquad::prelude::*;

pub const ZOOM_DEFAULT: f32 = 2.0; // change this if you want different starting zoom level
pub const ZOOM_VALUE: f32 = 0.2; // change this if you want to zoom in/out faster or slower

pub trait FreedomCamera2D {
    fn freedom() -> Self;
}

impl FreedomCamera2D for Camera2D {
    fn freedom() -> Self {
        Camera2D {
            zoom: vec2(2. / screen_width(), 2. / screen_height()),
            ..Default::default()
        }
    }
}

pub fn camera_fixer(camera: &mut Camera2D, zoomer: &mut f32) {
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

        *zoomer += ZOOM_VALUE;

    // zoom out
    } else if mouse_wheel().1 < 0. && *zoomer > ZOOM_VALUE {
        // way to always get precise .2 accuracy for floating point operation
        *zoomer = (*zoomer * 10.).round() / 10.;

        *zoomer -= ZOOM_VALUE;

        if *zoomer < ZOOM_VALUE {
            *zoomer = ZOOM_VALUE;
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
