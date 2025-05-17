pub mod config;

use crate::{Result, slide::Slide};
use ab_glyph::FontArc;
use image::{DynamicImage, GenericImage, GenericImageView};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

pub use config::{VideoConfig, VideoConfigBuilder};

pub struct Video {
    chunks: Vec<Vec<Slide>>,
    config: VideoConfig,
}

impl Video {
    pub fn builder(slides: Vec<Slide>, config: VideoConfig) -> VideoBuilder {
        VideoBuilder { slides, config }
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
    /// - `save_name`: 最终视频文件名。
    ///
    /// # Errors
    /// - 如果图像处理或保存过程中发生错误，则返回 `Err`。
    /// - 如果 `FFmpeg` 命令执行失败，则返回 `Err`。
    ///
    pub fn run(self) -> Result<()> {
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
            overlap,
            ..
        } = self.config;

        for (index, slides) in self.chunks.into_iter().enumerate() {
            let slides_len = slides.len();

            let target = combain_slides(&slides, &font, width_slides, screen)?;
            if index == 0 {
                let cover = target.crop_imm(0, 0, screen.0, screen.1);
                let cover_pic_name = Path::new("cover.png");
                // 保存组合后的图像
                cover.save(work_dir.join(cover_pic_name))?;

                let cover_video_name = cover_pic_name.with_extension("mp4");

                print!(
                    "{}/{chunks_len}: {} start",
                    index + 1,
                    cover_video_name.display()
                );
                generate_endpoint_video(
                    cover_pic_name,
                    &cover_video_name,
                    cover_sec,
                    back_color,
                    screen,
                    fps,
                    work_dir,
                )?;
                println!(
                    "\r{}/{chunks_len}: {} successed",
                    index + 1,
                    cover_video_name.display()
                );
                results.push(cover_video_name);
            }

            // 保存组合后的图像
            let mid_pic_name = format!("{index:0>2}.png");
            let mid_pic_name = Path::new(&mid_pic_name);
            target.save(work_dir.join(mid_pic_name))?;

            let mid_video_name = mid_pic_name.with_extension("mp4");
            let image_width = (slides_len as u32 - overlap) * width_slides;

            print!(
                "{}/{chunks_len}: {} start",
                index + 1,
                mid_video_name.display()
            );
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
            println!(
                "\r{}/{chunks_len}: {} successed",
                index + 1,
                mid_video_name.display()
            );
            results.push(mid_video_name);

            if index == chunks_len - 1 {
                let w = target.dimensions().0;
                let ending = target.crop_imm(w - screen.0, 0, screen.0, screen.1);
                let ending_pic_name = Path::new("ending.png");
                // 保存组合后的图像
                ending.save(work_dir.join(ending_pic_name))?;

                let ending_video_name = ending_pic_name.with_extension("mp4");

                print!(
                    "{}/{chunks_len}: {} start",
                    index + 1,
                    ending_video_name.display()
                );
                generate_endpoint_video(
                    ending_pic_name,
                    &ending_video_name,
                    ending_sec,
                    back_color,
                    screen,
                    fps,
                    work_dir,
                )?;
                println!(
                    "\r{}/{chunks_len}: {} successed",
                    index + 1,
                    ending_video_name.display()
                );
                results.push(ending_video_name);
            }
        }

        combain(results, work_dir, save_path)?;
        Ok(())
    }
}

/// 将多个图像块组合成一个完整的图像。
///
/// # Parameters
/// - `slides`: 要组合的图像块切片。
///
/// # Results
/// 如果成功，则返回组合后的 `DynamicImage`；如果失败，则返回 `Err`。
///
/// # Errors
/// - 如果 `slides` 为空，则返回 `Err`。
/// - 如果图像处理过程中发生错误，则返回 `Err`。
///
fn combain_slides(
    slides: &[Slide],
    font: &FontArc,
    width_slides: u32,
    screen: (u32, u32),
) -> Result<DynamicImage> {
    if slides.is_empty() {
        return Err("Empty slides".into());
    }

    let len = u32::try_from(slides.len())?;
    let mut target = DynamicImage::new_rgba8(len * width_slides, screen.1);

    // 将每张图片绘制到目标图像中
    for (i, item) in slides.iter().enumerate() {
        let img = item.render(width_slides, screen.1, font)?;
        target.copy_from(&img, u32::try_from(i)? * width_slides, 0)?;
        target.copy_from(&img, u32::try_from(i)? * width_slides, 0)?;
    }
    Ok(target)
}

