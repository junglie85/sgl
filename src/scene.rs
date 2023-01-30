use crate::{renderer::DrawCommand, Window};

use super::pixel::Pixel;

pub struct Scene {
    pub(crate) clear_color: Option<Pixel>,
    pub(crate) draw_commands: Vec<DrawCommand>,
}

impl Scene {
    pub fn new(window: &Window) -> Self {
        Self {
            clear_color: None,
            draw_commands: vec![DrawCommand::View(window.view)],
        }
    }

    pub fn clear(&mut self, color: Pixel) {
        self.clear_color = Some(color);
    }

    pub fn line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, thickness: f32, color: Pixel) {
        self.draw_commands.push(DrawCommand::Line {
            from: (x0, y0),
            to: (x1, y1),
            thickness,
            color,
        });
    }
}
