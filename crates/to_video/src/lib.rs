pub mod color;
pub mod constants;
pub mod imageproc;
pub mod video;

pub use {constants::*, video::slide};
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn test_encoder() -> Result<Vec<String>> {
    let ffmpeg_args = "ffmpeg -hide_banner -encoders | findstr h264";
    let ffm = std::process::Command::new("cmd")
        .arg("/c")
        .args(ffmpeg_args.split_ascii_whitespace())
        .output()
        .expect("failed to execute process");
    let result = String::from_utf8_lossy(&ffm.stdout);
    let encoders = result
        .lines()
        .map(|l| l.split_ascii_whitespace().nth(1).map(String::from))
        .collect::<Option<Vec<_>>>()
        .ok_or("Failed to parse ffmpeg output")?;
    Ok(encoders)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let enc = test_encoder().unwrap();
        dbg!(&enc);
    }
}