/// 生成视频封面或结尾视频。
///
/// # Parameters
/// - `pic_name`: 素材图片名称。
/// - `video_name`: 生成视频名称。
/// - `video_time`: 视频时长（秒）。
///
/// # Errors
/// - 如果 `FFmpeg` 命令执行失败，则返回 `Err`。
///
fn generate_endpoint_video(
    pic_name: &Path,
    video_name: &Path,
    video_time: u32,
    back_color: &str,
    screen: (u32, u32),
    fps: u32,
    work_dir: &Path,
) -> Result<()> {
    use std::io::Write;
    std::io::stdout().flush()?;
    ffmpeg(
        &[
            "-loglevel",
            "warning",
            "-r",
            "1",
            "-loop",
            "1",
            "-i",
            &format!("{}", pic_name.display()),
            "-filter_complex",
            &format!(
                "color={}:s={}x{}:r={}[bg];[bg][0]overlay=shortest=1",
                back_color, screen.0, screen.1, fps
            ),
            "-preset",
            "fast",
            "-t",
            &video_time.to_string(),
            "-y",
            &format!("{}", video_name.display()),
        ],
        work_dir,
    )?;
    Ok(())
}

/// 生成中间部分的视频。
///
/// # Parameters
/// - `len`: 素材图片中 `slides` 数量。
/// - `pic_name`: 素材图片名称。
/// - `video_name`: 生成视频名称。
///
/// # Errors
/// - 如果 `FFmpeg` 命令执行失败，则返回 `Err`。
///
fn generate_mid_video(
    pic_name: &Path,
    video_name: &Path,
    image_width: u32,
    screen: (u32, u32),
    swip_pixels_per_sec: u32,
    back_color: &str,
    fps: u32,
    work_dir: &Path,
) -> Result<()> {
    use std::io::Write;
    std::io::stdout().flush()?;
    let run_seconds = image_width / swip_pixels_per_sec + 1;

    ffmpeg(
        &[
            "-loglevel",
            "warning",
            "-r",
            "1",
            "-loop",
            "1",
            "-t",
            &run_seconds.to_string(),
            "-i",
            &format!("{}", pic_name.display()),
            "-filter_complex",
            &format!(
                "color={}:s={}x{}:r={}[bg];[bg][0]overlay=x=-t*{swip_pixels_per_sec}:shortest=1",
                back_color, screen.0, screen.1, fps
            ),
            "-preset",
            "fast",
            "-y",
            &format!("{}", video_name.display()),
        ],
        work_dir,
    )?;
    Ok(())
}

/// 合并多个文件为单个输出文件，使用ffmpeg的concat协议
///
/// # Parameters
/// - `results`: 需要合并的源文件路径列表
/// - `save_name`: 合并后的输出文件路径
///
/// # Errors
/// - 如果文件写入或 `FFmpeg` 命令执行失败，则返回 `Err`。
///
fn combain(mut results: Vec<PathBuf>, work_dir: &Path, save_path: &Path) -> Result<()> {
    use std::fmt::Write;
    // 构建ffmpeg concat协议要求的输入文件列表字符串
    // 格式示例：file '/path/to/file1'\nfile '/path/to/file2'
    let result_str =
        results
            .iter()
            .fold(String::with_capacity(results.len() * 20), |mut init, s| {
                let _ = writeln!(init, "file {}", s.to_string_lossy());
                init
            });

    // 将文件列表写入临时文本文件
    let list_file = work_dir.join("list.txt");
    std::fs::write(&list_file, result_str)?;

    // 调用ffmpeg执行合并操作参数说明：
    // -f concat 指定concat分离器
    // -i 输入文件列表
    // -c copy 使用流拷贝模式（不重新编码）
    // -y 覆盖输出文件
    ffmpeg(
        &[
            "-loglevel",
            "warning",
            "-f",
            "concat",
            "-i",
            &list_file.to_string_lossy(),
            "-c",
            "copy",
            "-y",
            &save_path.to_string_lossy(),
        ],
        work_dir,
    )?;

    println!("{} successed", save_path.display());

    // 清理临时文件（包含两个步骤）：
    // 1. 删除文件列表
    // 2. 删除所有中间结果文件及其对应的png文件
    let _ = std::fs::remove_file(&list_file);
    for result in results.iter_mut() {
        let _ = std::fs::remove_file(work_dir.join(&result));
        result.set_extension("png");
        let _ = std::fs::remove_file(work_dir.join(result));
    }
    println!("cleanup successed");
    Ok(())
}

/// 执行带有指定参数的FFmpeg命令
///
/// # Parameters
/// - `config: &VideoConfig` - 包含工作路径配置的结构体实例引用
/// - `args` - 传递给ffmpeg命令行工具的字符串参数切片
///
/// # Results
/// - 成功时返回Ok(())，失败时返回包含上下文信息的Err
///
/// # Errors
/// - 无法执行ffmpeg命令时返回IO错误
/// - ffmpeg进程返回非零状态码时打印stderr到控制台并返回Other类型错误
///
fn ffmpeg(args: &[&str], work_dir: &Path) -> Result<()> {
    let command = Command::new("ffmpeg")
        .current_dir(work_dir)
        .args(args)
        .output()?;
    if !command.status.success() {
        println!("{}", String::from_utf8(command.stderr)?);
        return Err("FFmpeg command failed".into());
    }
    Ok(())
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
