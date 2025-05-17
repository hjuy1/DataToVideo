use super::{definitions::Image, rect::Rect, DrawMut, Point};
use image::{GenericImage, ImageBuffer};

#[allow(unused)]
pub trait Draw: GenericImage + Sized {
    /// 在复制的新图像上绘制一条三次贝塞尔曲线。
    ///
    ///  绘制尽可能多的曲线部分，直至边界。
    ///
    /// Draws a cubic Bézier curve on a new copy of an image.
    ///
    /// Draws as much of the curve as lies within image bounds.
    #[must_use = "the function does not modify the original image"]
    fn draw_cubic_bezier_curve(
        &self,
        start: (f32, f32),
        end: (f32, f32),
        control_a: (f32, f32),
        control_b: (f32, f32),
        color: Self::Pixel,
    ) -> Image<Self::Pixel> {
        let mut out = ImageBuffer::new(self.width(), self.height());
        out.copy_from(self, 0, 0).unwrap();
        out.draw_cubic_bezier_curve_mut(start, end, control_a, control_b, color);
        out
    }

    /// 在复制的新图像上绘制空心椭圆的轮廓, 仅绘制位于图像边界内的椭圆轮廓
    ///
    /// 使用[中点椭圆绘制算法](https://web.archive.org/web/20160128020853/http://tutsheap.com/c/mid-point-ellipse-drawing-algorithm/)(修改自Bresenham算法)
    ///
    /// 椭圆是轴对齐的，并满足以下方程：`(x^2 / width_radius^2) + (y^2 / height_radius^2) = 1`
    ///
    /// Draws the outline of an ellipse on a new copy of an image.
    ///
    /// Draws as much of an ellipse as lies inside the image bounds.
    ///
    /// Uses the [Midpoint Ellipse Drawing Algorithm](https://web.archive.org/web/20160128020853/http://tutsheap.com/c/mid-point-ellipse-drawing-algorithm/).
    /// (Modified from Bresenham's algorithm)
    ///
    /// The ellipse is axis-aligned and satisfies the following equation:
    ///
    /// (`x^2 / width_radius^2) + (y^2 / height_radius^2) = 1`
    #[must_use = "the function does not modify the original image"]
    fn draw_hollow_ellipse(
        &self,
        center: (i32, i32),
        width_radius: i32,
        height_radius: i32,
        color: Self::Pixel,
    ) -> Image<Self::Pixel> {
        let mut out = ImageBuffer::new(self.width(), self.height());
        out.copy_from(self, 0, 0).unwrap();
        out.draw_hollow_ellipse_mut(center, width_radius, height_radius, color);
        out
    }

    /// 在复制的新图像上绘制实心椭圆。仅绘制位于图像边界内的椭圆
    ///
    /// 使用[中点椭圆绘制算法](https://web.archive.org/web/20160128020853/http://tutsheap.com/c/mid-point-ellipse-drawing-algorithm/)(修改自布雷森汉姆算法)
    ///
    /// 椭圆是轴对齐的，并满足以下方程: `(x^2 / width_radius^2) + (y^2 / height_radius^2) <= 1`
    ///
    /// Draws an ellipse and its contents on a new copy of the image.
    ///
    /// Draw as much of the ellipse and its contents as lies inside the image bounds.
    ///
    /// Uses the [Midpoint Ellipse Drawing Algorithm](https://web.archive.org/web/20160128020853/http://tutsheap.com/c/mid-point-ellipse-drawing-algorithm/).
    /// (Modified from Bresenham's algorithm)
    ///
    /// The ellipse is axis-aligned and satisfies the following equation:
    ///
    /// `(x^2 / width_radius^2) + (y^2 / height_radius^2) <= 1`
    #[must_use = "the function does not modify the original image"]
    fn draw_filled_ellipse(
        &self,
        center: (i32, i32),
        width_radius: i32,
        height_radius: i32,
        color: Self::Pixel,
    ) -> Image<Self::Pixel> {
        let mut out = ImageBuffer::new(self.width(), self.height());
        out.copy_from(self, 0, 0).unwrap();
        out.draw_filled_ellipse_mut(center, width_radius, height_radius, color);
        out
    }

    /// 在复制的新图像上绘制空心圆的轮廓。只绘制位于图像边界内的圆轮廓
    ///
    /// Draws the outline of a circle on a new copy of an image.
    ///
    /// Draw as much of the circle as lies inside the image bounds.
    #[must_use = "the function does not modify the original image"]
    fn draw_hollow_circle(
        &self,
        center: (i32, i32),
        radius: i32,
        color: Self::Pixel,
    ) -> Image<Self::Pixel> {
        let mut out = ImageBuffer::new(self.width(), self.height());
        out.copy_from(self, 0, 0).unwrap();
        out.draw_hollow_circle_mut(center, radius, color);
        out
    }

