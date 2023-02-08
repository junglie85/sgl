use sgl::{load_file, Bitmap, GraphicsDevice, Key, Pixel, Renderer, SglError, Window};

fn main() -> Result<(), SglError> {
    let mut window = Window::new(256, 192, "Sprite example", 1, 1)?;
    let mut gpu = GraphicsDevice::new(&window)?;
    let renderer = Renderer::new(&gpu, &window)?;

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
        scene.draw_textured_rect([10.0, 10.0], [42.0, 42.0], &wizard);
        scene.draw_textured_rect_ext(
            [52.0, 10.0],
            [52.0 + 64.0, 10.0 + 64.0],
            &ufo,
            center,
            [w, h],
        );

        renderer.end_scene(scene, &mut gpu);
    }

    Ok(())
}
