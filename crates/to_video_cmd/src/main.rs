mod init;

use std::{path::Path, time::Instant};
use to_video::{Result, video::Video};

fn main() -> Result<()> {
    let t = Instant::now();

    let (config, data) = init::parse()?;

    let video = Video::builder(data, config.build()?).build()?;

    let handle_progress = move |file: &Path, generate_len: usize, total: usize| {
        println!("{} / {} : {}  success", generate_len, total, file.display());
    };

    video.run(handle_progress)?;
    let cost = t.elapsed().as_millis();
    println!("cost {} s {} ms", cost / 1000, cost % 1000);
    Ok(())
}
