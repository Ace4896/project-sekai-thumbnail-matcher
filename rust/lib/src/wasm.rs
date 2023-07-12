use image::{DynamicImage, ImageBuffer};
use wasm_bindgen::prelude::*;
use web_sys::ImageData;

use crate::{extractor, hasher};

/// Converts JS ImageData (RGBA byte values) into a DynamicImage.
fn convert_to_dynamic_image(image_data: ImageData) -> DynamicImage {
    let image_buffer =
        ImageBuffer::from_vec(image_data.width(), image_data.height(), image_data.data().0)
            .unwrap();

    DynamicImage::ImageRgba8(image_buffer)
}

/// Converts a DynamicImage into JS ImageData.
fn convert_to_image_data(dynamic_image: &DynamicImage) -> ImageData {
    let rgba8_image = dynamic_image.to_rgba8();

    ImageData::new_with_u8_clamped_array_and_sh(
        wasm_bindgen::Clamped(rgba8_image.as_raw()),
        rgba8_image.width(),
        rgba8_image.height(),
    )
    .unwrap()
}

/// Extracts card thumbnails from a character list screenshot.
#[wasm_bindgen]
pub fn extract_thumbnail_images(image_data: ImageData) -> Vec<ImageData> {
    let image = convert_to_dynamic_image(image_data);
    extractor::extract_thumbnail_images(&image)
        .iter()
        .map(convert_to_image_data)
        .collect::<Vec<_>>()
}

/// Generate a pHash for the specified thumbnail image.
#[wasm_bindgen]
pub fn generate_thumbnail_phash(image_data: ImageData) -> u64 {
    let image = convert_to_dynamic_image(image_data);
    hasher::generate_thumbnail_phash(&image)
}
