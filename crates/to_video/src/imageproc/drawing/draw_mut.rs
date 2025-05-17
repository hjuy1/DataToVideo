#![allow(dead_code)]
use super::{
    draw_ellipse, draw_if_in_bounds, plot_wu_line, rect::Rect, BresenhamLineIter, Plotter, Point,
};
use image::GenericImage;
use std::{
    cmp::{max, min},
    mem::swap,
};

pub trait DrawMut: GenericImage + Sized {
    /// 在图像上绘制一条三次贝塞尔曲线。
    ///
    ///  绘制尽可能多的曲线部分，直至边界。
    ///
    /// Draws a cubic Bézier curve on an image in place.
    ///
    /// Draws as much of the curve as lies within image bounds.
    fn draw_cubic_bezier_curve_mut(
        &mut self,
        start: (f32, f32),
        end: (f32, f32),
        control_a: (f32, f32),
        control_b: (f32, f32),
        color: Self::Pixel,
    ) {
        // Bezier Curve function from: https://pomax.github.io/bezierinfo/#control
        let cubic_bezier_curve = |t: f32| {
            let t2 = t * t;
            let t3 = t2 * t;
            let mt = 1.0 - t;
            let mt2 = mt * mt;
            let mt3 = mt2 * mt;
            let x = (start.0 * mt3)
                + (3.0 * control_a.0 * mt2 * t)
                + (3.0 * control_b.0 * mt * t2)
                + (end.0 * t3);
            let y = (start.1 * mt3)
                + (3.0 * control_a.1 * mt2 * t)
                + (3.0 * control_b.1 * mt * t2)
                + (end.1 * t3);
            (x.round(), y.round()) // round to nearest pixel, to avoid ugly line artifacts
        };

        let distance = |point_a: (f32, f32), point_b: (f32, f32)| {
            ((point_a.0 - point_b.0).powi(2) + (point_a.1 - point_b.1).powi(2)).sqrt()
        };

        // Approximate curve's length by adding distance between control points.
        let curve_length_bound: f32 =
            distance(start, control_a) + distance(control_a, control_b) + distance(control_b, end);

        // Use hyperbola function to give shorter curves a bias in number of line segments.
        let num_segments: i32 = ((curve_length_bound.powi(2) + 800.0).sqrt() / 8.0) as i32;

        // Sample points along the curve and connect them with line segments.
        let t_interval = 1f32 / (num_segments as f32);
        let mut t1 = 0f32;
        for i in 0..num_segments {
            let t2 = (i as f32 + 1.0) * t_interval;
            self.draw_line_segment_mut(cubic_bezier_curve(t1), cubic_bezier_curve(t2), color);
            t1 = t2;
        }
    }

    /// 在图像上绘制空心椭圆的轮廓, 仅绘制位于图像边界内的椭圆轮廓
    ///
    /// 使用[中点椭圆绘制算法](https://web.archive.org/web/20160128020853/http://tutsheap.com/c/mid-point-ellipse-drawing-algorithm/)(修改自Bresenham算法)
    ///
    /// 椭圆是轴对齐的，并满足以下方程：`(x^2 / width_radius^2) + (y^2 / height_radius^2) = 1`
    ///
    /// Draws the outline of an ellipse on an image in place.
    ///
    /// Draws as much of an ellipse as lies inside the image bounds.
    ///
    /// Uses the [Midpoint Ellipse Drawing Algorithm](https://web.archive.org/web/20160128020853/http://tutsheap.com/c/mid-point-ellipse-drawing-algorithm/).
    /// (Modified from Bresenham's algorithm)
    ///
    /// The ellipse is axis-aligned and satisfies the following equation:
    ///
    /// `(x^2 / width_radius^2) + (y^2 / height_radius^2) = 1`
    fn draw_hollow_ellipse_mut(
        &mut self,
        center: (i32, i32),
        width_radius: i32,
        height_radius: i32,
        color: Self::Pixel,
    ) {
        // Circle drawing algorithm is faster, so use it if the given ellipse is actually a circle.
        if width_radius == height_radius {
            self.draw_hollow_circle_mut(center, width_radius, color);
            return;
        }

        let draw_quad_pixels = |x0: i32, y0: i32, x: i32, y: i32| {
            draw_if_in_bounds(self, x0 + x, y0 + y, color);
            draw_if_in_bounds(self, x0 - x, y0 + y, color);
            draw_if_in_bounds(self, x0 + x, y0 - y, color);
            draw_if_in_bounds(self, x0 - x, y0 - y, color);
        };

        draw_ellipse(draw_quad_pixels, center, width_radius, height_radius);
    }

