mod element;
mod position;

use crate::{Result, color::Color, imageproc::drawing::DrawMut};
use ab_glyph::FontArc;
pub use element::Position;
use element::{ContentType, Element};
use image::DynamicImage;
use serde::Deserialize;
use std::path::Path;

#[derive(Clone, Deserialize)]
pub struct Slide {
    elements: Vec<Element>,
    image_num: usize,
    text_num: usize,
    color_num: usize,
}

impl Slide {
    pub fn new() -> Self {
        Self {
            elements: Vec::with_capacity(8),
            image_num: 0,
            text_num: 0,
            color_num: 0,
        }
    }
    /// 获取元素数量, 返回(图片数量, 文字数量, 色块数量)
    pub fn element_size(&self) -> (usize, usize, usize) {
        (self.image_num, self.text_num, self.color_num)
    }
    pub fn image_num(&self) -> usize {
        self.image_num
    }
    pub fn text_num(&self) -> usize {
        self.text_num
    }
    pub fn color_num(&self) -> usize {
        self.color_num
    }
    pub fn add_text(mut self, str: &str, max_scale: f32, color: Color, position: Position) -> Self {
        self.elements.push(Element {
            content: ContentType::Text {
                content: str.to_string(),
                max_scale,
                color,
            },
            position,
        });
        self.text_num += 1;
        self
    }
    pub fn add_image(mut self, image_path: impl AsRef<Path>, position: Position) -> Self {
        self.elements.push(Element {
            content: ContentType::Image(image_path.as_ref().to_path_buf()),
            position,
        });
        self.image_num += 1;
        self
    }
    pub fn add_color(mut self, color: Color, position: Position) -> Self {
        self.elements.push(Element {
            content: ContentType::Color(color),
            position,
        });
        self.color_num += 1;
        self
    }
}

impl Slide {
    pub fn render(
        &self,
        width: u32,
        height: u32,
        font: &FontArc,
        split_line_color: Option<Color>,
    ) -> Result<DynamicImage> {
        let mut img = DynamicImage::new_rgba8(width, height);
        for element in &self.elements {
            let rect = element.position.to_rect(width);
            element.content.render(&mut img, rect, font)?;
        }
        // 绘制分割线
        if let Some(color) = split_line_color {
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
