use sgl::{load_file, Bitmap, GraphicsDevice, Key, Pixel, Renderer, SglError, Window};
use sgl_math::v2;

fn main() -> Result<(), SglError> {
    let mut window = Window::new(320, 240, "Pixel size example", 4, 4)?;
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
    let center = v2(w / 2.0, h / 2.0);

    while !window.closed() && !window.key_down(Key::Escape) {
        window.update();

        let mut scene = renderer.begin_scene(&window);
        scene.clear(Pixel::rgb(0x1f, 0x1f, 0xdf));
        scene.draw_line(v2(0.0, 0.0), v2(160.0, 120.0), Pixel::WHITE, 2.0);
        scene.draw_line(v2(160.0, 120.0), v2(320.0, 0.0), Pixel::YELLOW, 2.0);
        scene.draw_point(v2(160.0, 130.0), Pixel::RED);
        scene.draw_rect(v2(10.0, 200.0), v2(310.0, 230.0), Pixel::WHITE, 2.0);
        scene.draw_filled_rect(v2(10.0, 200.0), v2(310.0, 230.0), Pixel::GREEN);
        scene.draw_textured_rect(v2(10.0, 140.0), v2(50.0, 190.0), &texture);
        scene.draw_textured_rect(v2(60.0, 140.0), v2(76.0, 156.0), &wizard);
        scene.draw_textured_rect_ext(
            v2(80.0, 140.0),
            v2(80.0 + 64.0, 140.0 + 64.0),
            &ufo,
            center,
            v2(w, h),
        );

        renderer.end_scene(scene, &mut gpu);
    }

    Ok(())
}
