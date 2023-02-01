use mint::Vector2;

use crate::{renderer::DrawCommand, View};

use super::pixel::Pixel;

pub struct Scene {
    pub(crate) clear_color: Option<Pixel>,
    pub(crate) draw_commands: Vec<DrawCommand>,
}

impl Scene {
    pub fn new(view: View) -> Self {
        Self {
            clear_color: None,
            draw_commands: vec![DrawCommand::View(view)],
        }
    }

    pub fn clear(&mut self, color: Pixel) {
        self.clear_color = Some(color);
        self.draw_commands.drain(1..);
    }

    pub fn draw_line<V2>(&mut self, from: V2, to: V2, thickness: f32, color: Pixel)
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

#[cfg(test)]
mod tests {
    use sgl_math::v2;

    use crate::View;

    use super::*;

    #[test]
    fn new_scene_is_not_cleared() {
        let scene = Scene::new(View::new([0.0, 0.0], 1280.0, 720.0));

        assert_eq!(None, scene.clear_color);
    }

    #[test]
    fn new_scene_sets_window_view() {
        let window_view = View::new([0.0, 0.0], 1280.0, 720.0);
        let scene = Scene::new(window_view);

        assert_eq!(vec![DrawCommand::View(window_view)], scene.draw_commands);
    }

    #[test]
    fn scene_draw_line() {
        let mut scene = Scene::new(View::new([0.0, 0.0], 1280.0, 720.0));
        scene.draw_line(v2(0.0, 0.0), v2(100.0, 150.0), 2.0, Pixel::WHITE);

        assert_eq!(
            DrawCommand::Line {
                from: v2(0.0, 0.0),
                to: v2(100.0, 150.0),
                thickness: 2.0,
                color: Pixel::WHITE
            },
            scene.draw_commands[1]
        );
    }

    #[test]
    fn clear_scene_removes_draw_commands() {
        let window_view = View::new([0.0, 0.0], 1280.0, 720.0);
        let mut scene = Scene::new(window_view);
        scene.draw_line(v2(0.0, 0.0), v2(100.0, 150.0), 2.0, Pixel::WHITE);
        scene.clear(Pixel::YELLOW);

        assert_eq!(Some(Pixel::YELLOW), scene.clear_color);
        assert_eq!(vec![DrawCommand::View(window_view)], scene.draw_commands);
    }
}
