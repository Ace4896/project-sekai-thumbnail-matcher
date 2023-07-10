use image::{imageops::FilterType, DynamicImage};

const HASH_IMAGE_SIZE: usize = 32;
const REDUCED_DCT2_SIZE: usize = 8;
const HASH_LENGTH: usize = REDUCED_DCT2_SIZE * REDUCED_DCT2_SIZE;

/// Generates a pHash for the specified card thumbnail.
///
/// The pHash algorithm is based on this article from [HackerFactor](https://www.hackerfactor.com/blog/?/archives/432-Looks-Like-It.html).
pub fn generate_thumbnail_phash(img_thumbnail: &DynamicImage) -> u64 {
    // Crop thumbnail to remove UI elements
    let img_crop = crop_thumbnail_for_hash(img_thumbnail);

    // Resize image for hashing
    let img_resized = img_crop.resize_exact(
        HASH_IMAGE_SIZE as u32,
        HASH_IMAGE_SIZE as u32,
        FilterType::Triangle, // Bilinear
    );

    // Convert to grayscale - f32 is needed for DCT-II
    let img_gray = img_resized.to_luma32f();
    let raw_gray = img_gray.as_raw();

    // Perform DCT-II on the image
    // The first value is set to 0.0 to exclude completely flat image information
    // TODO: This can definitely be optimised, just need something that works first
    let mut raw_dct2 = vec![0.0; HASH_IMAGE_SIZE * HASH_IMAGE_SIZE];
    let pi_over_n = std::f32::consts::PI / HASH_IMAGE_SIZE as f32;

    for k1 in 0..HASH_IMAGE_SIZE {
        for k2 in 0..HASH_IMAGE_SIZE {
            let mut sum: f32 = 0.0;

            for n1 in 0..HASH_IMAGE_SIZE {
                let mut n2_sum: f32 = 0.0;

                for n2 in 0..HASH_IMAGE_SIZE {
                    n2_sum += raw_gray[n1 * HASH_IMAGE_SIZE + n2] * (pi_over_n * (n2 as f32 + 0.5) * k2 as f32).cos();
                }

                sum += n2_sum * (pi_over_n * (n1 as f32 + 0.5) * k1 as f32).cos();
            }

            raw_dct2[k1 * HASH_IMAGE_SIZE + k2] = sum;
        }
    }

    raw_dct2[0] = 0.0;

    // Determine the average DCT-II value for the top-left 8x8
    let mut average: f32 = 0.0;

    for x in 0..REDUCED_DCT2_SIZE {
        for y in 0..REDUCED_DCT2_SIZE {
            average += raw_dct2[x * HASH_IMAGE_SIZE + y];
        }
    }

    average /= HASH_LENGTH as f32;

    // Construct the hash from the top-left 8x8
    // If the DCT-II value is above the average, then set it's bit in the hash to 1
    let mut hash: u64 = 0;

    for x in 0..REDUCED_DCT2_SIZE {
        for y in 0..REDUCED_DCT2_SIZE {
            if raw_dct2[x * HASH_IMAGE_SIZE + y] > average {
                hash |= 1 << (HASH_LENGTH - 1 - (x * REDUCED_DCT2_SIZE + y));
            }
        }
    }

    hash
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
