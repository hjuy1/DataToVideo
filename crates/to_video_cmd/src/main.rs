mod init;

use std::{path::Path, time::Instant};
use to_video::Result;

fn main() -> Result<()> {
    let t = Instant::now();

    let video_builder = init::parse()?;

    let video = video_builder.build()?;

    let handle_progress = move |file: &Path, generate_len: usize, total: usize| {
        println!("{} / {} : {}  success", generate_len, total, file.display());
        Ok(())
    };

    video.run(handle_progress)?;
    let cost = t.elapsed().as_millis();
    println!("cost {} s {} ms", cost / 1000, cost % 1000);
    std::io::stdin().read_line(&mut String::new())?;
    Ok(())
}
