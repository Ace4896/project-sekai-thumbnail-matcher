use image::{DynamicImage, GenericImageView, GrayImage, Luma};
use imageproc::{contours::BorderType, contrast::ThresholdType, rect::Rect};

use crate::utils::{median, BoundingRect};

const COLOUR_WHITE: Luma<u8> = Luma([255]);

/// Extracts card thumbnails from a character list screenshot.
pub fn extract_thumbnail_images(img_list: &DynamicImage) -> Vec<DynamicImage> {
    let img_list_gray = img_list.to_luma8();

    extract_character_box(&img_list_gray)
        .map(|rect| extract_character_thumbnails(&img_list, &img_list_gray, &rect))
        .unwrap_or_default()
}

fn extract_character_box(img_character_list_gray: &GrayImage) -> Option<BoundingRect> {
    // Threshold the grayscale image to retain near-white pixels
    let img_thresh =
        imageproc::contrast::threshold(&img_character_list_gray, 250, ThresholdType::Binary);

    // Find the largest outer contour - this should be the character box
    let contours = imageproc::contours::find_contours::<u32>(&img_thresh);

    contours
        .iter()
        .filter(|contour| contour.border_type == BorderType::Outer)
        .max_by_key(|contour| {
            let rect = BoundingRect::from_points(&contour.points);
            rect.width() * rect.height()
        })
        .map(|max_contour| {
            // Approximate the contour to a rectangle
            // TODO: This approximation isn't working, it's still picking up some of the background around the character box
            let arc_length = imageproc::geometry::arc_length(&max_contour.points, true);
            let max_contour_approx = imageproc::geometry::approximate_polygon_dp(
                &max_contour.points,
                0.1 * arc_length,
                true,
            );

            BoundingRect::from_points(&max_contour_approx)
        })
}

fn extract_character_thumbnails(
    img_list: &DynamicImage,
    img_list_gray: &GrayImage,
    box_rect: &BoundingRect,
) -> Vec<DynamicImage> {
    // Find outer contours within the character box
    // This time, we want near-black pixels to form the contours, so use an inverse threshold
    let mut img_box_thresh = img_list_gray
        .view(
            box_rect.left,
            box_rect.top,
            box_rect.width(),
            box_rect.height(),
        )
        .to_image();

    imageproc::contrast::threshold_mut(&mut img_box_thresh, 250, ThresholdType::BinaryInverted);

    let initial_contours = imageproc::contours::find_contours::<u32>(&img_box_thresh);

    // These contours may be disjointed, so draw a new binary image from the outer contours and find contours again
    img_box_thresh = GrayImage::new(box_rect.width(), box_rect.height());

    for outer_contour in initial_contours
        .iter()
        .filter(|contour| contour.border_type == BorderType::Outer)
    {
        let rect = BoundingRect::from_points(&outer_contour.points);

        if rect.width() > 0 && rect.height() > 0 {
            imageproc::drawing::draw_filled_rect_mut(
                &mut img_box_thresh,
                Rect::at(rect.left as i32, rect.top as i32).of_size(rect.width(), rect.height()),
                COLOUR_WHITE,
            )
        }
    }

    let mut final_contours = imageproc::contours::find_contours::<u32>(&img_box_thresh)
        .iter()
        .filter(|contour| contour.border_type == BorderType::Outer)
        .map(|contour| BoundingRect::from_points(&contour.points))
        .collect::<Vec<_>>();

    // Determine the average width of each contours
    // Only need to consider square-like contours that have above-average width
    let widths = final_contours
        .iter()
        .map(|rect| rect.width())
        .collect::<Vec<_>>();

    let width_threshold = median(&widths) * 0.9;

    final_contours.retain(|rect| rect.width() as f64 > width_threshold && rect.is_square_like());

    // Finally, crop the thumbnail images from the original image
    let img_box = img_list.view(
        box_rect.left,
        box_rect.top,
        box_rect.width(),
        box_rect.height(),
    );

    final_contours
        .iter()
        .map(|rect| {
            img_box
                .view(rect.left, rect.top, rect.width(), rect.height())
                .to_image()
                .into()
        })
        .collect::<Vec<_>>()
}
