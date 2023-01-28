use sgl::{Key, Pixel, Scene, SglError, Window};

fn main() -> Result<(), SglError> {
    let mut window = Window::new(320, 240, "Sandbox")?;

    while !window.closed() && !window.key_down(Key::Escape) {
        window.update();

        let mut scene = Scene::new();
        scene.clear(Pixel::rgb(0x1f, 0x1f, 0xdf));

        window.display(scene);
    }

    Ok(())
}
