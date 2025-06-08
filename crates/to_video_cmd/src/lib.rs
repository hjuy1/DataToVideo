use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use to_video::{
    BLACK, COLOR_3_1, POSITION_4_2, Result,
    slide::{Operation, Position},
    video::{Video, VideoBuilder, VideoConfig, VideoConfigBuilder},
};

#[derive(Deserialize, Serialize)]
pub struct Info {
    pub operations: Vec<Operation>,
    pub config: VideoConfigBuilder,
    pub data: PathBuf,
}

pub fn example() -> Result<()> {
    let example_dir = PathBuf::from("example");
    if !example_dir.exists() {
        fs::create_dir(&example_dir)?;
    }
    let data_example = example_dir.join("data.json");
    let pic_1 = format!("{}", example_dir.join("1.png").display());
    let pic_2 = format!("{}", example_dir.join("2.png").display());
    let pic_3 = format!("{}", example_dir.join("3.png").display());
    if !data_example.exists() {
        let data = [
            [&pic_1, "my wife", "text_1_1", "text_1_2"],
            [&pic_2, "my wife too", "text_2_1", "text_2_2"],
            [&pic_3, "my wife three", "text_3_1", "text_3_2"],
        ]
        .repeat(10);
        let example = serde_json::to_string_pretty(&data).unwrap();
        fs::write(&data_example, example)?;
    }

    let info_example = example_dir.join("info.json");
    if !info_example.exists() {
        let info = Info {
            operations: vec![
                Operation::Image {
                    pos: POSITION_4_2.0,
                    z_index: 0,
                },
                Operation::Color {
                    color: COLOR_3_1.0,
                    pos: POSITION_4_2.1,
                    z_index: 1,
                },
                Operation::Color {
                    color: COLOR_3_1.1,
                    pos: POSITION_4_2.2,
                    z_index: 2,
                },
                Operation::Color {
                    color: COLOR_3_1.2,
                    pos: Position::new(1, 900, 180),
                    z_index: 3,
                },
                Operation::Text {
                    scale: 120.0,
                    color: BLACK,
                    pos: POSITION_4_2.1,
                    z_index: 4,
                },
                Operation::Text {
                    scale: 120.0,
                    color: BLACK,
                    pos: POSITION_4_2.2,
                    z_index: 5,
                },
                Operation::Text {
                    scale: 120.0,
                    color: BLACK,
                    pos: POSITION_4_2.3,
                    z_index: 6,
                },
            ],
            config: VideoConfig::builder().fps(30).step(15),
            data: data_example,
        };
        let example = serde_json::to_string_pretty(&info).unwrap();
        fs::write(info_example, example)?;
    }
    Ok(())
}

pub fn parse() -> Result<VideoBuilder> {
    if let Some(s) = std::env::args().skip(1).next() {
        if s == "--example" || s == "-e" {
            example()?;
            std::process::exit(0);
        }
    }
    let file = loop {
        match FileDialog::new()
            .add_filter("json", &["json"])
            .set_title("Select info json file")
            .set_directory("/")
            .pick_file()
        {
            Some(p) => break p,
            None => {
                println!("No file selected");
                continue;
            }
        }
    };
    let info = serde_json::from_slice(&fs::read(&file)?)
        .map_err(|e| format!("Invalid info file:  {e}"))?;
    let Info {
        mut operations,
        config,
        data,
    } = info;
    let data: Vec<Vec<String>> = serde_json::from_slice(&fs::read(data)?)?;
    let video_builder = Video::builder(&mut operations, data, config.build()?)?;
    Ok(video_builder)
}