    /// 在复制的新图像上绘制实心圆。只绘制位于图像边界内的圆
    ///
    /// Draws a circle and its contents on a new copy of the image.
    ///
    /// Draws as much of a circle and its contents as lies inside the image bounds.
    #[must_use = "the function does not modify the original image"]
    fn draw_filled_circle(
        &self,
        center: (i32, i32),
        radius: i32,
        color: Self::Pixel,
    ) -> Image<Self::Pixel> {
        let mut out = ImageBuffer::new(self.width(), self.height());
        out.copy_from(self, 0, 0).unwrap();
        out.draw_filled_circle_mut(center, radius, color);
        out
    }

    /// 在复制的新图像上绘制一个彩色十字。处理图像边界外的坐标。
    ///
    /// Draws a colored cross on a new copy of an image.
    ///
    /// Handles coordinates outside image bounds.
    #[must_use = "the function does not modify the original image"]
    fn draw_cross(&self, color: Self::Pixel, x: i32, y: i32) -> Image<Self::Pixel> {
        let mut out = ImageBuffer::new(self.width(), self.height());
        out.copy_from(self, 0, 0).unwrap();
        out.draw_cross_mut(color, x, y);
        out
    }

    /// 在复制的新图像上绘制线段。绘制起点和终点之间位于图像边界内的线段部分
    ///
    /// 使用 [Bresenham'画线算法](https://en.wikipedia.org/wiki/Bresenham's_line_algorithm)
    ///
    /// Draws a line segment on a new copy of an image.
    ///
    /// Draws as much of the line segment between start and end as lies inside the image bounds.
    ///
    /// Uses [Bresenham's line drawing algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm).
    #[must_use = "the function does not modify the original image"]
    fn draw_line_segment(
        &self,
        start: (f32, f32),
        end: (f32, f32),
        color: Self::Pixel,
    ) -> Image<Self::Pixel> {
        let mut out = ImageBuffer::new(self.width(), self.height());
        out.copy_from(self, 0, 0).unwrap();
        out.draw_line_segment_mut(start, end, color);
        out
    }

    /// 在复制的新图像上绘制抗锯齿的线段。绘制起点和终点之间位于图像边界内的线段部分
    ///
    /// blend 的参数为(线条颜色，原始颜色，线条宽度)
    ///
    /// 考虑使用 [`interpolate`](fn.interpolate.html) 进行混合
    ///
    /// 使用 [Xu 的线条绘制算法](https://en.wikipedia.org/wiki/Xiaolin_Wu's_line_algorithm)
    ///
    /// Draws an antialised line segment on a new copy of an image.
    ///
    /// Draws as much of the line segment between `start` and `end` as lies inside the image bounds.
    ///
    /// The parameters of blend are (line color, original color, line weight).
    /// Consider using [`interpolate`](fn.interpolate.html) for blend.
    ///
    /// Uses [Xu's line drawing algorithm](https://en.wikipedia.org/wiki/Xiaolin_Wu%27s_line_algorithm).
    #[must_use = "the function does not modify the original image"]
    fn draw_antialiased_line_segment<B>(
        &self,
        start: (i32, i32),
        end: (i32, i32),
        color: Self::Pixel,
        blend: B,
    ) -> Image<Self::Pixel>
    where
        B: Fn(Self::Pixel, Self::Pixel, f32) -> Self::Pixel,
    {
        let mut out = ImageBuffer::new(self.width(), self.height());
        out.copy_from(self, 0, 0).unwrap();
        out.draw_antialiased_line_segment_mut(start, end, color, blend);
        out
    }

    #[must_use = "the function does not modify the original image"]
    fn draw_polygon_with<L>(
        &self,
        poly: &[Point<i32>],
        color: Self::Pixel,
        plotter: L,
    ) -> Image<Self::Pixel>
    where
        L: Fn(&mut Image<Self::Pixel>, (f32, f32), (f32, f32), Self::Pixel),
    {
        let mut out = ImageBuffer::new(self.width(), self.height());
        out.copy_from(self, 0, 0).unwrap();
        out.draw_polygon_with_mut(poly, color, plotter);
        out
    }

