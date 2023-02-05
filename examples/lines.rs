use sgl::{GraphicsDevice, Key, Pixel, Renderer, SglError, Window};

fn main() -> Result<(), SglError> {
    let mut window = Window::new(1280, 960, "Lines example", 1, 1)?;
    let mut gpu = GraphicsDevice::new(&window)?;
    let renderer = Renderer::new(&gpu, &window)?;

    while !window.closed() && !window.key_down(Key::Escape) {
        window.update();

        let mut scene = renderer.begin_scene(&window);
        scene.clear(Pixel::rgb(0x1f, 0x1f, 0xdf));
        scene.draw_line([0.0, 0.0], [640.0, 480.0], Pixel::WHITE, 2.0);
        scene.draw_line([640.0, 480.0], [1280.0, 0.0], Pixel::YELLOW, 2.0);

        renderer.end_scene(scene, &mut gpu);
    }

    Ok(())
}
