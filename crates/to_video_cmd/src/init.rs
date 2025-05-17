use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use to_video::{
    BLACK, COLOR_COMBIMATION_1, POSITION_COMBIMATION_1, Result,
    color::Color,
    slide::{Position, Slide},
    video::{VideoConfig, VideoConfigBuilder},
};

#[derive(Deserialize, Serialize)]
pub struct Info {
    pub slide_default: u8,
    pub operations: Vec<Operation>,
    pub config: Option<VideoConfigBuilder>,
    pub data: PathBuf,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Operation {
    Image(Position),
    Text(f32, Color, Position),
    Color(Color, Position),
}

pub fn init() -> Result<()> {
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
            slide_default: 0,
            operations: vec![
                Operation::Image(POSITION_COMBIMATION_1.0),
                Operation::Text(100.0, BLACK, POSITION_COMBIMATION_1.1),
                Operation::Text(100.0, BLACK, POSITION_COMBIMATION_1.2),
                Operation::Color(COLOR_COMBIMATION_1.0, POSITION_COMBIMATION_1.1),
                Operation::Color(COLOR_COMBIMATION_1.1, POSITION_COMBIMATION_1.2),
            ],
            config: Some(VideoConfig::builder()),
            data: data_example,
        };
        let example = serde_json::to_string_pretty(&info).unwrap();
        fs::write(info_example, example)?;
    }

    if !example_dir.join("MiSans-Demibold.ttf").exists() {
        return Err("default font file: ./example/MiSans-Demibold.ttf not found".into());
    }
    Ok(())
}

pub fn parse() -> Result<(Option<VideoConfigBuilder>, Vec<Slide>)> {
    let mut args = std::env::args().skip(1);
    let info: Info = if let Some(info) = args.next() {
        serde_json::from_slice(&fs::read(info)?)?
    } else if std::env::current_dir()?.join("info.json").exists() {
        serde_json::from_slice(&fs::read(std::env::current_dir()?.join("info.json"))?)?
    } else {
        return Err("no info.json file".into());
    };
    let Info {
        slide_default,
        operations,
        config,
        data,
    } = info;
    let data: Vec<Vec<String>> = serde_json::from_slice(&fs::read(data)?)?;
    let slides = data
        .iter()
        .map(|d| {
            let mut slide = Slide::new_default(slide_default);
            let mut d = d.iter();
            for o in operations.iter() {
                match o {
                    Operation::Text(scale, color, pos) => {
                        slide = slide.add_text(d.next().unwrap(), *scale, *color, *pos)
                    }
                    Operation::Image(pos) => slide = slide.add_image(d.next().unwrap(), *pos),
                    Operation::Color(pos, color) => slide = slide.add_color(*pos, *color),
                }
            }
            slide
        })
        .collect::<Vec<_>>();
    Ok((config, slides))
}
