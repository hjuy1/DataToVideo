use crate::{Result, color::Color, slide::Slide};
use ab_glyph::FontArc;
use image::{DynamicImage, GenericImage};
use std::{
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
pub fn generate_endpoint_video(
    pic_name: &Path,
    video_name: &Path,
    video_time: u32,
    back_color: &str,
    screen: (u32, u32),
    fps: u32,
    work_dir: &Path,
) -> Result<()> {
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
#[allow(clippy::too_many_arguments)]
pub fn generate_mid_video(
    pic_name: &Path,
    video_name: &Path,
    image_width: u32,
    screen: (u32, u32),
    swip_pixels_per_sec: u32,
    back_color: &str,
    fps: u32,
    work_dir: &Path,
) -> Result<()> {
    let run_sec = (image_width - screen.0) / swip_pixels_per_sec + 1;
    ffmpeg(
        &[
            "-loglevel",
            "warning",
            "-r",
            "1",
            "-loop",
            "1",
            "-t",
            &run_sec.to_string(),
            "-i",
            &format!("{}", pic_name.display()),
            "-filter_complex",
            &format!(
                "color={}:s={}x{}:r={}[bg];\
                [bg][0]overlay=x=-t*{swip_pixels_per_sec}:shortest=1",
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
pub fn generate_ending_video(
    pic_name: &Path,
    video_name: &Path,
    image_width: u32,
    screen: (u32, u32),
    swip_pixels_per_sec: u32,
    back_color: &str,
    fps: u32,
    ending_sec: u32,
    work_dir: &Path,
) -> Result<()> {
    let run_sec = (image_width - screen.0) / swip_pixels_per_sec;
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
                "color={back_color}:s={}x{}:r={fps}[bg];\
                [bg][0]overlay=x=-t*{swip_pixels_per_sec}:enable='lte(n,{})',scale=1920:1080[slide];\
                [0]crop=1920:1080:2880:0[right_end]; \
                [bg][right_end]overlay,scale=1920:1080[freeze];\
                [freeze]loop=120:size=1:start=0[freeze_2sec]; \
                [slide][freeze_2sec]concat=n=2:v=1:a=0,format=yuv420p[v]",
                screen.0,
                screen.1,
                run_sec * fps
            ),
            "-map",
            "[v]",
            "-preset",
            "fast",
            "-t",
            &(run_sec + ending_sec).to_string(),
            "-y",
            &format!("{}", video_name.display()),
        ],
        work_dir,
    )?;
    Ok(())
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
pub fn generate_ending2_video(
    pic_name: &Path,
    video_name: &Path,
    image_width: u32,
    screen: (u32, u32),
    swip_pixels_per_sec: u32,
    back_color: &str,
    fps: u32,
    ending_sec: u32,
    work_dir: &Path,
) -> Result<()> {
    let run_sec = (image_width - screen.0) / swip_pixels_per_sec;
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
                "color={back_color}:s={}x{}:r={fps}[bg];\
                [0]crop=1920:1080:2880:0,trim=start_frame=0:end_frame=1,setpts=N/60/TB[right_end]; \
                [bg][right_end]overlay,scale=1920:1080[freeze];\
                [freeze]loop=120:size=1:start=0",
                screen.0, screen.1,
            ),
            "-preset",
            "fast",
            "-t",
            &(run_sec + ending_sec).to_string(),
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
pub fn combain(mut results: Vec<PathBuf>, work_dir: &Path, save_path: &Path) -> Result<()> {
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
pub fn ffmpeg(args: &[&str], work_dir: &Path) -> Result<()> {
    let command = Command::new("ffmpeg")
        .current_dir(work_dir)
        .args(args)
        .output()?;
    if !command.status.success() {
        let put = format!("{}", String::from_utf8(command.stderr)?);
        return Err(format!("FFmpeg command failed: {}", put).into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_ending() {
        let start = Instant::now();
        generate_ending_video(
            Path::new(r"00.png"),
            Path::new(r"01.mp4"),
            10 * 480,
            (1920, 1080),
            160,
            "gray",
            60,
            4,
            Path::new(r"D:\program\DataToVideo\make_video\work"),
        )
        .unwrap();
        println!("{} ms", start.elapsed().as_millis());
    }
}
