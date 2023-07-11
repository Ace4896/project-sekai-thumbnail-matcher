use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

use crate::hasher;

#[wasm_bindgen]
pub fn generate_thumbnail_phash(buffer: &[u8]) -> Option<Uint8Array> {
    if let Ok(img_thumbnail) = image::load_from_memory(buffer) {
        let phash = hasher::generate_thumbnail_phash(&img_thumbnail);
        Some(Uint8Array::from(phash.to_be_bytes().as_slice()))
    } else {
        None
    }
}