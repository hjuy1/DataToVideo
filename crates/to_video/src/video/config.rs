use crate::{Result, color::Color};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum MotionType {
    Linear,    // 匀速运动
    EaseIn,    // 缓入（变速）
    EaseOut,   // 缓出（变速）
    EaseInOut, // 缓入缓出（变速）
}

pub struct VideoConfig {
    pub(super) encoder: String,
    pub(super) screen: (u32, u32),
    pub(super) fps: u32,
    pub(super) work_dir: PathBuf,
    pub(super) back_color: String,
    pub(super) cover_sec: f32,
    pub(super) motion_type: MotionType,
    pub(super) ending_sec: u32,
    pub(super) swip_pixels_per_sec: u32,
    pub(super) width_slides: u32,
    pub(super) save_path: PathBuf,
    pub(super) step: u32,
    pub(super) overlap: u32,
    pub(super) font: PathBuf,
    pub(super) split_line_color: Option<Color>,
    pub(super) clean_temp: bool,
}

impl VideoConfig {
    pub fn builder() -> VideoConfigBuilder {
        VideoConfigBuilder::new()
    }

    pub fn save_path(&self) -> &PathBuf {
        &self.save_path
    }
}

#[derive(Serialize, Deserialize)]
pub struct VideoConfigBuilder {
    pub encoder: String,
    pub screen: (u32, u32),
    pub fps: u32,
    pub work_dir: Option<PathBuf>,
    pub back_color: String,
    pub cover_sec: f32,
    pub motion_type: MotionType,
    pub ending_sec: u32,
    pub swip_pixels_per_sec: u32,
    pub width_slides: u32,
    pub save_path: Option<PathBuf>,
    pub step: u32,
    pub font: Option<PathBuf>,
    pub split_line_color: Option<Color>,
    pub clean_temp: bool,
}

impl VideoConfigBuilder {
    pub fn new() -> Self {
        Self {
            encoder: "libx264".into(),
            screen: (1920, 1080),
            fps: 60,
            work_dir: None,
            back_color: "white".to_string(),
            cover_sec: 10.0,
            motion_type: MotionType::EaseInOut,
            ending_sec: 4,
            swip_pixels_per_sec: 160,
            width_slides: 480,
            save_path: None,
            step: 20,
            font: None,
            split_line_color: Some(Color([255, 255, 255])),
            clean_temp: true,
        }
    }

    pub fn build(self) -> Result<VideoConfig> {
        if self.screen.0 % self.width_slides != 0 {
            return Err(format!(
                "width_screen % width_slides != 0; {} % {} != 0",
                self.screen.0, self.width_slides
            )
            .into());
        }

        let overlap = self.screen.0 / self.width_slides;

        if self.step <= overlap {
            return Err("step is shorter than overlap".into());
        }

        let work_dir = if let Some(work_dir) = self.work_dir {
            if !work_dir.exists() {
                return Err("work_dir is set but does not exist".into());
            }
            work_dir
        } else {
            let default_work_dir = std::env::current_dir()?.join("work");
            println!("Using default work_dir: {}", default_work_dir.display());
            if !default_work_dir.exists() {
                std::fs::create_dir_all(&default_work_dir)?;
            }
            default_work_dir
        };

        let font = match self.font {
            Some(font) => {
                if font.exists() {
                    font
                } else {
                    return Err("Font is set but does not exist".into());
                }
            }
            None => return Err("Font not set".into()),
        };

        Ok(VideoConfig {
            encoder: self.encoder,
            screen: self.screen,
            fps: self.fps,
            work_dir: work_dir.clone(),
            back_color: self.back_color,
            cover_sec: self.cover_sec,
            motion_type: self.motion_type,
            ending_sec: self.ending_sec,
            swip_pixels_per_sec: self.swip_pixels_per_sec,
            width_slides: self.width_slides,
            save_path: self.save_path.unwrap_or_else(|| {
                let default_path = work_dir.join("output.mp4");
                println!("Using default save_path: {}", default_path.display());
                default_path
            }),
            step: self.step,
            overlap,
            font,
            split_line_color: self.split_line_color,
            clean_temp: self.clean_temp,
        })
    }
}

impl VideoConfigBuilder {
    pub fn encoder(mut self, encoder: &str) -> Self {
        self.encoder = encoder.to_string();
        self
    }

    pub fn screen(mut self, screen: (u32, u32)) -> Self {
        self.screen = screen;
        self
    }

    pub fn fps(mut self, fps: u32) -> Self {
        self.fps = fps;
        self
    }

    pub fn work_dir(mut self, work_dir: PathBuf) -> Self {
        self.work_dir = Some(work_dir);
        self
    }

    pub fn back_color<S: Into<String>>(mut self, back_color: S) -> Self {
        self.back_color = back_color.into();
        self
    }

    pub fn cover_sec(mut self, cover_sec: f32) -> Self {
        self.cover_sec = cover_sec;
        self
    }

    pub fn motion_type(mut self, motion_type: MotionType) -> Self {
        self.motion_type = motion_type;
        self
    }

    pub fn ending_sec(mut self, ending_sec: u32) -> Self {
        self.ending_sec = ending_sec;
        self
    }

    pub fn swip_pixels_per_sec(mut self, swip_pixels_per_sec: u32) -> Self {
        self.swip_pixels_per_sec = swip_pixels_per_sec;
        self
    }

    pub fn width_slides(mut self, width_slides: u32) -> Self {
        self.width_slides = width_slides;
        self
    }

    pub fn save_path(mut self, save_path: PathBuf) -> Self {
        self.save_path = Some(save_path);
        self
    }

    pub fn step(mut self, step: u32) -> Self {
        self.step = step;
        self
    }

    pub fn font(mut self, font: PathBuf) -> Self {
        self.font = Some(font);
        self
    }

    pub fn split_line_color(mut self, split_line_color: Option<Color>) -> Self {
        self.split_line_color = split_line_color;
        self
    }

    pub fn clean_temp(mut self, clean_temp: bool) -> Self {
        self.clean_temp = clean_temp;
        self
    }
}

impl Default for VideoConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
