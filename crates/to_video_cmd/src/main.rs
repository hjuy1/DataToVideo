mod init;

use std::time::Instant;
use to_video::{
    Result,
    video::{Video, VideoConfig},
};

fn main() -> Result<()> {
    init::init()?;
    let t = Instant::now();

    let (config, data) = init::parse()?;

    let video = Video::builder(
        data,
        config.unwrap_or_else(|| VideoConfig::builder()).build()?,
    )
    .build()?;
    video.run()?;
    let cost = t.elapsed().as_millis();
    println!("cost {} s {} ms", cost / 1000, cost % 1000);
    Ok(())
}
