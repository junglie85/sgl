use wgpu::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            r: 0xff,
            g: 0xff,
            b: 0xff,
            a: 0xff,
        }
    }
}

impl Pixel {
    pub const BLACK: Self = Self::rgba(0x00, 0x00, 0x00, 0xff);
    pub const GREEN: Self = Self::rgba(0x00, 0xff, 0x00, 0xff);
    pub const RED: Self = Self::rgba(0xff, 0x00, 0x00, 0xff);
    pub const WHITE: Self = Self::rgba(0xff, 0xff, 0xff, 0xff);
    pub const YELLOW: Self = Self::rgba(0xff, 0xff, 0x00, 0xff);

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 0xff)
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn to_array(&self) -> [f32; 4] {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        ]
    }
}

impl From<Pixel> for Color {
    fn from(value: Pixel) -> Self {
        Self {
            r: value.r as f64 / 255.0,
            g: value.g as f64 / 255.0,
            b: value.b as f64 / 255.0,
            a: value.a as f64 / 255.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_pixel_to_array() {
        let pixel = Pixel::default();

        assert_eq!([1.0; 4], pixel.to_array());
    }

    #[test]
    fn convert_pixel_to_wgpu_color() {
        let pixel = Pixel::default();

        assert_eq!(Color::WHITE, Into::<Color>::into(pixel));
    }

    #[test]
    fn pixel_equality() {
        let white = Pixel::WHITE;
        let black = Pixel::BLACK;

        assert_eq!(white, white);
        assert_ne!(white, black);
    }
}
