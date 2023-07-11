use image::{imageops, DynamicImage, GenericImageView, GrayImage};
use imageproc::{contours::BorderType, point::Point, rect::Rect};

/// Extracts card thumbnails from a character list screenshot.
pub fn extract_thumbnail_images(img_list: &DynamicImage) -> Vec<DynamicImage> {
    let img_list_gray = img_list.to_luma8();

    extract_character_box(&img_list_gray)
        .map(|rect| extract_character_thumbnails(&img_list, &img_list_gray, &rect))
        .unwrap_or_default()
}

/// Finds the bounding rectangle for the specified points, without considering object rotation.
fn get_bounding_rect(points: &[Point<u32>]) -> Rect {
    let (min_x, min_y, max_x, max_y) = points.iter().fold(
        (u32::MAX, u32::MAX, u32::MIN, u32::MIN),
        |(left, top, right, bot), point| {
            (
                left.min(point.x),
                top.min(point.y),
                right.max(point.x),
                bot.max(point.y),
            )
        },
    );

    println!("({}, {}), ({}, {})", min_x, min_y, max_x, max_y);

    Rect::at(min_x as i32, min_y as i32).of_size(max_x - min_x, max_y - min_y)
}

fn extract_character_box(img_character_list_gray: &GrayImage) -> Option<Rect> {
    // Threshold the grayscale image to retain near-white pixels
    let img_thresh = imageproc::contrast::threshold(&img_character_list_gray, 250);

    // Find the largest outer contour - this should be the character box
    let contours = imageproc::contours::find_contours::<u32>(&img_thresh);

    // TODO: The width/height of a bounding rect has to be positive, so filter out anything that is too small
    contours
        .iter()
        .filter(|contour| contour.border_type == BorderType::Outer)
        .max_by_key(|contour| {
            let rect = get_bounding_rect(&contour.points);
            rect.width() * rect.height()
        })
        .map(|max_contour| {
            // Approximate the contour to a rectangle
            let arc_length = imageproc::geometry::arc_length(&max_contour.points, true);
            let max_contour_approx = imageproc::geometry::approximate_polygon_dp(
                &max_contour.points,
                0.1 * arc_length,
                true,
            );

            get_bounding_rect(&max_contour_approx)
        })
}

fn extract_character_thumbnails(
    img_list: &DynamicImage,
    img_list_gray: &GrayImage,
    box_rect: &Rect,
) -> Vec<DynamicImage> {
    let img_box_gray = img_list_gray
        .view(
            box_rect.left() as u32,
            box_rect.top() as u32,
            box_rect.width(),
            box_rect.height(),
        )
        .to_image();

    // Find outer contours within the character box
    // This time, we want near-black pixels to form the contours, so use an inverse threshold
    let mut img_box_thresh = imageproc::contrast::threshold(&img_box_gray, 5);
    imageops::invert(&mut img_box_thresh);

    let initial_contours = imageproc::contours::find_contours::<u32>(&img_box_thresh);

    // These contours may be disjointed, so draw a new binary image from the outer contours and find contours again
    img_box_thresh = GrayImage::new(img_box_gray.width(), img_box_gray.height());

    for outer_contour in initial_contours
        .iter()
        .filter(|contour| contour.border_type == BorderType::Outer)
    {
        let rect = get_bounding_rect(&outer_contour.points);

        imageproc::drawing::draw_filled_rect_mut(&mut img_box_thresh, rect, [255].into())
    }

    let mut final_contours = imageproc::contours::find_contours::<u32>(&img_box_thresh)
        .iter()
        .filter(|contour| contour.border_type == BorderType::Outer)
        .map(|contour| get_bounding_rect(&contour.points))
        .collect::<Vec<_>>();

    // Determine the average width of each contours
    // Only need to consider square-like contours that have above-average width
    let average_width = final_contours.iter().map(|rect| rect.width()).sum::<u32>() as f64
        / final_contours.len() as f64;

    final_contours.retain(|rect| {
        rect.width() as f64 > average_width
            && ((rect.width() as f64 - rect.height() as f64).abs() / rect.width() as f64) < 0.1
    });

    // Finally, crop the thumbnail images from the original image
    let img_box = img_list
        .view(
            box_rect.left() as u32,
            box_rect.top() as u32,
            box_rect.width(),
            box_rect.height(),
        )
        .to_image();

    // TODO: There's a lot of casting here, probably can be improved
    final_contours
        .iter()
        .map(|rect| {
            DynamicImage::ImageRgba8(
                imageops::crop_imm(
                    &img_box,
                    rect.left() as u32,
                    rect.top() as u32,
                    rect.width(),
                    rect.height(),
                )
                .to_image(),
            )
        })
        .collect::<Vec<_>>()
}