    /// 在图像上绘制实心椭圆。仅绘制位于图像边界内的椭圆
    ///
    /// 使用[中点椭圆绘制算法](https://web.archive.org/web/20160128020853/http://tutsheap.com/c/mid-point-ellipse-drawing-algorithm/)(修改自布雷森汉姆算法)
    ///
    /// 椭圆是轴对齐的，并满足以下方程: `(x^2 / width_radius^2) + (y^2 / height_radius^2) <= 1`
    ///
    /// Draws an ellipse and its contents on an image in place.
    ///
    /// Draw as much of the ellipse and its contents as lies inside the image bounds.
    ///
    /// Uses the [Midpoint Ellipse Drawing Algorithm](https://web.archive.org/web/20160128020853/http://tutsheap.com/c/mid-point-ellipse-drawing-algorithm/).
    /// (Modified from Bresenham's algorithm)
    ///
    /// The ellipse is axis-aligned and satisfies the following equation:
    ///
    /// `(x^2 / width_radius^2) + (y^2 / height_radius^2) <= 1`
    fn draw_filled_ellipse_mut(
        &mut self,
        center: (i32, i32),
        width_radius: i32,
        height_radius: i32,
        color: Self::Pixel,
    ) {
        // Circle drawing algorithm is faster, so use it if the given ellipse is actually a circle.
        if width_radius == height_radius {
            self.draw_filled_circle_mut(center, width_radius, color);
            return;
        }

        let draw_line_pairs = |x0: i32, y0: i32, x: i32, y: i32| {
            self.draw_line_segment_mut(
                ((x0 - x) as f32, (y0 + y) as f32),
                ((x0 + x) as f32, (y0 + y) as f32),
                color,
            );
            self.draw_line_segment_mut(
                ((x0 - x) as f32, (y0 - y) as f32),
                ((x0 + x) as f32, (y0 - y) as f32),
                color,
            );
        };

        draw_ellipse(draw_line_pairs, center, width_radius, height_radius);
    }

    /// 在图像上绘制空心圆的轮廓。只绘制位于图像边界内的圆轮廓
    ///
    /// Draws the outline of a circle on an image in place.
    ///
    /// Draw as much of the circle as lies inside the image bounds.
    fn draw_hollow_circle_mut(&mut self, center: (i32, i32), radius: i32, color: Self::Pixel) {
        let mut x = 0i32;
        let mut y = radius;
        let mut p = 1 - radius;
        let x0 = center.0;
        let y0 = center.1;

        while x <= y {
            draw_if_in_bounds(self, x0 + x, y0 + y, color);
            draw_if_in_bounds(self, x0 + y, y0 + x, color);
            draw_if_in_bounds(self, x0 - y, y0 + x, color);
            draw_if_in_bounds(self, x0 - x, y0 + y, color);
            draw_if_in_bounds(self, x0 - x, y0 - y, color);
            draw_if_in_bounds(self, x0 - y, y0 - x, color);
            draw_if_in_bounds(self, x0 + y, y0 - x, color);
            draw_if_in_bounds(self, x0 + x, y0 - y, color);

            x += 1;
            if p < 0 {
                p += 2 * x + 1;
            } else {
                y -= 1;
                p += 2 * (x - y) + 1;
            }
        }
    }

