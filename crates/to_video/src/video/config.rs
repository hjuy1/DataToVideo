use crate::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub struct VideoConfig {
    pub(super) screen: (u32, u32),
    pub(super) fps: u32,
    pub(super) work_dir: PathBuf,
    pub(super) back_color: String,
    pub(super) cover_sec: u32,
    pub(super) ending_sec: u32,
    pub(super) swip_pixels_per_sec: u32,
    pub(super) width_slides: u32,
    pub(super) save_path: PathBuf,
    pub(super) step: u32,
    pub(super) overlap: u32,
    pub(super) font: PathBuf,
}

impl VideoConfig {
    pub fn builder() -> VideoConfigBuilder {
        VideoConfigBuilder::new()
    }
}

#[derive(Serialize, Deserialize)]
pub struct VideoConfigBuilder {
    pub screen: (u32, u32),
    pub fps: u32,
    pub work_dir: Option<PathBuf>,
    pub back_color: String,
    pub cover_sec: u32,
    pub ending_sec: u32,
    pub swip_pixels_per_sec: u32,
    pub width_slides: u32,
    pub save_path: Option<PathBuf>,
    pub step: u32,
    pub font: Option<PathBuf>,
}

impl VideoConfigBuilder {
    pub fn new() -> Self {
        Self {
            screen: (1920, 1080),
            fps: 60,
            work_dir: None,
            back_color: "white".to_string(),
            cover_sec: 4,
            ending_sec: 4,
            swip_pixels_per_sec: 160,
            width_slides: 480,
            save_path: None,
            step: 20,
            font: None,
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

        let work_dir = match self.work_dir {
            Some(work_dir) => {
                if !work_dir.exists() {
                    return Err("work_dir is set but does not exist".into());
                } else {
                    work_dir
                }
            }
            None => {
                let default_work_dir = std::env::current_dir()?.join("work");
                println!("Using default work_dir: {}", default_work_dir.display());
                if !default_work_dir.exists() {
                    std::fs::create_dir_all(&default_work_dir)?;
                }
                default_work_dir
            }
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
            screen: self.screen,
            fps: self.fps,
            work_dir: work_dir.clone(),
            back_color: self.back_color,
            cover_sec: self.cover_sec,
            ending_sec: self.ending_sec,
            swip_pixels_per_sec: self.swip_pixels_per_sec,
            width_slides: self.width_slides,
            save_path: self.save_path.unwrap_or_else(|| {
                let default_path = work_dir.join("output");
                println!("Using default save_path: {}", default_path.display());
                default_path
            }),
            step: self.step,
            overlap,
            font,
        })
    }
}

impl VideoConfigBuilder {
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

    pub fn cover_sec(mut self, cover_sec: u32) -> Self {
        self.cover_sec = cover_sec;
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
}

impl Default for VideoConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
