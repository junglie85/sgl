use mint::Vector2;

#[derive(Debug, Clone, Copy)]
pub struct View {
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    viewport_left: f32,
    viewport_right: f32,
    viewport_bottom: f32,
    viewport_top: f32,
    transform: [f32; 16],
}

impl View {
    pub fn new(center: impl Into<Vector2<f32>>, width: f32, height: f32) -> Self {
        let center = center.into();

        let half_width = width / 2.0;
        let half_height = height / 2.0;

        let mut view = Self {
            left: center.x - half_width,
            right: center.x + half_width,
            bottom: center.y - half_height,
            top: center.y + half_height,
            viewport_left: 0.0,
            viewport_right: 1.0,
            viewport_bottom: 0.0,
            viewport_top: 1.0,
            transform: [0.0; 16],
        };

        view.calculate_transform();

        view
    }

    pub fn left(&self) -> f32 {
        self.left
    }

    pub fn right(&self) -> f32 {
        self.right
    }

    pub fn bottom(&self) -> f32 {
        self.bottom
    }

    pub fn top(&self) -> f32 {
        self.top
    }

    pub fn width(&self) -> f32 {
        self.right - self.left
    }

    pub fn height(&self) -> f32 {
        self.top - self.bottom
    }

    pub fn viewport_left(&self) -> f32 {
        self.viewport_left
    }

    pub fn viewport_right(&self) -> f32 {
        self.viewport_right
    }

    pub fn viewport_top(&self) -> f32 {
        self.viewport_top
    }

    pub fn viewport_bottom(&self) -> f32 {
        self.viewport_bottom
    }

    pub fn transform(&self) -> [f32; 16] {
        self.transform
    }

    #[rustfmt::skip]
    fn calculate_transform(&mut self) {
        // http://learnwebgl.brown37.net/08_projections/projections_ortho.html
        let width = self.right - self.left;
        let height = self.top - self.bottom;

        self.transform = [
             2.0 / width,                        0.0,                               0.0, 0.0,
             0.0,                                2.0 / height,                      0.0, 0.0,
             0.0,                                0.0,                              -0.5, 0.0,
            -(self.right + self.left) / width, -(self.top + self.bottom) / height, -0.0, 1.0,
        ];
    }
}
