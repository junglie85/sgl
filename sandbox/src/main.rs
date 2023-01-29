use sgl::{Key, Pixel, Scene, SglError, Window};

fn main() -> Result<(), SglError> {
    let mut window = Window::new(320, 240, "Sandbox")?;
    let mut scene = Scene::new();

    while !window.closed() && !window.key_down(Key::Escape) {
        window.update();

        scene.clear(Pixel::rgb(0x1f, 0x1f, 0xdf));
        scene.line(0.0, 0.0, 160.0, 120.0, Pixel::WHITE); //TODO: Next.

        window.display(&mut scene);
    }

    Ok(())
}
