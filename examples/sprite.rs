use sgl::{GraphicsDevice, Key, Pixel, Renderer, SglError, Window};

fn main() -> Result<(), SglError> {
    let mut window = Window::new(128, 96, "Sprite example", 1, 1)?;
    let mut gpu = GraphicsDevice::new(&window)?;
    let renderer = Renderer::new(&gpu, &window)?;

    let wizard_bitmap = sgl::util::bitmap::from_image_bytes(include_bytes!("wizard.png"))?;
    let wizard = renderer.create_texture(&gpu, &wizard_bitmap, Some("wizard"))?;

    while !window.closed() && !window.key_down(Key::Escape) {
        window.update();

        let mut scene = renderer.begin_scene(&window);
        scene.clear(Pixel::rgb(0x1f, 0x1f, 0xdf));
        scene.draw_textured_rect([10.0, 10.0], [42.0, 42.0], &wizard);

        renderer.end_scene(scene, &mut gpu);
    }

    Ok(())
}
