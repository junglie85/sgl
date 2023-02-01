use sgl::{Key, Pixel, Scene, SglError, Window};

fn main() -> Result<(), SglError> {
    let mut window = Window::new(1280, 960, "Lines example", 1, 1)?;

    while !window.closed() && !window.key_down(Key::Escape) {
        window.update();

        let mut scene = Scene::new(window.view());
        scene.clear(Pixel::rgb(0x1f, 0x1f, 0xdf));
        scene.draw_line([0.0, 0.0], [640.0, 480.0], 2.0, Pixel::WHITE);
        scene.draw_line([640.0, 480.0], [1280.0, 0.0], 2.0, Pixel::YELLOW);

        window.display(scene);
    }

    Ok(())
}
