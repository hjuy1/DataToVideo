use crate::{
    Result,
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
use std::path::PathBuf;

#[derive(Clone, Deserialize)]
pub enum ContentType {
    Image(PathBuf),
    Text {
        content: String,
        max_scale: f32,
        color: Color,
    },
    Color(Color),
}

impl ContentType {
    pub fn render(&self, img: &mut DynamicImage, rect: Rect, font: &FontArc) -> Result<()> {
        match self {
            ContentType::Image(path) => {
                let img_element = image::open(path)?.thumbnail(rect.width(), rect.height());
                let (img_w, img_h) = img_element.dimensions();
                img.copy_from(
                    &img_element,
                    rect.left() as u32 + (rect.width() - img_w) / 2,
                    rect.top() as u32 + (rect.height() - img_h) / 2,
                )?;
            }
            ContentType::Text {
                content,
                max_scale,
                color,
            } => {
                img.draw_text_center_mut(Into::into(*color), rect, *max_scale, font, content);
            }
            ContentType::Color(color) => {
                img.draw_filled_rounded_rect_mut(rect, 10, Into::into(*color));
            }
        }
        Ok(())
    }
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

#[derive(Clone, Deserialize)]
pub struct Element {
    pub content: ContentType,
    pub position: Position,
}
