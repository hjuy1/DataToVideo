use super::config::MotionType;
use crate::{Result, color::Color, slide::Slide};
use ab_glyph::FontArc;
use image::{DynamicImage, GenericImage};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::Command,
};

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
pub fn combain_slides(
    slides: &[Slide],
    font: &FontArc,
    width_slides: u32,
    screen: (u32, u32),
    split_line_color: Option<Color>,
) -> Result<DynamicImage> {
    if slides.is_empty() {
        return Err("Empty slides".into());
    }

    let len = u32::try_from(slides.len())?;
    let mut target = DynamicImage::new_rgba8(len * width_slides, screen.1);

    // 将每张图片绘制到目标图像中
    for (i, item) in slides.iter().enumerate() {
        let img = item.render((width_slides, screen.1), font, split_line_color)?;
        target.copy_from(&img, u32::try_from(i)? * width_slides, 0)?;
    }
    Ok(target)
}

impl MotionType {
    pub fn get_motion_range(&self, ranges: &str) -> String {
        match self {
            MotionType::Linear => ranges.to_string(),
            MotionType::EaseIn => format!("sin({ranges}*3.14/2-3.14/2)+1"),
            MotionType::EaseOut => format!("sin({ranges}*3.14/2)"),
            MotionType::EaseInOut => format!("(sin({ranges}*3.14-3.14/2)+1)/2"),
        }
    }
}

pub fn generate_cover_video(
    encoder: &str,
    input_images: Vec<String>,
    cover_sec: f32,
    back_color: &str,
    screen: (u32, u32),
    width_slides: u32,
    fps: u32,
    motion_type: MotionType,
    work_dir: &Path,
    video_name: &Path,
) -> Result<()> {
    let num_images = input_images.len();
    let fade_duration = cover_sec / num_images as f32;

    // 添加输入图片
    let mut inputs = String::new();
    let mut filters = String::new();

    // 创建基础画布
    filters.push_str(&format!(
        "color={back_color}:s={}x{}:r={fps}[base];",
        screen.0, screen.1
    ));

    // 处理每张图片
    for (i, img) in input_images.iter().enumerate() {
        inputs.push_str(&format!("-i {} ", img));
        let start_time = i as f32 * fade_duration;

        // 图片输入和格式转换
        filters.push_str(&format!(
            "[{}:v]format=yuva420p,setpts=PTS-STARTPTS+{}/TB[v{}];",
            i, start_time, i
        ));

        // 计算水平位置（x坐标）
        let x_pos = i as u32 * width_slides;

        // 计算垂直运动（y坐标）
        let ranges = motion_type.get_motion_range(&format!(
            "clip(t-{},0,{fade_duration})/{fade_duration}",
            i as f32 * fade_duration,
        ));
        let y_expr = format!("{}-({ranges})*{}", screen.1, screen.1);

        // 叠加到画布
        let input = if i == 0 {
            "base".to_string()
        } else {
            format!("tmp{}", i - 1)
        };
        filters.push_str(&format!(
            "[{}][v{}]overlay=x={}:y='{}'[tmp{}];",
            input, i, x_pos, y_expr, i
        ));
    }

    let ffmpeg_args = format!(
        "{inputs} -filter_complex {} -map [tmp{}] \
        -c:v {encoder} -crf 18 -preset fast -movflags +faststart -t {cover_sec} {}",
        filters.trim_end_matches(';'),
        num_images - 1,
        video_name.display()
    );

    ffmpeg(work_dir, ffmpeg_args.split_ascii_whitespace())
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
#[allow(clippy::too_many_arguments)]
pub fn generate_mid_video(
    encoder: &str,
    pic_name: &Path,
    video_name: &Path,
    screen: (u32, u32),
    swip_pixels_per_sec: u32,
    back_color: &str,
    fps: u32,
    move_sec: u32,
    static_sec: u32,
    work_dir: &Path,
) -> Result<()> {
    let (width, height) = screen;
    let ffmpeg_args = format!(
        "-r 1 -loop 1 -i {} \
        -filter_complex \
        color={back_color}:s={width}x{height}:r={fps}[bg];\
        [bg][0]overlay=x='-{swip_pixels_per_sec}*clip(t,0,{move_sec})' \
        -c:v {encoder} -crf 18 -preset fast -movflags +faststart -t {} {}",
        pic_name.display(),
        move_sec + static_sec,
        video_name.display()
    );
    ffmpeg(work_dir, ffmpeg_args.split_ascii_whitespace())
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
pub fn combain(results: &mut Vec<PathBuf>, work_dir: &Path, save_path: &Path) -> Result<()> {
    // 构建ffmpeg concat协议要求的输入文件列表字符串
    // 格式示例：
    //file /path/to/file1
    //file /path/to/file2
    let result_str: String = results
        .iter()
        .filter_map(|s| {
            s.to_str()
                .and_then(|ss| ss.ends_with("mp4").then(|| format!("file {}\n", ss)))
        })
        .collect();

    // 将文件列表写入临时文本文件
    let list_file = "list.txt";
    std::fs::write(work_dir.join(list_file), result_str)?;
    results.push(PathBuf::from(list_file));

    // 调用ffmpeg执行合并操作
    let ffmpeg_args = format!(
        "-f concat -i {list_file} -c copy -y {}",
        save_path.display()
    );
    ffmpeg(work_dir, ffmpeg_args.split_ascii_whitespace())?;

    println!("{} successed", save_path.display());
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
pub fn ffmpeg<I, S>(work_dir: &Path, args: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let command = Command::new("ffmpeg")
        .current_dir(work_dir)
        .arg("-loglevel")
        .arg("warning")
        .arg("-y")
        .args(args)
        .output()?;
    if !command.status.success() {
        let put = format!("{}", String::from_utf8(command.stderr)?);
        return Err(format!("FFmpeg command failed: {}", put).into());
    }
    Ok(())
}
