mod draw;
mod draw_mut;
mod draw_text;

use image::GenericImage;
use std::mem::swap;

pub use self::{draw_mut::DrawMut, draw_text::DrawText};
use super::{definitions, rect, weighted_sum};

// Set pixel at (x, y) to color if this point lies within image bounds,
// otherwise do nothing.
fn draw_if_in_bounds<I>(image: &mut I, x: i32, y: i32, color: I::Pixel)
where
    I: GenericImage,
{
    if x >= 0 && x < image.width() as i32 && y >= 0 && y < image.height() as i32 {
        image.put_pixel(x as u32, y as u32, color);
    }
}

/// 一个2D的点
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point<T> {
    /// x-coordinate.
    pub x: T,
    /// y-coordinate.
    pub y: T,
}

impl<T> Point<T> {
    /// Construct a point at (x, y).
    #[allow(dead_code)]
    pub fn new(x: T, y: T) -> Point<T> {
        Point::<T> { x, y }
    }
}

/// Iterates over the coordinates in a line segment using
/// [Bresenham's line drawing algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm).
/// 使用Bresenham's画线算法，生成绘制线段所需的点的迭代器
pub struct BresenhamLineIter {
    dx: f32,
    dy: f32,
    x: i32,
    y: i32,
    error: f32,
    end_x: i32,
    is_steep: bool,
    y_step: i32,
}

impl BresenhamLineIter {
    /// Creates a [`BresenhamLineIter`](struct.BresenhamLineIter.html) which will iterate over the integer coordinates
    /// between `start` and `end`.
    pub fn new(start: (f32, f32), end: (f32, f32)) -> BresenhamLineIter {
        let (mut x0, mut y0) = (start.0, start.1);
        let (mut x1, mut y1) = (end.0, end.1);

        let is_steep = (y1 - y0).abs() > (x1 - x0).abs();
        if is_steep {
            swap(&mut x0, &mut y0);
            swap(&mut x1, &mut y1);
        }

        if x0 > x1 {
            swap(&mut x0, &mut x1);
            swap(&mut y0, &mut y1);
        }

        let dx = x1 - x0;

        BresenhamLineIter {
            dx,
            dy: (y1 - y0).abs(),
            x: x0 as i32,
            y: y0 as i32,
            error: dx / 2f32,
            end_x: x1 as i32,
            is_steep,
            y_step: if y0 < y1 { 1 } else { -1 },
        }
    }
}

impl Iterator for BresenhamLineIter {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<(i32, i32)> {
        if self.x > self.end_x {
            None
        } else {
            let ret = if self.is_steep {
                (self.y, self.x)
            } else {
                (self.x, self.y)
            };

            self.x += 1;
            self.error -= self.dy;
            if self.error < 0f32 {
                self.y += self.y_step;
                self.error += self.dx;
            }

            Some(ret)
        }
    }
}

// Implements the Midpoint Ellipse Drawing Algorithm https://web.archive.org/web/20160128020853/http://tutsheap.com/c/mid-point-ellipse-drawing-algorithm/). (Modified from Bresenham's algorithm)
//
// Takes a function that determines how to render the points on the ellipse.
// 实现中点椭圆绘制算法(修改自布雷森汉姆算法)接受一个函数，用于确定如何在椭圆上渲染点。
fn draw_ellipse<F>(mut render_func: F, center: (i32, i32), width_radius: i32, height_radius: i32)
where
    F: FnMut(i32, i32, i32, i32),
{
    let (x0, y0) = center;
    let w2 = (width_radius * width_radius) as f32;
    let h2 = (height_radius * height_radius) as f32;
    let mut x = 0;
    let mut y = height_radius;
    let mut px = 0.0;
    let mut py = 2.0 * w2 * y as f32;

    render_func(x0, y0, x, y);

    // Top and bottom regions.
    let mut p = h2 - (w2 * height_radius as f32) + (0.25 * w2);
    while px < py {
        x += 1;
        px += 2.0 * h2;
        if p < 0.0 {
            p += h2 + px;
        } else {
            y -= 1;
            py += -2.0 * w2;
            p += h2 + px - py;
        }

        render_func(x0, y0, x, y);
    }

    // Left and right regions.
    p = h2 * (x as f32 + 0.5).powi(2) + (w2 * (y - 1).pow(2) as f32) - w2 * h2;
    while y > 0 {
        y -= 1;
        py += -2.0 * w2;
        if p > 0.0 {
            p += w2 - py;
        } else {
            x += 1;
            px += 2.0 * h2;
            p += w2 - py + px;
        }

        render_func(x0, y0, x, y);
    }
}

fn plot_wu_line<I, T, B>(
    mut plotter: Plotter<'_, I, T, B>,
    start: (i32, i32),
    end: (i32, i32),
    color: I::Pixel,
) where
    I: GenericImage,
    T: Fn(i32, i32) -> (i32, i32),
    B: Fn(I::Pixel, I::Pixel, f32) -> I::Pixel,
{
    let dx = end.0 - start.0;
    let dy = end.1 - start.1;
    let gradient = dy as f32 / dx as f32;
    let mut fy = start.1 as f32;

    for x in start.0..(end.0 + 1) {
        plotter.plot(x, fy as i32, color, 1.0 - fy.fract());
        plotter.plot(x, fy as i32 + 1, color, fy.fract());
        fy += gradient;
    }
}

struct Plotter<'a, I, T, B>
where
    I: GenericImage,
    T: Fn(i32, i32) -> (i32, i32),
    B: Fn(I::Pixel, I::Pixel, f32) -> I::Pixel,
{
    image: &'a mut I,
    transform: T,
    blend: B,
}

impl<I, T, B> Plotter<'_, I, T, B>
where
    I: GenericImage,
    T: Fn(i32, i32) -> (i32, i32),
    B: Fn(I::Pixel, I::Pixel, f32) -> I::Pixel,
{
    fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.image.width() as i32 && y >= 0 && y < self.image.height() as i32
    }

    pub fn plot(&mut self, x: i32, y: i32, line_color: I::Pixel, line_weight: f32) {
        let (x_trans, y_trans) = (self.transform)(x, y);
        if self.in_bounds(x_trans, y_trans) {
            let original = self.image.get_pixel(x_trans as u32, y_trans as u32);
            let blended = (self.blend)(line_color, original, line_weight);
            self.image
                .put_pixel(x_trans as u32, y_trans as u32, blended);
        }
    }
}