    /// 在图像上绘制实心圆。只绘制位于图像边界内的圆
    ///
    /// Draws a circle and its contents on an image in place.
    ///
    /// Draws as much of a circle and its contents as lies inside the image bounds.
    fn draw_filled_circle_mut(&mut self, center: (i32, i32), radius: i32, color: Self::Pixel) {
        let mut x = 0i32;
        let mut y = radius;
        let mut p = 1 - radius;
        let x0 = center.0;
        let y0 = center.1;

        while x <= y {
            self.draw_line_segment_mut(
                ((x0 - x) as f32, (y0 + y) as f32),
                ((x0 + x) as f32, (y0 + y) as f32),
                color,
            );
            self.draw_line_segment_mut(
                ((x0 - y) as f32, (y0 + x) as f32),
                ((x0 + y) as f32, (y0 + x) as f32),
                color,
            );
            self.draw_line_segment_mut(
                ((x0 - x) as f32, (y0 - y) as f32),
                ((x0 + x) as f32, (y0 - y) as f32),
                color,
            );
            self.draw_line_segment_mut(
                ((x0 - y) as f32, (y0 - x) as f32),
                ((x0 + y) as f32, (y0 - x) as f32),
                color,
            );

            x += 1;
            if p < 0 {
                p += 2 * x + 1;
            } else {
                y -= 1;
                p += 2 * (x - y) + 1;
            }
        }
    }

    /// 在图像上绘制一个彩色十字。处理图像边界外的坐标。
    ///
    /// Draws a colored cross on an image in place.
    ///
    /// Handles coordinates outside image bounds.
    fn draw_cross_mut(&mut self, color: Self::Pixel, x: i32, y: i32) {
        let (width, height) = self.dimensions();
        let idx = |x, y| (3 * (y + 1) + x + 1) as usize;
        let stencil = [0u8, 1u8, 0u8, 1u8, 1u8, 1u8, 0u8, 1u8, 0u8];

        for sy in -1..2 {
            let iy = y + sy;
            if iy < 0 || iy >= height as i32 {
                continue;
            }

            for sx in -1..2 {
                let ix = x + sx;
                if ix < 0 || ix >= width as i32 {
                    continue;
                }

                if stencil[idx(sx, sy)] == 1u8 {
                    self.put_pixel(ix as u32, iy as u32, color);
                }
            }
        }
    }

    /// 在图像上绘制线段。绘制起点和终点之间位于图像边界内的线段部分
    ///
    /// 使用 [Bresenham'画线算法](https://en.wikipedia.org/wiki/Bresenham's_line_algorithm)
    ///
    /// Draws a line segment on an image in place.
    ///
    /// Draws as much of the line segment between start and end as lies inside the image bounds.
    ///
    /// Uses [Bresenham's line drawing algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm).
    fn draw_line_segment_mut(&mut self, start: (f32, f32), end: (f32, f32), color: Self::Pixel) {
        let (width, height) = self.dimensions();
        let in_bounds = |x, y| x >= 0 && x < width as i32 && y >= 0 && y < height as i32;

        let line_iterator = BresenhamLineIter::new(start, end);

        for point in line_iterator {
            let x = point.0;
            let y = point.1;

            if in_bounds(x, y) {
                self.put_pixel(x as u32, y as u32, color);
            }
        }
    }

    /// 在图像上绘制抗锯齿的线段。绘制起点和终点之间位于图像边界内的线段部分
    ///
    /// blend 的参数为(线条颜色，原始颜色，线条宽度)
    ///
    /// 考虑使用 [`interpolate`](fn.interpolate.html) 进行混合
    ///
    /// 使用 [Xu 的线条绘制算法](https://en.wikipedia.org/wiki/Xiaolin_Wu's_line_algorithm)
    ///
    /// Draws an antialised line segment on an image in place.
    ///
    /// Draws as much of the line segment between `start` and `end` as lies inside the image bounds.
    ///
    /// The parameters of blend are (line color, original color, line weight).
    /// Consider using [`interpolate`](fn.interpolate.html) for blend.
    ///
    /// Uses [Xu's line drawing algorithm](https://en.wikipedia.org/wiki/Xiaolin_Wu%27s_line_algorithm).
    fn draw_antialiased_line_segment_mut<B>(
        &mut self,
        start: (i32, i32),
        end: (i32, i32),
        color: Self::Pixel,
        blend: B,
    ) where
        B: Fn(Self::Pixel, Self::Pixel, f32) -> Self::Pixel;

