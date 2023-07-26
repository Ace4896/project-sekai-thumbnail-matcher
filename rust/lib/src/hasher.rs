use image::{imageops::FilterType, DynamicImage};
use ndarray::{s, Array2, ArrayView2};
use ndrustfft::DctHandler;

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
    let raw_gray =
        ArrayView2::from_shape((HASH_IMAGE_SIZE, HASH_IMAGE_SIZE), img_gray.as_raw()).unwrap();

    // Perform DCT-II on the image
    // The first value is set to 0.0 to exclude completely flat image information
    let mut raw_dct2: Array2<f32> = Array2::zeros(raw_gray.raw_dim());

    {
        let mut tmp: Array2<f32> = Array2::zeros(raw_gray.raw_dim());
        let mut dct2_handler_ax0 = DctHandler::<f32>::new(HASH_IMAGE_SIZE);
        let mut dct2_handler_ax1 = DctHandler::<f32>::new(HASH_IMAGE_SIZE);

        ndrustfft::nddct2_par(&raw_gray, &mut tmp, &mut dct2_handler_ax1, 1);
        ndrustfft::nddct2_par(&tmp, &mut raw_dct2, &mut dct2_handler_ax0, 0);
    }

    raw_dct2[[0, 0]] = 0.0;

    // Determine the average DCT-II value for the top-left 8x8
    let reduced_dct2 = raw_dct2.slice(s![0..REDUCED_DCT2_SIZE, 0..REDUCED_DCT2_SIZE]);
    let average = reduced_dct2.mean().unwrap();

    // Construct the hash from the top-left 8x8
    // If the DCT-II value is above the average, then set it's bit in the hash to 1
    reduced_dct2
        .indexed_iter()
        .filter(|(_, &dct2_val)| dct2_val > average)
        .fold(0u64, |hash, ((y, x), _)| {
            hash | (1 << (HASH_LENGTH - 1 - (x * REDUCED_DCT2_SIZE + y)))
        })
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

    img_thumbnail.crop_imm(left, top, w - left - right, h - top - bot)
}
