use image::{DynamicImage, ImageBuffer};
use serde::Serialize;
use wasm_bindgen::prelude::*;
use web_sys::ImageData;

use crate::{extractor, hasher};

#[wasm_bindgen(typescript_custom_section)]
const IRGBA8_IMAGE_DATA: &'static str = r#"
interface IRgba8ImageData {
  data: number[];
  width: number;
  height: number;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IRgba8ImageData[]")]
    pub type IRgba8ImageDataArray;
}

/// A thin wrapper around RGBA8 image data.
///
/// JS's ImageData constructor doesn't copy the data into it's own buffer, so data corruption occurs.
/// See: https://github.com/rustwasm/wasm-bindgen/issues/2445
#[derive(Serialize)]
struct Rgba8ImageData {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

impl From<&DynamicImage> for Rgba8ImageData {
    fn from(value: &DynamicImage) -> Self {
        Self {
            data: value.to_rgba8().as_raw().clone(),
            width: value.width(),
            height: value.height(),
        }
    }
}

/// Converts JS ImageData (RGBA byte values) into a DynamicImage.
fn convert_to_dynamic_image(image_data: ImageData) -> DynamicImage {
    let image_buffer =
        ImageBuffer::from_vec(image_data.width(), image_data.height(), image_data.data().0)
            .unwrap();

    DynamicImage::ImageRgba8(image_buffer)
}

/// Extracts card thumbnails from a character list screenshot.
#[wasm_bindgen]
pub fn extract_thumbnail_images(image_data: ImageData) -> IRgba8ImageDataArray {
    let image = convert_to_dynamic_image(image_data);
    let thumbnail_images = extractor::extract_thumbnail_images(&image)
        .iter()
        .map(Rgba8ImageData::from)
        .collect::<Vec<_>>();

    serde_wasm_bindgen::to_value(&thumbnail_images)
        .unwrap()
        .unchecked_into()
}

/// Generate a pHash for the specified thumbnail image.
#[wasm_bindgen]
pub fn generate_thumbnail_phash(image_data: ImageData) -> u64 {
    let image = convert_to_dynamic_image(image_data);
    hasher::generate_thumbnail_phash(&image)
}
