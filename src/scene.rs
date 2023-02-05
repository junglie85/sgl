use mint::Vector2;
use sgl_math::{v2, Vec2};

use crate::{renderer::DrawCommand, Texture, View};

use super::pixel::Pixel;

pub struct Scene<'scene> {
    pub(crate) clear_color: Option<Pixel>,
    pub(crate) draw_commands: Vec<DrawCommand<'scene>>,
}

impl<'scene> Scene<'scene> {
    pub(crate) fn new(view: View) -> Self {
        Self {
            clear_color: None,
            draw_commands: vec![DrawCommand::View(view)],
        }
    }

    pub fn clear(&mut self, color: Pixel) {
        self.clear_color = Some(color);
        self.draw_commands.drain(1..);
    }

    pub fn draw_point<V>(&mut self, point: V, color: Pixel)
    where
        V: Into<Vector2<f32>>,
    {
        let from: Vec2 = point.into().into();
        let to = from + v2(1.0, 1.0);
        self.draw_filled_rect(from, to, color);
    }

    pub fn draw_line<V>(&mut self, from: V, to: V, color: Pixel, thickness: f32)
    where
        V: Into<Vector2<f32>>,
    {
        self.draw_commands.push(DrawCommand::Line {
            from: from.into().into(),
            to: to.into().into(),
            color,
            thickness,
        });
    }

    pub fn draw_rect<V>(&mut self, from: V, to: V, color: Pixel, thickness: f32)
    where
        V: Into<Vector2<f32>>,
    {
        self.draw_commands.push(DrawCommand::Rect {
            from: from.into().into(),
            to: to.into().into(),
            color,
            thickness,
        });
    }

    pub fn draw_filled_rect<V>(&mut self, from: V, to: V, color: Pixel)
    where
        V: Into<Vector2<f32>>,
    {
        self.draw_commands.push(DrawCommand::RectFilled {
            from: from.into().into(),
            to: to.into().into(),
            color,
        })
    }

    pub fn draw_textured_rect<V>(&mut self, from: V, to: V, texture: &'scene Texture)
    where
        V: Into<Vector2<f32>>,
    {
        self.draw_commands.push(DrawCommand::RectTextured {
            from: from.into().into(),
            to: to.into().into(),
            texture,
        })
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

        if let DrawCommand::View(actual_view) =
            scene.draw_commands.get(0).expect("draw_command::view")
        {
            assert_eq!(window_view, *actual_view);
        };
    }

    #[test]
    fn scene_draw_line() {
        let mut scene = Scene::new(View::new([0.0, 0.0], 1280.0, 720.0));

        let line_from = v2(0.0, 0.0);
        let line_to = v2(100.0, 150.0);
        let line_color = Pixel::WHITE;
        let line_thickness = 2.0;
        scene.draw_line(line_from, line_to, line_color, line_thickness);

        if let DrawCommand::Line {
            from,
            to,
            color,
            thickness,
        } = scene.draw_commands.get(1).expect("draw_command::line")
        {
            assert_eq!(line_from, *from);
            assert_eq!(line_to, *to);
            assert_eq!(line_color, *color);
            assert_eq!(line_thickness, *thickness);
        };
    }

    #[test]
    fn scene_draw_rect() {
        let mut scene = Scene::new(View::new([0.0, 0.0], 1280.0, 720.0));

        let rect_from = v2(0.0, 0.0);
        let rect_to = v2(100.0, 150.0);
        let rect_color = Pixel::WHITE;
        let rect_thickness = 2.0;
        scene.draw_rect(rect_from, rect_to, rect_color, rect_thickness);

        if let DrawCommand::Rect {
            from,
            to,
            color,
            thickness,
        } = scene.draw_commands.get(1).expect("draw_command::rect")
        {
            assert_eq!(rect_from, *from);
            assert_eq!(rect_to, *to);
            assert_eq!(rect_color, *color);
            assert_eq!(rect_thickness, *thickness);
        };
    }

    #[test]
    fn scene_draw_filled_rect() {
        let mut scene = Scene::new(View::new([0.0, 0.0], 1280.0, 720.0));

        let rect_from = v2(0.0, 0.0);
        let rect_to = v2(100.0, 150.0);
        let rect_color = Pixel::WHITE;
        scene.draw_filled_rect(rect_from, rect_to, rect_color);

        if let DrawCommand::RectFilled { from, to, color } = scene
            .draw_commands
            .get(1)
            .expect("draw_command::rect_filled")
        {
            assert_eq!(rect_from, *from);
            assert_eq!(rect_to, *to);
            assert_eq!(rect_color, *color);
        };
    }

    #[test]
    fn scene_draw_point() {
        let mut scene = Scene::new(View::new([0.0, 0.0], 1280.0, 720.0));

        let point_from = v2(0.0, 0.0);
        let point_color = Pixel::WHITE;
        scene.draw_point(point_from, point_color);

        if let DrawCommand::RectFilled { from, to, color } = scene
            .draw_commands
            .get(1)
            .expect("draw_command::rect_filled")
        {
            assert_eq!(point_from, *from);
            assert_eq!(point_from + v2(1.0, 1.0), *to);
            assert_eq!(point_color, *color);
        };
    }

    #[test]
    fn clear_scene_removes_draw_commands() {
        let window_view = View::new([0.0, 0.0], 1280.0, 720.0);
        let mut scene = Scene::new(window_view);
        scene.draw_line(v2(0.0, 0.0), v2(100.0, 150.0), Pixel::WHITE, 2.0);
        scene.clear(Pixel::YELLOW);

        assert_eq!(Some(Pixel::YELLOW), scene.clear_color);

        if let DrawCommand::View(actual_view) =
            scene.draw_commands.get(0).expect("draw_command::view")
        {
            assert_eq!(window_view, *actual_view);
        };
    }
}
