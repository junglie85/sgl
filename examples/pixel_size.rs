use sgl::{GraphicsDevice, Key, Pixel, Renderer, SglError, Window};
use sgl_math::v2;

fn main() -> Result<(), SglError> {
    let mut window = Window::new(320, 240, "Pixel size example", 4, 4)?;
    let mut gpu = GraphicsDevice::new(&window)?;
    let renderer = Renderer::new(&gpu, &window)?;

    while !window.closed() && !window.key_down(Key::Escape) {
        window.update();

        let mut scene = renderer.begin_scene(&window);
        scene.clear(Pixel::rgb(0x1f, 0x1f, 0xdf));
        scene.draw_line(v2(0.0, 0.0), v2(160.0, 120.0), Pixel::WHITE, 2.0);
        scene.draw_line(v2(160.0, 120.0), v2(320.0, 0.0), Pixel::YELLOW, 2.0);

        renderer.end_scene(scene, &mut gpu);
    }

    Ok(())
}
