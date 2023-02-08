use sgl::{load_file, Bitmap, GraphicsDevice, Key, Pixel, Renderer, SglError, Window};

fn main() -> Result<(), SglError> {
    let mut window = Window::new(320, 240, "Sandbox", 1, 1)?;
    let mut gpu = GraphicsDevice::new(&window)?;
    let renderer = Renderer::new(&gpu, &window)?;

    let pixels = [Pixel::RED, Pixel::GREEN, Pixel::WHITE, Pixel::YELLOW];
    let bitmap = Bitmap::from_pixels(2, 2, pixels)?;
    let texture = renderer.create_texture(&gpu, &bitmap, Some("a texture"))?;

    let wizard_bytes = load_file("examples/assets/wizard.png")?;
    let wizard_bitmap = Bitmap::from_image_bytes(&wizard_bytes)?;
    let wizard = renderer.create_texture(&gpu, &wizard_bitmap, Some("wizard"))?;

    let ufo_bytes = load_file("examples/assets/ufo.png")?;
    let ufo_bitmap = Bitmap::from_image_bytes(&ufo_bytes)?;
    let ufo = renderer.create_texture(&gpu, &ufo_bitmap, Some("ufo"))?;
    let w = ufo_bitmap.width() as f32;
    let h = ufo_bitmap.height() as f32;
    let center = [w / 2.0, h / 2.0];

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
        scene.draw_textured_rect_ext(
            [80.0, 140.0],
            [80.0 + 64.0, 140.0 + 64.0],
            &ufo,
            center,
            [w, h],
        );

        renderer.end_scene(scene, &mut gpu);
    }

    Ok(())
}
