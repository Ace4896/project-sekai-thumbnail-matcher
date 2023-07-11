use image::{DynamicImage, ImageBuffer};
use wasm_bindgen::prelude::*;
use web_sys::ImageData;

use crate::hasher;

/// Converts JS ImageData (RGBA byte values) into a DynamicImage.
fn convert_image_data(image_data: ImageData) -> DynamicImage {
    let image_buffer =
        ImageBuffer::from_vec(image_data.width(), image_data.height(), image_data.data().0)
            .unwrap();

    DynamicImage::ImageRgba8(image_buffer)
}

/// Generate a pHash for the specified thumbnail image.
#[wasm_bindgen]
pub fn generate_thumbnail_phash(image_data: ImageData) -> u64 {
    let image = convert_image_data(image_data);
    hasher::generate_thumbnail_phash(&image)
}