    fn draw_polygon_with_mut<L>(&mut self, poly: &[Point<i32>], color: Self::Pixel, plotter: L)
    where
        L: Fn(&mut Self, (f32, f32), (f32, f32), Self::Pixel),
    {
        if poly.is_empty() {
            return;
        }
        if poly[0] == poly[poly.len() - 1] {
            panic!(
                "First point {:?} == last point {:?}",
                poly[0],
                poly[poly.len() - 1]
            );
        }

        let mut y_min = i32::MAX;
        let mut y_max = i32::MIN;
        for p in poly {
            y_min = min(y_min, p.y);
            y_max = max(y_max, p.y);
        }

        let (width, height) = self.dimensions();

        // Intersect polygon vertical range with image bounds
        y_min = max(0, min(y_min, height as i32 - 1));
        y_max = max(0, min(y_max, height as i32 - 1));

        let mut closed: Vec<Point<i32>> = poly.to_vec();
        closed.push(poly[0]);

        let edges: Vec<&[Point<i32>]> = closed.windows(2).collect();
        let mut intersections = Vec::new();

        for y in y_min..y_max + 1 {
            for edge in &edges {
                let p0 = edge[0];
                let p1 = edge[1];

                if p0.y <= y && p1.y >= y || p1.y <= y && p0.y >= y {
                    if p0.y == p1.y {
                        // Need to handle horizontal lines specially
                        intersections.push(p0.x);
                        intersections.push(p1.x);
                    } else if p0.y == y || p1.y == y {
                        if p1.y > y {
                            intersections.push(p0.x);
                        }
                        if p0.y > y {
                            intersections.push(p1.x);
                        }
                    } else {
                        let fraction = (y - p0.y) as f32 / (p1.y - p0.y) as f32;
                        let inter = p0.x as f32 + fraction * (p1.x - p0.x) as f32;
                        intersections.push(inter.round() as i32);
                    }
                }
            }

            intersections.sort_unstable();
            intersections.chunks(2).for_each(|range| {
                let mut from = min(range[0], width as i32);
                let mut to = min(range[1], width as i32 - 1);
                if from < width as i32 && to >= 0 {
                    // draw only if range appears on the image
                    from = max(0, from);
                    to = max(0, to);

                    for x in from..to + 1 {
                        self.put_pixel(x as u32, y as u32, color);
                    }
                }
            });

            intersections.clear();
        }

        for edge in &edges {
            let start = (edge[0].x as f32, edge[0].y as f32);
            let end = (edge[1].x as f32, edge[1].y as f32);
            plotter(self, start, end, color);
        }
    }

    /// 在图像上绘制一个实心多边形。仅绘制位于图像边界内的多边形
    ///
    /// 提供的点列表应为开放路径，即第一个和最后一个点不能相等。最后一个点到第一个点会自动连接。
    ///
    /// Draws a polygon and its contents on an image in place.
    ///
    /// Draws as much of a filled polygon as lies within image bounds. The provided
    /// list of points should be an open path, i.e. the first and last points must not be equal.
    /// An implicit edge is added from the last to the first point in the slice.
    fn draw_polygon_mut(&mut self, poly: &[Point<i32>], color: Self::Pixel) {
        self.draw_polygon_with_mut(poly, color, |image, start, end, color| {
            image.draw_line_segment_mut(start, end, color)
        });
    }

    /// 在图像上绘制抗锯齿的实心多边形。仅绘制位于图像边界内的多边形
    ///
    /// 提供的点列表应该是一个开放路径，即第一个和最后一个点不能相等。最后一个点到第一个点会自动连接
    ///
    /// blend的参数是(线条颜色，原始颜色，线条粗细), 考虑使用[`interpolate`](fn.interpolate.html) 来进行混合。
    ///
    /// Draws an anti-aliased polygon and its contents on an image in place.
    ///
    /// Draws as much of a filled polygon as lies within image bounds. The provided
    /// list of points should be an open path, i.e. the first and last points must not be equal.
    /// An implicit edge is added from the last to the first point in the slice.
    ///
    /// The parameters of blend are (line color, original color, line weight).
    /// Consider using [`interpolate`](fn.interpolate.html) for blend.
    fn draw_antialiased_polygon_mut<B>(
        &mut self,
        poly: &[Point<i32>],
        color: Self::Pixel,
        blend: B,
    ) where
        B: Fn(Self::Pixel, Self::Pixel, f32) -> Self::Pixel;

