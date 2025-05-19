use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use to_video::{
    BLACK, COLOR_2_1, POSITION_3_1, Result,
    slide::{Operation, OperationType},
    video::{Video, VideoBuilder, VideoConfig, VideoConfigBuilder},
};

#[derive(Deserialize, Serialize)]
pub struct Info {
    pub operations: Vec<Operation>,
    pub config: VideoConfigBuilder,
    pub data: PathBuf,
}

pub fn example() -> Result<()> {
    let example_dir = PathBuf::from("./example");
    if !example_dir.exists() {
        fs::create_dir(&example_dir)?;
    }
    let data_example = example_dir.join("Data_example.json");
    let pic_1 = format!("{}", example_dir.join("1.png").display());
    let pic_2 = format!("{}", example_dir.join("2.png").display());
    let pic_3 = format!("{}", example_dir.join("3.png").display());
    if !data_example.exists() {
        let data = [
            [&pic_1, "my wife", "text_1_2"],
            [&pic_2, "my wife too", "text_2_2"],
            [&pic_3, "my wife three", "text_3_2"],
        ]
        .repeat(10);
        let example = serde_json::to_string_pretty(&data).unwrap();
        fs::write(&data_example, example)?;
    }

    let info_example = example_dir.join("Info_example.json");
    if !info_example.exists() {
        let info = Info {
            operations: vec![
                Operation {
                    op: OperationType::Image(POSITION_3_1.0),
                    z_index: 2,
                },
                Operation {
                    op: OperationType::Text(100.0, BLACK, POSITION_3_1.1),
                    z_index: 2,
                },
                Operation {
                    op: OperationType::Text(100.0, BLACK, POSITION_3_1.2),
                    z_index: 2,
                },
                Operation {
                    op: OperationType::Color(COLOR_2_1.0, POSITION_3_1.1),
                    z_index: 2,
                },
                Operation {
                    op: OperationType::Color(COLOR_2_1.1, POSITION_3_1.2),
                    z_index: 2,
                },
            ],
            config: VideoConfig::builder(),
            data: data_example,
        };
        let example = serde_json::to_string_pretty(&info).unwrap();
        fs::write(info_example, example)?;
    }
    Ok(())
}

pub fn parse() -> Result<VideoBuilder> {
    let mut args = std::env::args().skip(1);
    let info: Info = if let Some(info) = args.next() {
        if info.as_str().trim() == "example" {
            example()?;
            std::process::exit(0);
        } else {
            serde_json::from_slice(&fs::read(info)?)?
        }
    } else {
        let default_info = std::env::current_dir()?.join("info.json");
        if default_info.exists() {
            serde_json::from_slice(&fs::read(default_info)?)?
        } else {
            return Err(format!(
                "Not info_file arg and default_info_file: {} does not exist",
                default_info.display()
            )
            .into());
        }
    };
    let Info {
        mut operations,
        config,
        data,
    } = info;
    let data: Vec<Vec<String>> = serde_json::from_slice(&fs::read(data)?)?;
    let video_builder = Video::builder(&mut operations, data, config.build()?)?;
    Ok(video_builder)
}
