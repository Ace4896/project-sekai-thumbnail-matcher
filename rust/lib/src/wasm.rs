use wasm_bindgen::prelude::*;

use crate::hasher;

#[wasm_bindgen]
pub fn generate_thumbnail_phash(buffer: &[u8]) -> Option<u64> {
    if let Ok(img_thumbnail) = image::load_from_memory(buffer) {
        Some(hasher::generate_thumbnail_phash(&img_thumbnail))
    } else {
        None
    }
}