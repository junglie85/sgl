use mint::Vector2;

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

    pub fn line<V2>(&mut self, from: V2, to: V2, thickness: f32, color: Pixel)
    where
        V2: Into<Vector2<f32>>,
    {
        self.draw_commands.push(DrawCommand::Line {
            from: from.into().into(),
            to: to.into().into(),
            thickness,
            color,
        });
    }
}
