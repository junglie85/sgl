use sgl_math::{v2, Vec2};

use crate::{geometry::Vertex, Pixel, Texture};

pub(crate) struct LineShape {
    from: Vec2,
    to: Vec2,
}

impl LineShape {
    pub(crate) fn new(from: Vec2, to: Vec2) -> Self {
        Self { from, to }
    }

    pub(crate) fn fill_geometry(
        &self,
        thickness: f32,
        color: Pixel,
        pixel_size: Vec2,
    ) -> (Vec<Vertex>, Vec<u32>) {
        let fill_color = color.to_array();

        let extent = (self.to - self.from).perp_cw().norm() * thickness;

        let vertices = vec![
            Vertex {
                coords: (v2(self.from.x, self.from.y) * pixel_size).to_array(),
                tex_coords: [0.0, 1.0],
                fill_color,
            },
            Vertex {
                coords: (v2(self.to.x, self.to.y) * pixel_size).to_array(),
                tex_coords: [0.0, 0.0],
                fill_color,
            },
            Vertex {
                coords: (v2(self.from.x + extent.x, self.from.y + extent.y) * pixel_size)
                    .to_array(),
                tex_coords: [1.0, 1.0],
                fill_color,
            },
            Vertex {
                coords: (v2(self.to.x + extent.x, self.to.y + extent.y) * pixel_size).to_array(),
                tex_coords: [1.0, 0.0],
                fill_color,
            },
        ];

        let indices = vec![0, 1, 2, 3];

        (vertices, indices)
    }
}

pub(crate) struct RectangleShape {
    from: Vec2,
    to: Vec2,
    point_count: usize,
}

impl RectangleShape {
    pub(crate) fn new(from: Vec2, to: Vec2) -> Self {
        Self {
            from,
            to,
            point_count: 4,
        }
    }

    fn point(&self, i: usize) -> Vec2 {
        match i {
            0 => v2(self.from.x, self.from.y),
            1 => v2(self.from.x, self.to.y),
            2 => v2(self.to.x, self.to.y),
            3 => v2(self.to.x, self.from.y),
            _ => unreachable!("rectangles only have 4 points"),
        }
    }

    fn width(&self) -> f32 {
        self.to.x - self.from.x
    }

    fn height(&self) -> f32 {
        self.to.y - self.from.y
    }

    pub(crate) fn outline_geometry(
        &self,
        thickness: f32,
        color: Pixel,
        pixel_size: Vec2,
    ) -> (Vec<Vertex>, Vec<u32>) {
        let mut vertices: Vec<Vertex> = Vec::with_capacity((self.point_count + 1) * 2);
        let mut indices = Vec::with_capacity((self.point_count + 1) * 2);

        for i in 0..self.point_count {
            // https://stackoverflow.com/questions/69631855/extrude-2d-vertices-vectors

            // Get the normals of the vectors either side of the current point (the in and out vectors).
            let idx_p1 = (i + 1) % self.point_count;
            let idx_p2 = if i == 0 { self.point_count - 1 } else { i - 1 };

            let p0 = self.point(i);
            let p1 = self.point(idx_p1);
            let p2 = self.point(idx_p2);

            let v_in = (p0 - p1).perp_cw().norm();
            let v_out = (p2 - p0).perp_cw().norm();

            // Bisect the normals.
            let mut bisector = v_in + v_out;
            bisector /= bisector.dot(v_in).abs();
            bisector *= thickness;

            // Add the original vertex and the extruded vertex to the geometry.
            vertices.push(Vertex::new(
                (p0 * pixel_size).to_array(),
                [0.0, 0.0],
                color.to_array(),
            ));
            vertices.push(Vertex::new(
                ((p0 + bisector) * pixel_size).to_array(),
                [0.0, 0.0],
                color.to_array(),
            ));

            // And the indices for each.
            indices.push(i as u32 * 2);
            indices.push(i as u32 * 2 + 1);
        }

        // Close the outline.
        indices.push(indices[0]);
        indices.push(indices[1]);

        (vertices, indices)
    }

    pub(crate) fn fill_geometry(&self, color: Pixel, pixel_size: Vec2) -> (Vec<Vertex>, Vec<u32>) {
        let fill_color = color.to_array();

        let mut vertices = Vec::with_capacity(self.point_count);

        for i in 0..self.point_count {
            let coords = self.point(i as usize);

            vertices.push(Vertex {
                coords: (coords * pixel_size).to_array(),
                tex_coords: [0.0, 0.0],
                fill_color,
            });
        }

        let indices = vec![0, 1, 3, 3, 1, 2];

        (vertices, indices)
    }

    pub(crate) fn texture_geometry(
        &self,
        texture: &Texture,
        sub_coords: Option<(Vec2, Vec2)>,
        pixel_size: Vec2,
    ) -> (Vec<Vertex>, Vec<u32>) {
        let fill_color = Pixel::WHITE.to_array();

        let (tex_from, tex_to) = sub_coords.unwrap_or((
            v2(0.0, 0.0),
            v2(texture.width() as f32, texture.height() as f32),
        ));

        // Normalize texture coords.
        let tex_left = tex_from.x / texture.width() as f32;
        let tex_right = tex_to.x / texture.width() as f32;
        let tex_bottom = tex_from.y / texture.height() as f32;
        let tex_top = tex_to.y / texture.height() as f32;

        let tex_width = tex_right - tex_left;
        let tex_height = tex_top - tex_bottom;

        let mut vertices = Vec::with_capacity(self.point_count);

        for i in 0..self.point_count {
            let coords = self.point(i as usize);

            let ratio_x = (coords.x - self.from.x) / self.width();
            let ratio_y = (coords.y - self.from.y) / self.height();

            let tex_coords = v2(
                tex_left + tex_width * ratio_x,
                tex_bottom + tex_height * ratio_y,
            );

            vertices.push(Vertex {
                coords: (coords * pixel_size).to_array(),
                tex_coords: tex_coords.to_array(),
                fill_color,
            });
        }

        let indices = vec![0, 1, 3, 3, 1, 2];

        (vertices, indices)
    }
}