    /// 在复制的新图像上绘制一个实心多边形。仅绘制位于图像边界内的多边形
    ///
    /// 提供的点列表应为开放路径，即第一个和最后一个点不能相等。最后一个点到第一个点会自动连接。
    ///
    /// Draws a polygon and its contents on a new copy of an image.
    ///
    /// Draws as much of a filled polygon as lies within image bounds. The provided
    /// list of points should be an open path, i.e. the first and last points must not be equal.
    /// An implicit edge is added from the last to the first point in the slice.
    fn draw_polygon(&self, poly: &[Point<i32>], color: Self::Pixel) -> Image<Self::Pixel> {
        self.draw_polygon_with(poly, color, |image, start, end, color| {
            image.draw_line_segment_mut(start, end, color)
        })
    }

    /// 在复制的新图像上绘制抗锯齿的实心多边形。仅绘制位于图像边界内的多边形
    ///
    /// 提供的点列表应该是一个开放路径，即第一个和最后一个点不能相等。最后一个点到第一个点会自动连接
    ///
    /// blend的参数是(线条颜色，原始颜色，线条粗细), 考虑使用[`interpolate`](fn.interpolate.html) 来进行混合。
    ///
    /// Draws an anti-aliased polygon polygon and its contents on a new copy of an image.
    ///
    /// Draws as much of a filled polygon as lies within image bounds. The provided
    /// list of points should be an open path, i.e. the first and last points must not be equal.
    /// An implicit edge is added from the last to the first point in the slice.
    ///
    /// The parameters of blend are (line color, original color, line weight).
    /// Consider using [`interpolate`](fn.interpolate.html) for blend.
    fn draw_antialiased_polygon<B>(
        &self,
        poly: &[Point<i32>],
        color: Self::Pixel,
        blend: B,
    ) -> Image<Self::Pixel>
    where
        B: Fn(Self::Pixel, Self::Pixel, f32) -> Self::Pixel,
    {
        self.draw_polygon_with(poly, color, |image, start, end, color| {
            image.draw_antialiased_line_segment_mut(
                (start.0 as i32, start.1 as i32),
                (end.0 as i32, end.1 as i32),
                color,
                &blend,
            )
        })
    }

    /// 在复制的新图像上绘制多边形的轮廓线。仅绘制位于图像边界内的多边形的轮廓
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
    fn draw_hallow_polygon(&self, poly: &[Point<f32>], color: Self::Pixel) -> Image<Self::Pixel> {
        let mut out = ImageBuffer::new(self.width(), self.height());
        out.copy_from(self, 0, 0).unwrap();
        out.draw_hallow_polygon_mut(poly, color);
        out
    }

    /// 在复制的新图像上绘制矩形的轮廓。仅绘制在图像边界内的矩形的轮廓。
    ///
    /// Draws the outline of a rectangle on a new copy of an image.
    ///
    /// Draws as much of the boundary of the rectangle as lies inside the image bounds.
    #[must_use = "the function does not modify the original image"]
    fn draw_hollow_rect(&self, rect: Rect, color: Self::Pixel) -> Image<Self::Pixel> {
        let mut out = ImageBuffer::new(self.width(), self.height());
        out.copy_from(self, 0, 0).unwrap();
        out.draw_hollow_rect_mut(rect, color);
        out
    }

    /// 在复制的新图像上绘制实心矩形。仅绘制在图像边界内的矩形。
    ///
    /// Draws a rectangle and its contents on a new copy of an image.
    ///
    /// Draws as much of the rectangle and its contents as lies inside the image bounds.
    #[must_use = "the function does not modify the original image"]
    fn draw_filled_rect(&self, rect: Rect, color: Self::Pixel) -> Image<Self::Pixel> {
        let mut out = ImageBuffer::new(self.width(), self.height());
        out.copy_from(self, 0, 0).unwrap();
        out.draw_filled_rect_mut(rect, color);
        out
    }

    fn draw_hollow_rounded_rect(
        &self,
        rect: Rect,
        radius: i32,
        color: Self::Pixel,
    ) -> Image<Self::Pixel> {
        let mut out = ImageBuffer::new(self.width(), self.height());
        out.copy_from(self, 0, 0).unwrap();
        out.draw_hollow_rounded_rect_mut(rect, radius, color);
        out
    }

    fn draw_filled_rounded_rect(
        &self,
        rect: Rect,
        radius: i32,
        color: Self::Pixel,
    ) -> Image<Self::Pixel> {
        let mut out = ImageBuffer::new(self.width(), self.height());
        out.copy_from(self, 0, 0).unwrap();
        out.draw_filled_rounded_rect_mut(rect, radius, color);
        out
    }
}

