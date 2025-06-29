use macroquad::prelude::*;
use macroquad_freedom_camera2d::*;

#[macroquad::main("Freedom Camera2D")]
async fn main() {
    // setup camera variable so we can use it while the app is running
    let mut camera = Camera2D::freedom();

    // variable for zooming in and out
    // you can remove this if you dont plan to use camera zoom
    let mut zoomer = ZOOM_DEFAULT;

    loop {
        // ! draw
        set_camera(&camera);

        camera_fixer(&mut camera, &mut zoomer);

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
