use std::{mem::size_of, ops::Deref, slice::from_raw_parts};

use crate::{Pixel, SglError};

pub struct Bitmap {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
}

impl Bitmap {
    pub fn from_pixels<P>(width: u32, height: u32, pixels: P) -> Result<Self, SglError>
    where
        P: IntoIterator<Item = Pixel>,
    {
        let pixels = Vec::from_iter(pixels.into_iter());

        if width as usize * height as usize != pixels.len() {
            return Err(SglError::General(format!(
                "expected pixel count of {} but got {}",
                width * height,
                pixels.len(),
            )));
        }

        Ok(Self {
            width,
            height,
            pixels,
        })
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixels(&self) -> &[Pixel] {
        &self.pixels
    }
}

impl Deref for Bitmap {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe {
            from_raw_parts(
                self.pixels.as_ptr() as *const u8,
                size_of::<Pixel>() * self.pixels.len(),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_some_bitmap_from_pixels() {
        let pixels = [Pixel::RED; 6];
        let bitmap = Bitmap::from_pixels(2, 3, pixels).unwrap();

        assert_eq!(2, bitmap.width());
        assert_eq!(3, bitmap.height());
        assert_eq!(&[Pixel::RED; 6], bitmap.pixels());
    }

    #[test]
    fn does_not_create_bitmap_when_dimensions_dont_match_pixel_count() {
        let pixels = [Pixel::RED];
        let bitmap = Bitmap::from_pixels(2, 3, pixels);

        assert!(bitmap.is_err());
    }

    #[test]
    fn derefs_into_slice_of_bytes() {
        let pixels = [Pixel::RED; 1];
        let bitmap = Bitmap::from_pixels(1, 1, pixels).unwrap();

        assert_eq!([0xff, 0x00, 0x00, 0xff], &*bitmap);
    }
}