impl<I: GenericImage> Draw for I {}

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
    fn test_draw_cubic_bezier_curve() {
        let img = RgbaImage::new(100, 100);
        let color = Rgba([255, 0, 0, 255]);
        let img2 =
            img.draw_cubic_bezier_curve((50.0, 50.0), (0.0, 0.0), (2.0, 2.0), (3.0, 3.0), color);
        save(&img2, "draw_cubic_bezier_curve");

        assert_eq!(img.get_pixel(50, 30), &color);
        assert_eq!(img.get_pixel(70, 50), &color);
        assert_eq!(img.get_pixel(50, 70), &color);
        assert_eq!(img.get_pixel(30, 50), &color);
    }

    #[test]
    fn test_draw_hollow_ellipse() {
        let img = RgbaImage::new(100, 100);
        let color = Rgba([0, 255, 255, 255]);
        let img2 = img.draw_hollow_ellipse((50, 50), 30, 20, color);
        save(&img2, "draw_hollow_ellipse");

        assert_eq!(img.get_pixel(50, 30), &color);
        assert_eq!(img.get_pixel(80, 50), &color);
        assert_eq!(img.get_pixel(50, 70), &color);
        assert_eq!(img.get_pixel(20, 50), &color);
    }

    #[test]
    fn test_draw_filled_ellipse() {
        let img = RgbaImage::new(100, 100);
        let color = Rgba([255, 128, 0, 255]);
        let img2 = img.draw_filled_ellipse((50, 50), 30, 20, color);
        save(&img2, "draw_filled_ellipse");

        assert_eq!(img.get_pixel(50, 50), &color);
        assert_eq!(img.get_pixel(50, 30), &color);
        assert_eq!(img.get_pixel(80, 50), &color);
        assert_eq!(img.get_pixel(50, 70), &color);
    }

    #[test]
    fn test_draw_hollow_circle() {
        let img = RgbaImage::new(100, 100);
        let color = Rgba([255, 0, 0, 255]);
        let img2 = img.draw_hollow_circle((50, 50), 20, color);
        save(&img2, "draw_hollow_circle");

        assert_eq!(img.get_pixel(50, 30), &color);
        assert_eq!(img.get_pixel(70, 50), &color);
        assert_eq!(img.get_pixel(50, 70), &color);
        assert_eq!(img.get_pixel(30, 50), &color);
    }

    #[test]
    fn test_draw_filled_circle() {
        let img = RgbaImage::new(100, 100);
        let color = Rgba([255, 0, 0, 255]);
        let img2 = img.draw_filled_circle((50, 50), 20, color);
        save(&img2, "draw_filled_circle");

        assert_eq!(img.get_pixel(50, 50), &color);
        assert_eq!(img.get_pixel(50, 30), &color);
        assert_eq!(img.get_pixel(70, 50), &color);
        assert_eq!(img.get_pixel(50, 70), &color);
    }

    #[test]
    fn test_draw_cross() {
        let img = RgbaImage::new(100, 100);
        let color = Rgba([128, 128, 128, 255]);
        let img2 = img.draw_cross(color, 40, 50);
        save(&img2, "draw_cross");

        assert_eq!(img.get_pixel(40, 50), &color);
        assert_eq!(img.get_pixel(39, 50), &color);
        assert_eq!(img.get_pixel(41, 50), &color);
        assert_eq!(img.get_pixel(40, 49), &color);
        assert_eq!(img.get_pixel(40, 51), &color);
    }

    #[test]
    fn test_draw_line_segment() {
        let img = RgbaImage::new(100, 100);
        let color = Rgba([255, 0, 0, 255]);
        let img2 = img.draw_line_segment((10.0, 10.0), (90.0, 90.0), color);
        save(&img2, "draw_line_segment");

        assert_eq!(img.get_pixel(10, 10), &color);
        assert_eq!(img.get_pixel(50, 50), &color);
        assert_eq!(img.get_pixel(90, 90), &color);
    }

    // #[test]
    // fn test_draw_antialiased_line_segment() {
    //     let img = RgbaImage::new(100, 100);
    //     let color = Rgba([255, 0, 0, 255]);
    //     let img2=img.draw_antialiased_line_segment((10.0, 10.0), (90.0, 90.0), color);
    //     save(&img2, "draw_antialiased_line_segment");
    //     assert_eq!(img.get_pixel(10, 10), &color);
    //     assert_eq!(img.get_pixel(50, 50), &color);
    //     assert_eq!(img.get_pixel(90, 90), &color);
    // }

    #[test]
    fn test_draw_polygon() {
        let img = RgbaImage::new(100, 100);
        let color = Rgba([255, 255, 0, 255]);
        let points = [Point::new(73, 5), Point::new(20, 65), Point::new(35, 86)];
        let img2 = img.draw_polygon(&points, color);
        save(&img2, "draw_polygon");

        assert_eq!(img.get_pixel(30, 30), &color);
        assert_eq!(img.get_pixel(49, 30), &color);
        assert_eq!(img.get_pixel(30, 49), &color);
        assert_eq!(img.get_pixel(49, 49), &color);
    }

    // #[test]
    // fn test_draw_antialiased_polygon() {
    //     let img = RgbaImage::new(100, 100);
    //     let color = Rgba([255, 255, 0, 255]);
    //     let points = [Point::new(73, 5), Point::new(20, 65), Point::new(35, 86)];
    //     let img2=img.draw_antialiased_polygon(&points, color);
    //     save(&img2, "draw_antialiased_polygon");
    //     assert_eq!(img.get_pixel(30, 30), &color);
    //     assert_eq!(img.get_pixel(49, 30), &color);
    //     assert_eq!(img.get_pixel(30, 49), &color);
    //     assert_eq!(img.get_pixel(49, 49), &color);
    // }

    #[test]
    fn test_draw_hallow_polygon() {
        let img = RgbaImage::new(100, 100);
        let color = Rgba([255, 0, 0, 255]);
        let points = [
            Point::new(73.0, 5.0),
            Point::new(20.0, 65.0),
            Point::new(35.0, 86.0),
        ];
        let img2 = img.draw_hallow_polygon(&points, color);
        save(&img2, "draw_hallow_polygon");

        assert_eq!(img.get_pixel(20, 20), &color);
        assert_eq!(img.get_pixel(59, 20), &color);
        assert_eq!(img.get_pixel(20, 59), &color);
        assert_eq!(img.get_pixel(59, 59), &color);
    }

    #[test]
    fn test_draw_hollow_rect() {
        let img = RgbaImage::new(100, 100);
        let color = Rgba([255, 0, 0, 255]);
        let rect = Rect::at(20, 20).of_size(40, 40);
        let img2 = img.draw_hollow_rect(rect, color);
        save(&img2, "draw_hollow_rect");

        assert_eq!(img.get_pixel(20, 20), &color);
        assert_eq!(img.get_pixel(59, 20), &color);
        assert_eq!(img.get_pixel(20, 59), &color);
        assert_eq!(img.get_pixel(59, 59), &color);
    }

    #[test]
    fn test_draw_filled_rect() {
        let img = RgbaImage::new(100, 100);
        let color = Rgba([255, 255, 0, 255]);
        let rect = Rect::at(30, 30).of_size(20, 20);
        let img2 = img.draw_filled_rect(rect, color);
        save(&img2, "draw_filled_rect");

        assert_eq!(img.get_pixel(30, 30), &color);
        assert_eq!(img.get_pixel(49, 30), &color);
        assert_eq!(img.get_pixel(30, 49), &color);
        assert_eq!(img.get_pixel(49, 49), &color);
    }

    #[test]
    fn test_draw_hollow_rounded_rect() {
        let img = RgbaImage::new(100, 100);
        let color = Rgba([255, 0, 0, 255]);
        let rect = Rect::at(20, 20).of_size(40, 40);
        let img2 = img.draw_hollow_rounded_rect(rect, 5, color);
        save(&img2, "draw_hollow_rounded_rect");

        assert_eq!(img.get_pixel(20, 20), &color);
        assert_eq!(img.get_pixel(59, 20), &color);
        assert_eq!(img.get_pixel(20, 59), &color);
        assert_eq!(img.get_pixel(59, 59), &color);
    }

    #[test]
    fn test_draw_filled_rounded_rect() {
        let img = RgbaImage::new(100, 100);
        let color = Rgba([255, 255, 0, 255]);
        let rect = Rect::at(30, 30).of_size(20, 20);
        let img2 = img.draw_filled_rounded_rect(rect, 5, color);
        save(&img2, "draw_filled_rounded_rect");

        assert_eq!(img.get_pixel(30, 30), &color);
        assert_eq!(img.get_pixel(49, 30), &color);
        assert_eq!(img.get_pixel(30, 49), &color);
        assert_eq!(img.get_pixel(49, 49), &color);
    }
}
