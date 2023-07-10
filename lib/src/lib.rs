use image::DynamicImage;

/// Generates a pHash for the specified card thumbnail.
pub fn generate_thumbnail_phash(img_thumbnail: &DynamicImage) -> String {
    let img_cropped = crop_thumbnail_for_hash(img_thumbnail);
    String::new()
}

/// Crops a thumbnail to a region that isn't affected by UI elements. This removes:
/// 
/// - 10% from the top
/// - 40% from the bottom
/// - 25% from the left
/// - 10% from the right
fn crop_thumbnail_for_hash(img_thumbnail: &DynamicImage) -> DynamicImage {
    let w = img_thumbnail.width();
    let h = img_thumbnail.height();

    let top = (h as f32 * 0.1).floor() as u32;
    let bot = (h as f32 * 0.4).floor() as u32;
    let left = (w as f32 * 0.25).floor() as u32;
    let right = (w as f32 * 0.1).floor() as u32;

    img_thumbnail.crop_imm(left, right, w - left - right, h - top - bot)
}
