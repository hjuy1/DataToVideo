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
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize)]
pub enum Element {
    Image {
        path: PathBuf,
        pos: Position,
    },
    Text {
        content: String,
        max_scale: f32,
        color: Color,
        pos: Position,
    },
    Color {
        color: Color,
        pos: Position,
    },
}

impl Element {
    pub fn render(&self, img: &mut DynamicImage, width: u32, font: &FontArc) -> Result<()> {
        match self {
            Element::Image { path, pos } => {
                let rect = pos.to_rect(width);
                let img_element = image::open(path)?.thumbnail(rect.width(), rect.height());
                let (img_w, img_h) = img_element.dimensions();
                img.copy_from(
                    &img_element,
                    rect.left() as u32 + (rect.width() - img_w) / 2,
                    rect.top() as u32 + (rect.height() - img_h) / 2,
                )?;
            }
            Element::Text {
                content,
                max_scale,
                color,
                pos,
            } => {
                let rect = pos.to_rect(width);
                img.draw_text_center_mut(Into::into(*color), rect, *max_scale, font, content);
            }
            Element::Color { color, pos } => {
                let rect = pos.to_rect(width);
                img.draw_filled_rounded_rect_mut(rect, 10, Into::into(*color));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Copy, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Deserialize, Serialize)]
pub enum Operation {
    Image {
        pos: Position,
        z_index: u8,
    },
    Text {
        scale: f32,
        color: Color,
        pos: Position,
        z_index: u8,
    },
    Color {
        color: Color,
        pos: Position,
        z_index: u8,
    },
}

impl Operation {
    fn z_index(&self) -> u8 {
        match self {
            Operation::Image { z_index, .. } => *z_index,
            Operation::Text { z_index, .. } => *z_index,
            Operation::Color { z_index, .. } => *z_index,
        }
    }
}

impl PartialEq for Operation {
    fn eq(&self, other: &Self) -> bool {
        self.z_index() == other.z_index()
    }
}

impl Eq for Operation {}

impl PartialOrd for Operation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.z_index().partial_cmp(&other.z_index())
    }
}

impl Ord for Operation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.z_index().cmp(&other.z_index())
    }
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
            .map(|op| match op {
                Operation::Image { pos, .. } => Ok(Element::Image {
                    path: PathBuf::from(data.next().ok_or(format!("图片数据不足"))?),
                    pos: *pos,
                }),
                Operation::Text {
                    scale, color, pos, ..
                } => Ok(Element::Text {
                    content: data.next().take().ok_or(format!("文本数据不足"))?,
                    max_scale: *scale,
                    color: *color,
                    pos: *pos,
                }),
                Operation::Color { color, pos, .. } => Ok(Element::Color {
                    color: *color,
                    pos: *pos,
                }),
            })
            .collect::<Result<Vec<Element>>>()?;
        Ok(Self(elements))
    }
    pub fn add_text(&mut self, str: &str, max_scale: f32, color: Color, pos: Position) {
        self.0.push(Element::Text {
            content: str.to_string(),
            max_scale,
            color,
            pos,
        });
    }
    pub fn add_image(&mut self, image_path: impl AsRef<Path>, pos: Position) {
        self.0.push(Element::Image {
            path: image_path.as_ref().to_path_buf(),
            pos,
        });
    }
    pub fn add_color(&mut self, color: Color, pos: Position) {
        self.0.push(Element::Color { color, pos });
    }
}

impl Slide {
    pub fn render(
        &self,
        size: (u32, u32),
        font: &FontArc,
        split_line_color: Option<Color>,
    ) -> Result<DynamicImage> {
        let (width, height) = size;
        let mut img = DynamicImage::new_rgba8(width, height);
        for element in &self.0 {
            element.render(&mut img, width, font)?;
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
