use sgl::{util, Bitmap, GraphicsDevice, Key, Pixel, Renderer, SglError, Window};

fn main() -> Result<(), SglError> {
    let mut window = Window::new(320, 240, "Sandbox", 1, 1)?;
    let mut gpu = GraphicsDevice::new(&window)?;
    let renderer = Renderer::new(&gpu, &window)?;

    let pixels = [Pixel::RED, Pixel::GREEN, Pixel::WHITE, Pixel::YELLOW];
    let bitmap = Bitmap::from_pixels(2, 2, pixels)?;
    let texture = renderer.create_texture(&gpu, &bitmap, Some("a texture"))?;

    let wizard_bitmap = util::bitmap::from_image_bytes(include_bytes!("wizard.png"))?;
    let wizard = renderer.create_texture(&gpu, &wizard_bitmap, Some("wizard"))?;

    while !window.closed() && !window.key_down(Key::Escape) {
        window.update();

        let mut scene = renderer.begin_scene(&window);
        scene.clear(Pixel::rgb(0x1f, 0x1f, 0xdf));
        scene.draw_line([0.0, 0.0], [160.0, 120.0], Pixel::WHITE, 2.0);
        scene.draw_line([160.0, 120.0], [320.0, 0.0], Pixel::YELLOW, 2.0);
        scene.draw_point([160.0, 130.0], Pixel::RED);
        scene.draw_rect([10.0, 200.0], [310.0, 230.0], Pixel::WHITE, 2.0);
        scene.draw_filled_rect([10.0, 200.0], [310.0, 230.0], Pixel::GREEN);
        scene.draw_textured_rect([10.0, 140.0], [50.0, 190.0], &texture);
        scene.draw_textured_rect([60.0, 140.0], [76.0, 156.0], &wizard);

        renderer.end_scene(scene, &mut gpu);
    }

    Ok(())
}
