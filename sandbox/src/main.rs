use sgl::{Key, Pixel, Scene, SglError, Window};

fn main() -> Result<(), SglError> {
    let mut window = Window::new(320, 240, "Sandbox", 1, 1)?;

    while !window.closed() && !window.key_down(Key::Escape) {
        window.update();

        let mut scene = Scene::new(window.view());
        scene.clear(Pixel::rgb(0x1f, 0x1f, 0xdf));
        scene.draw_line([0.0, 0.0], [160.0, 120.0], Pixel::WHITE, 2.0);
        scene.draw_line([160.0, 120.0], [320.0, 0.0], Pixel::YELLOW, 2.0);
        scene.draw_point([160.0, 130.0], Pixel::RED);
        scene.draw_rect([10.0, 200.0], [310.0, 230.0], Pixel::WHITE, 2.0);
        scene.draw_filled_rect([10.0, 200.0], [310.0, 230.0], Pixel::GREEN);

        window.display(scene);
    }

    Ok(())
}
