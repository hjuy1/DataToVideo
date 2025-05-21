mod element;

use crate::{Result, color::Color, imageproc::drawing::DrawMut};
use ab_glyph::FontArc;
pub use element::{ContentType, Element, Position};
use image::DynamicImage;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize)]
pub enum OperationType {
    Image(Position),
    Text(f32, Color, Position),
    Color(Color, Position),
}

#[derive(Deserialize, Serialize)]
pub struct Operation {
    pub op: OperationType,
    pub z_index: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Slide(Vec<Element>);

impl Slide {
    pub fn new() -> Self {
        Self(Vec::with_capacity(8))
    }
    pub fn generation(operations: &[Operation], datas: Vec<String>) -> Result<Self> {
        let mut data = datas.into_iter();
        let elements = operations
            .iter()
            .map(|op| match op.op {
                OperationType::Image(position) => Ok(Element {
                    content: ContentType::Image(PathBuf::from(
                        data.next().ok_or(format!("图片数据不足"))?,
                    )),
                    position,
                }),
                OperationType::Text(max_scale, color, position) => Ok(Element {
                    content: ContentType::Text {
                        content: data.next().take().ok_or(format!("文本数据不足"))?,
                        max_scale,
                        color,
                    },
                    position,
                }),
                OperationType::Color(color, position) => Ok(Element {
                    content: ContentType::Color(color),
                    position,
                }),
            })
            .collect::<Result<Vec<Element>>>()?;
        Ok(Self(elements))
    }
    pub fn add_text(&mut self, str: &str, max_scale: f32, color: Color, position: Position) {
        self.0.push(Element {
            content: ContentType::Text {
                content: str.to_string(),
                max_scale,
                color,
            },
            position,
        });
    }
    pub fn add_image(&mut self, image_path: impl AsRef<Path>, position: Position) {
        self.0.push(Element {
            content: ContentType::Image(image_path.as_ref().to_path_buf()),
            position,
        });
    }
    pub fn add_color(&mut self, color: Color, position: Position) {
        self.0.push(Element {
            content: ContentType::Color(color),
            position,
        });
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
        for element in &self.0 {
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
