use super::pixel::Pixel;

pub struct Scene {
    pub(crate) clear_color: Option<Pixel>,
}

impl Scene {
    pub fn new() -> Self {
        Self { clear_color: None }
    }

    pub fn clear(&mut self, pixel: Pixel) {
        self.clear_color = Some(pixel);
    }
}
