use crate::{
    COLOR_COMBIMATION_1, COLOR_COMBIMATION_2, COLOR_COMBIMATION_3, COLOR_COMBIMATION_4,
    POSITION_COMBIMATION_1, Result,
    color::Color,
    imageproc::{
        drawing::{DrawMut, DrawText},
        rect::Rect,
    },
};
use ab_glyph::FontArc;
use image::{DynamicImage, GenericImage, GenericImageView};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::path::{Path, PathBuf};

#[derive(Clone)]
struct Text {
    content: String,
    max_scale: f32,
    color: Color,
}

#[derive(Clone)]
enum ContentType {
    Image(PathBuf),
    Text(Text),
    Color(Color),
}

#[derive(Clone, Debug, Copy, Deserialize, Serialize)]
pub struct Position {
    left: i32,
    top: i32,
    height: u32,
}

impl Position {
    pub const fn new(left: i32, top: i32, height: u32) -> Self {
        Self { left, top, height }
    }
    pub fn to_rect(&self, width: u32) -> Rect {
        Rect::at(self.left, self.top).of_size(width - self.left as u32 * 2, self.height)
    }
}

#[derive(Clone)]
struct Element {
    content: ContentType,
    position: Position,
}

#[derive(Clone)]
pub struct Slide {
    elements: Vec<Element>,
    split_line_color: Option<Color>,
}

impl Slide {
    pub fn new() -> Self {
        Self {
            elements: Vec::with_capacity(8),
            split_line_color: None,
        }
    }
    pub fn new_default(order: u8) -> Self {
        match order {
            1 => Self::new()
                .set_split_line_color([255, 255, 255].into())
                .add_color(COLOR_COMBIMATION_1.0, POSITION_COMBIMATION_1.1)
                .add_color(COLOR_COMBIMATION_1.1, POSITION_COMBIMATION_1.2),
            2 => Self::new()
                .set_split_line_color([255, 255, 255].into())
                .add_color(COLOR_COMBIMATION_2.0, POSITION_COMBIMATION_1.1)
                .add_color(COLOR_COMBIMATION_2.1, POSITION_COMBIMATION_1.2),
            3 => Self::new()
                .set_split_line_color([255, 255, 255].into())
                .add_color(COLOR_COMBIMATION_3.0, POSITION_COMBIMATION_1.1)
                .add_color(COLOR_COMBIMATION_3.1, POSITION_COMBIMATION_1.2),
            4 => Self::new()
                .set_split_line_color([255, 255, 255].into())
                .add_color(COLOR_COMBIMATION_4.0, POSITION_COMBIMATION_1.1)
                .add_color(COLOR_COMBIMATION_4.1, POSITION_COMBIMATION_1.2),
            _ => Self::new(),
        }
    }
    pub fn add_text(mut self, str: &str, max_scale: f32, color: Color, position: Position) -> Self {
        self.elements.push(Element {
            content: ContentType::Text(Text {
                content: str.to_string(),
                max_scale,
                color,
            }),
            position,
        });
        self
    }
    pub fn add_image(mut self, image_path: impl AsRef<Path>, position: Position) -> Self {
        self.elements.push(Element {
            content: ContentType::Image(image_path.as_ref().to_path_buf()),
            position,
        });
        self
    }
    pub fn add_color(mut self, color: Color, position: Position) -> Self {
        self.elements.push(Element {
            content: ContentType::Color(color),
            position,
        });
        self
    }
    pub fn set_split_line_color(mut self, color: Color) -> Self {
        self.split_line_color = Some(color);
        self
    }
}

impl Slide {
    pub fn render(&self, width: u32, height: u32, font: &FontArc) -> Result<DynamicImage> {
        let mut img = DynamicImage::new_rgba8(width, height);
        for element in &self.elements {
            let rect = element.position.to_rect(width);
            match element.content {
                ContentType::Image(ref path) => {
                    let img_element = image::open(path)?.thumbnail(rect.width(), rect.height());
                    let (img_w, img_h) = img_element.dimensions();
                    img.copy_from(
                        &img_element,
                        rect.left() as u32 + (rect.width() - img_w) / 2,
                        rect.top() as u32 + (rect.height() - img_h) / 2,
                    )?;
                }
                ContentType::Text(Text {
                    ref content,
                    max_scale,
                    color,
                }) => {
                    img.draw_text_center_mut(color.into(), rect, max_scale, font, content);
                }
                ContentType::Color(color) => {
                    img.draw_filled_rounded_rect_mut(rect, 10, color.into());
                }
            }
        }
        // 绘制分割线
        if let Some(color) = self.split_line_color {
            img.draw_line_segment_mut((0.0, 0.0), (0.0, height as f32), color.into());
        }
        Ok(img)
    }
}

impl Default for Slide {
    fn default() -> Self {
        Self::new()
    }
}
