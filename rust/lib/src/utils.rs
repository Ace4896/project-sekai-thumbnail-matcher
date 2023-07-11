use imageproc::point::Point;

/// A bounding rectangle.
#[derive(Debug)]
pub struct BoundingRect {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
}

impl BoundingRect {
    /// Gets the width of this bounding rectangle.
    pub fn width(&self) -> u32 {
        self.right - self.left
    }

    /// Gets the height of this bounding rectangle.
    pub fn height(&self) -> u32 {
        self.bottom - self.top
    }

    /// Determines the bounding rectangle from a set of points, ignoring object rotation.
    pub fn from_points(points: &[Point<u32>]) -> Self {
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

        Self {
            left: min_x,
            top: min_y,
            right: max_x,
            bottom: max_y,
        }
    }
}

// TODO: I'm pretty sure there's an O(n) algorithm
/// Calculates the median for the given numbers.
pub fn median(nums: &[u32]) -> f64 {
    let mut sorted = nums.to_vec();
    sorted.sort_unstable();

    if sorted.len() % 2 == 0 {
        (sorted[sorted.len() / 2] + sorted[sorted.len() / 2]) as f64 / 2.0
    } else {
        sorted[sorted.len() / 2] as f64
    }
}