    /// 在图像上绘制多边形的轮廓线。仅绘制位于图像边界内的多边形的轮廓
    ///
    /// 提供的点列表应按多边形顺序排列，并且应为开放路径，即第一个和最后一个点不得相等
    ///
    /// 多边形的边将按提供的顺序绘制，最后一个点到第一个点会自动连接
    ///
    /// Draws the outline of a polygon on an image in place.
    ///
    /// Draws as much of the outline of the polygon as lies within image bounds. The provided
    /// list of points should be in polygon order and be an open path, i.e. the first
    /// and last points must not be equal. The edges of the polygon will be drawn in the order
    /// that they are provided, and an implicit edge will be added from the last to the first
    /// point in the slice.
    fn draw_hallow_polygon_mut(&mut self, poly: &[Point<f32>], color: Self::Pixel) {
        if poly.is_empty() {
            return;
        }
        if poly.len() < 2 {
            panic!(
                "Polygon only has {} points, but at least two are needed.",
                poly.len(),
            );
        }
        if poly[0] == poly[poly.len() - 1] {
            panic!(
                "First point {:?} == last point {:?}",
                poly[0],
                poly[poly.len() - 1]
            );
        }
        for window in poly.windows(2) {
            self.draw_line_segment_mut(
                (window[0].x, window[0].y),
                (window[1].x, window[1].y),
                color,
            );
        }
        let first = poly[0];
        let last = poly.iter().last().unwrap();
        self.draw_line_segment_mut((first.x, first.y), (last.x, last.y), color);
    }

    /// 在图像上绘制矩形的轮廓。仅绘制在图像边界内的矩形的轮廓。
    ///
    /// Draws the outline of a rectangle on an image in place.
    ///
    /// Draws as much of the boundary of the rectangle as lies inside the image bounds.
    fn draw_hollow_rect_mut(&mut self, rect: Rect, color: Self::Pixel) {
        let left = rect.left() as f32;
        let right = rect.right() as f32;
        let top = rect.top() as f32;
        let bottom = rect.bottom() as f32;

        self.draw_line_segment_mut((left, top), (right, top), color);
        self.draw_line_segment_mut((left, bottom), (right, bottom), color);
        self.draw_line_segment_mut((left, top), (left, bottom), color);
        self.draw_line_segment_mut((right, top), (right, bottom), color);
    }

    /// 在图像上绘制实心矩形。仅绘制在图像边界内的矩形。
    ///
    /// Draws a rectangle and its contents on an image in place.
    ///
    /// Draws as much of the rectangle and its contents as lies inside the image bounds.
    fn draw_filled_rect_mut(&mut self, rect: Rect, color: Self::Pixel) {
        let image_bounds = Rect::at(0, 0).of_size(self.width(), self.height());
        if let Some(intersection) = image_bounds.intersect(rect) {
            for dy in 0..intersection.height() {
                for dx in 0..intersection.width() {
                    let x = intersection.left() as u32 + dx;
                    let y = intersection.top() as u32 + dy;
                    self.put_pixel(x, y, color);
                }
            }
        }
    }

    fn draw_hollow_rounded_rect_mut(&mut self, _rect: Rect, _radius: i32, _color: Self::Pixel) {
        todo!()
        // let left = rect.left() as f32;
        // let right = rect.right() as f32;
        // let top = rect.top() as f32;
        // let bottom = rect.bottom() as f32;

        // self.draw_line_segment_mut((left, top), (right, top), &color);
        // self.draw_line_segment_mut((left, bottom), (right, bottom), &color);
        // self.draw_line_segment_mut((left, top), (left, bottom), &color);
        // self.draw_line_segment_mut((right, top), (right, bottom), &color);
    }

