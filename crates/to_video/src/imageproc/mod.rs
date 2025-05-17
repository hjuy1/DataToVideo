// [package]
// authors = ["theotherphil"]
// description = "Image processing operations"
// edition = "2021"
// exclude = [".github/*", "examples/*", "tests/*"]
// homepage = "https://github.com/image-rs/imageproc"
// license = "MIT"
// name = "imageproc"
// readme = "README.md"
// repository = "https://github.com/image-rs/imageproc.git"
// rust-version = "1.70.0"
// version = "0.25.0"
pub mod definitions;
pub mod drawing;
pub mod rect;

use definitions::Clamp;
use image::Pixel;

/// Adds pixels with the given weights. Results are clamped to prevent arithmetical overflows.
///
/// # Examples
/// ```
/// # extern crate image;
/// # extern crate imageproc;
/// # fn main() {
/// use image::Rgb;
/// use imageproc::pixelops::weighted_sum;
///
/// let left = Rgb([10u8, 20u8, 30u8]);
/// let right = Rgb([100u8, 80u8, 60u8]);
///
/// let sum = weighted_sum(left, right, 0.7, 0.3);
/// assert_eq!(sum, Rgb([37, 38, 39]));
/// # }
/// ```
pub fn weighted_sum<P: Pixel>(left: P, right: P, left_weight: f32, right_weight: f32) -> P
where
    P::Subpixel: Into<f32> + Clamp<f32>,
{
    left.map2(&right, |p, q| {
        weighted_channel_sum(p, q, left_weight, right_weight)
    })
}

#[inline(always)]
fn weighted_channel_sum<C>(left: C, right: C, left_weight: f32, right_weight: f32) -> C
where
    C: Into<f32> + Clamp<f32>,
{
    Clamp::clamp(left.into() * left_weight + right.into() * right_weight)
}
