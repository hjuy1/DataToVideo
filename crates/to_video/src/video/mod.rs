pub mod config;
pub mod ffmpeg;
pub mod slide;

use crate::Result;
use ab_glyph::FontArc;
use ffmpeg::{combain, combain_slides, generate_endpoint_video, generate_mid_video};
use image::GenericImageView;
use slide::{Operation, Slide};
use std::{fs, path::Path};

pub use config::{VideoConfig, VideoConfigBuilder};

pub struct Video {
    chunks: Vec<Vec<Slide>>,
    config: VideoConfig,
}

impl Video {
    pub fn builder(
        operations: &mut [Operation],
        datas: Vec<Vec<String>>,
        config: VideoConfig,
    ) -> Result<VideoBuilder> {
        operations.sort();
        Ok(VideoBuilder {
            slides: datas
                .into_iter()
                .map(|data| Slide::generation(operations, data))
                .collect::<Result<Vec<Slide>>>()?,
            config: config,
        })
    }

    pub fn chunks(&self) -> &Vec<Vec<Slide>> {
        &self.chunks
    }

    pub fn config(&self) -> &VideoConfig {
        &self.config
    }
}

impl Video {
    /// 组合所有图像块并生成最终视频。
    ///
    /// # Parameters
    /// - `handle_progress`: 处理进度的回调函数，参数为处理文件名、已处理数量和总数量。
    pub fn run<F>(self, handle_progress: F) -> Result<()>
    where
        F: Fn(&Path, usize, usize) -> std::result::Result<(), String>,
    {
        let chunks_len = self.chunks.len();
        let mut results = Vec::with_capacity(chunks_len + 2);

        let font_buf = fs::read(&self.config.font)?;
        let font = FontArc::try_from_vec(font_buf).map_err(|_| "Invalid font file")?;
        let VideoConfig {
            screen,
            fps,
            ref work_dir,
            ref back_color,
            cover_sec,
            ending_sec,
            swip_pixels_per_sec,
            width_slides,
            ref save_path,
            split_line_color,
            ..
        } = self.config;

        for (index, slides) in self.chunks.into_iter().enumerate() {
            let slides_len = slides.len();

            let target = combain_slides(&slides, &font, width_slides, screen, split_line_color)?;
            if index == 0 {
                let cover = target.crop_imm(0, 0, screen.0, screen.1);
                let cover_pic_name = Path::new("cover.png");
                // 保存组合后的图像
                cover.save(work_dir.join(cover_pic_name))?;

                let cover_video_name = cover_pic_name.with_extension("mp4");

                generate_endpoint_video(
                    cover_pic_name,
                    &cover_video_name,
                    cover_sec,
                    back_color,
                    screen,
                    fps,
                    work_dir,
                )?;

                handle_progress(&cover_video_name, index + 1, chunks_len + 2)?;
                results.push(cover_video_name);
            }

            // 保存组合后的图像
            let mid_pic_name = format!("{index:0>2}.png");
            let mid_pic_name = Path::new(&mid_pic_name);
            target.save(work_dir.join(mid_pic_name))?;

            let mid_video_name = mid_pic_name.with_extension("mp4");
            let image_width = slides_len as u32 * width_slides;

            generate_mid_video(
                mid_pic_name,
                &mid_video_name,
                image_width,
                screen,
                swip_pixels_per_sec,
                back_color,
                fps,
                work_dir,
            )?;
            handle_progress(&mid_video_name, index + 2, chunks_len + 2)?;
            results.push(mid_video_name);

            if index == chunks_len - 1 {
                let w = target.dimensions().0;
                let ending = target.crop_imm(w - screen.0, 0, screen.0, screen.1);
                let ending_pic_name = Path::new("ending.png");
                // 保存组合后的图像
                ending.save(work_dir.join(ending_pic_name))?;

                let ending_video_name = ending_pic_name.with_extension("mp4");

                generate_endpoint_video(
                    ending_pic_name,
                    &ending_video_name,
                    ending_sec,
                    back_color,
                    screen,
                    fps,
                    work_dir,
                )?;
                handle_progress(&ending_video_name, index + 3, chunks_len + 2)?;
                results.push(ending_video_name);
            }
        }

        combain(results, work_dir, save_path)?;
        Ok(())
    }
}

pub struct VideoBuilder {
    slides: Vec<Slide>,
    config: VideoConfig,
}

impl VideoBuilder {
    #[allow(dead_code)]
    pub fn add_slides(mut self, mut slides: Vec<Slide>) -> Self {
        self.slides.append(&mut slides);
        self
    }

    pub fn len(&self) -> usize {
        self.slides.len()
    }

    pub fn is_empty(&self) -> bool {
        self.slides.is_empty()
    }

    pub fn build(self) -> Result<Video> {
        if self.slides.is_empty() {
            return Err("slides data is empty".into());
        }

        let (step, overlap, len) = (
            self.config.step as usize,
            self.config.overlap as usize,
            self.len(),
        );

        if len < overlap {
            return Err("slides data is shorter than overlap".into());
        }

        let chunks = (0..len - overlap)
            .step_by(step - overlap)
            .map(|i| self.slides[i..(i + step).min(len)].to_vec())
            .collect();
        Ok(Video {
            chunks,
            config: self.config,
        })
    }
}