    fn draw_filled_rounded_rect_mut(&mut self, rect: Rect, radius: i32, color: Self::Pixel) {
        let (left, right, top, bottom) = (rect.left(), rect.right(), rect.top(), rect.bottom());
        // 绘制四个圆角
        self.draw_filled_circle_mut((left + radius, top + radius), radius, color);
        self.draw_filled_circle_mut((left + radius, bottom - radius), radius, color);
        self.draw_filled_circle_mut((right - radius, top + radius), radius, color);
        self.draw_filled_circle_mut((right - radius, bottom - radius), radius, color);

        // 绘制矩形的顶部和底部，去除圆角部分
        self.draw_filled_rect_mut(
            Rect::at(left, top + radius).of_size(rect.width(), rect.height() - 2 * radius as u32),
            color,
        );
        // 绘制矩形的左侧和右侧，去除圆角部分
        self.draw_filled_rect_mut(
            Rect::at(left + radius, top).of_size(rect.width() - 2 * radius as u32, rect.height()),
            color,
        );
    }
}

impl<I: GenericImage> DrawMut for I {
    fn draw_antialiased_line_segment_mut<B>(
        &mut self,
        start: (i32, i32),
        end: (i32, i32),
        color: Self::Pixel,
        blend: B,
    ) where
        B: Fn(Self::Pixel, Self::Pixel, f32) -> Self::Pixel,
    {
        let (mut x0, mut y0) = (start.0, start.1);
        let (mut x1, mut y1) = (end.0, end.1);

        let is_steep = (y1 - y0).abs() > (x1 - x0).abs();

        if is_steep {
            if y0 > y1 {
                swap(&mut x0, &mut x1);
                swap(&mut y0, &mut y1);
            }
            let plotter = Plotter {
                image: self,
                transform: |x, y| (y, x),
                blend,
            };
            plot_wu_line(plotter, (y0, x0), (y1, x1), color);
        } else {
            if x0 > x1 {
                swap(&mut x0, &mut x1);
                swap(&mut y0, &mut y1);
            }
            let plotter = Plotter {
                image: self,
                transform: |x, y| (x, y),
                blend,
            };
            plot_wu_line(plotter, (x0, y0), (x1, y1), color);
        };
    }

