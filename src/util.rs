#[cfg(feature = "image")]
pub mod bitmap {
    use image::{load_from_memory, DynamicImage, GenericImageView};

    use crate::{Bitmap, SglError};

    pub fn from_image_bytes(bytes: &[u8]) -> Result<Bitmap, SglError> {
        let img = load_from_memory(bytes).map_err(|e| SglError::General(e.to_string()))?;

        from_image(&img)
    }

    pub fn from_image(img: &DynamicImage) -> Result<Bitmap, SglError> {
        let dimensions = img.dimensions();
        let rgba = img.to_rgba8();
        let pixels = rgba
            .chunks(4)
            .map(|b| b.try_into())
            .collect::<Result<Vec<_>, _>>()?;

        Bitmap::from_pixels(dimensions.0, dimensions.1, pixels)
    }
}