    fn draw_antialiased_polygon_mut<B>(&mut self, poly: &[Point<i32>], color: Self::Pixel, blend: B)
    where
        B: Fn(Self::Pixel, Self::Pixel, f32) -> Self::Pixel,
    {
        self.draw_polygon_with_mut(poly, color, |image, start, end, color| {
            image.draw_antialiased_line_segment_mut(
                (start.0 as i32, start.1 as i32),
                (end.0 as i32, end.1 as i32),
                color,
                &blend,
            )
        });
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use image::{Rgba, RgbaImage};

    fn save(image: &RgbaImage, name: &str) {
        image
            .save(format!("./src/imageproc/drawing/test/{}.png", name))
            .expect("save image error");
    }

    #[test]
    #[ignore]
    fn test_draw_cubic_bezier_curve_mut() {
        let mut img = RgbaImage::new(100, 100);
        let color = Rgba([255, 0, 0, 255]);
        img.draw_cubic_bezier_curve_mut((50.0, 50.0), (0.0, 0.0), (2.0, 2.0), (3.0, 3.0), color);
        save(&img, "draw_cubic_bezier_curve_mut");

        assert_eq!(img.get_pixel(50, 30), &color);
        assert_eq!(img.get_pixel(70, 50), &color);
        assert_eq!(img.get_pixel(50, 70), &color);
        assert_eq!(img.get_pixel(30, 50), &color);
    }

    #[test]
    fn test_draw_hollow_ellipse_mut() {
        let mut img = RgbaImage::new(100, 100);
        let color = Rgba([0, 255, 255, 255]);
        img.draw_hollow_ellipse_mut((50, 50), 30, 20, color);
        save(&img, "draw_hollow_ellipse_mut");

        assert_eq!(img.get_pixel(50, 30), &color);
        assert_eq!(img.get_pixel(80, 50), &color);
        assert_eq!(img.get_pixel(50, 70), &color);
        assert_eq!(img.get_pixel(20, 50), &color);
    }

    #[test]
    fn test_draw_filled_ellipse_mut() {
        let mut img = RgbaImage::new(100, 100);
        let color = Rgba([255, 128, 0, 255]);
        img.draw_filled_ellipse_mut((50, 50), 30, 20, color);
        save(&img, "draw_filled_ellipse_mut");

        assert_eq!(img.get_pixel(50, 50), &color);
        assert_eq!(img.get_pixel(50, 30), &color);
        assert_eq!(img.get_pixel(80, 50), &color);
        assert_eq!(img.get_pixel(50, 70), &color);
    }

    #[test]
    fn test_draw_hollow_circle_mut() {
        let mut img = RgbaImage::new(100, 100);
        let color = Rgba([255, 0, 0, 255]);
        img.draw_hollow_circle_mut((50, 50), 20, color);
        save(&img, "draw_hollow_circle_mut");

        assert_eq!(img.get_pixel(50, 30), &color);
        assert_eq!(img.get_pixel(70, 50), &color);
        assert_eq!(img.get_pixel(50, 70), &color);
        assert_eq!(img.get_pixel(30, 50), &color);
    }

    #[test]
    fn test_draw_filled_circle_mut() {
        let mut img = RgbaImage::new(100, 100);
        let color = Rgba([255, 0, 0, 255]);
        img.draw_filled_circle_mut((50, 50), 20, color);
        save(&img, "draw_filled_circle_mut");

        assert_eq!(img.get_pixel(50, 50), &color);
        assert_eq!(img.get_pixel(50, 30), &color);
        assert_eq!(img.get_pixel(70, 50), &color);
        assert_eq!(img.get_pixel(50, 70), &color);
    }

    #[test]
    fn test_draw_cross_mut() {
        let mut img = RgbaImage::new(100, 100);
        let color = Rgba([255, 0, 0, 255]);
        img.draw_cross_mut(color, 40, 50);
        save(&img, "draw_cross_mut");

        assert_eq!(img.get_pixel(40, 50), &color);
        assert_eq!(img.get_pixel(39, 50), &color);
        assert_eq!(img.get_pixel(41, 50), &color);
        assert_eq!(img.get_pixel(40, 49), &color);
        assert_eq!(img.get_pixel(40, 51), &color);
    }

    #[test]
    fn test_draw_line_segment_mut() {
        let mut img = RgbaImage::new(100, 100);
        let color = Rgba([255, 0, 0, 255]);
        img.draw_line_segment_mut((10.0, 10.0), (90.0, 90.0), color);
        save(&img, "draw_line_segment_mut");

        assert_eq!(img.get_pixel(10, 10), &color);
        assert_eq!(img.get_pixel(50, 50), &color);
        assert_eq!(img.get_pixel(90, 90), &color);
    }

    // #[test]
    // fn test_draw_antialiased_line_segment_mut() {
    //     let mut img = RgbaImage::new(100, 100);
    //     let color = Rgba([255, 0, 0, 255]);
    //     img.draw_antialiased_line_segment_mut((10.0, 10.0), (90.0, 90.0), color);
    //     save(&img, "draw_antialiased_line_segment_mut");
    //     assert_eq!(img.get_pixel(10, 10), &color);
    //     assert_eq!(img.get_pixel(50, 50), &color);
    //     assert_eq!(img.get_pixel(90, 90), &color);
    // }

    #[test]
    #[ignore]
    fn test_draw_polygon_mut() {
        let mut img = RgbaImage::new(100, 100);
        let color = Rgba([255, 255, 0, 255]);
        let points = [Point::new(73, 5), Point::new(20, 65), Point::new(35, 86)];
        img.draw_polygon_mut(&points, color);
        save(&img, "draw_polygon_mut");

        assert_eq!(img.get_pixel(30, 30), &color);
        assert_eq!(img.get_pixel(49, 30), &color);
        assert_eq!(img.get_pixel(30, 49), &color);
        assert_eq!(img.get_pixel(49, 49), &color);
    }

    // #[test]
    // fn test_draw_antialiased_polygon_mut() {
    //     let mut img = RgbaImage::new(100, 100);
    //     let color = Rgba([255, 255, 0, 255]);
    //     let points = [Point::new(73, 5), Point::new(20, 65), Point::new(35, 86)];
    //     img.draw_antialiased_polygon_mut(&points, color);
    //     save(&img, "draw_antialiased_polygon_mut");
    //     assert_eq!(img.get_pixel(30, 30), &color);
    //     assert_eq!(img.get_pixel(49, 30), &color);
    //     assert_eq!(img.get_pixel(30, 49), &color);
    //     assert_eq!(img.get_pixel(49, 49), &color);
    // }

    #[test]
    #[ignore]
    fn test_draw_hallow_polygon_mut() {
        let mut img = RgbaImage::new(100, 100);
        let color = Rgba([255, 0, 0, 255]);
        let points = [
            Point::new(73.0, 5.0),
            Point::new(20.0, 65.0),
            Point::new(35.0, 86.0),
        ];
        img.draw_hallow_polygon_mut(&points, color);
        save(&img, "draw_hallow_polygon_mut");

        assert_eq!(img.get_pixel(20, 20), &color);
        assert_eq!(img.get_pixel(59, 20), &color);
        assert_eq!(img.get_pixel(20, 59), &color);
        assert_eq!(img.get_pixel(59, 59), &color);
    }

    #[test]
    fn test_draw_hollow_rect_mut() {
        let mut img = RgbaImage::new(100, 100);
        let color = Rgba([255, 0, 0, 255]);
        let rect = Rect::at(20, 20).of_size(40, 40);
        img.draw_hollow_rect_mut(rect, color);
        save(&img, "draw_hollow_rect_mut");

        assert_eq!(img.get_pixel(20, 20), &color);
        assert_eq!(img.get_pixel(59, 20), &color);
        assert_eq!(img.get_pixel(20, 59), &color);
        assert_eq!(img.get_pixel(59, 59), &color);
    }

    #[test]
    fn test_draw_filled_rect_mut() {
        let mut img = RgbaImage::new(100, 100);
        let color = Rgba([255, 255, 0, 255]);
        let rect = Rect::at(30, 30).of_size(20, 20);
        img.draw_filled_rect_mut(rect, color);
        save(&img, "draw_filled_rect_mut");

        assert_eq!(img.get_pixel(30, 30), &color);
        assert_eq!(img.get_pixel(49, 30), &color);
        assert_eq!(img.get_pixel(30, 49), &color);
        assert_eq!(img.get_pixel(49, 49), &color);
    }

    #[test]
    #[ignore]
    fn test_draw_hollow_rounded_rect_mut() {
        let mut img = RgbaImage::new(100, 100);
        let color = Rgba([255, 0, 0, 255]);
        let rect = Rect::at(20, 20).of_size(40, 40);
        img.draw_hollow_rounded_rect_mut(rect, 8, color);
        save(&img, "draw_hollow_rounded_rect_mut");

        assert_eq!(img.get_pixel(20, 20), &color);
        assert_eq!(img.get_pixel(59, 20), &color);
        assert_eq!(img.get_pixel(20, 59), &color);
        assert_eq!(img.get_pixel(59, 59), &color);
    }

    #[test]
    #[ignore]
    fn test_draw_filled_rounded_rect_mut() {
        let mut img = RgbaImage::new(100, 100);
        let color = Rgba([255, 255, 0, 255]);
        let rect = Rect::at(30, 30).of_size(50, 50);
        img.draw_filled_rounded_rect_mut(rect, 8, color);
        save(&img, "draw_filled_rounded_rect_mut");

        assert_eq!(img.get_pixel(30, 30), &color);
        assert_eq!(img.get_pixel(49, 30), &color);
        assert_eq!(img.get_pixel(30, 49), &color);
        assert_eq!(img.get_pixel(49, 49), &color);
    }
}
